[CmdletBinding()]
param(
    [Parameter(Mandatory = $true)]
    [string]$ScenarioPath,
    [string]$OutputDir = ".tmp/oxxlplay-w006",
    [switch]$EmitBundle = $true
)

Set-StrictMode -Version Latest
$ErrorActionPreference = "Stop"

function Resolve-FullPath {
    param(
        [Parameter(Mandatory = $true)]
        [string]$BasePath,
        [Parameter(Mandatory = $true)]
        [string]$CandidatePath
    )

    if ([System.IO.Path]::IsPathRooted($CandidatePath)) {
        return [System.IO.Path]::GetFullPath($CandidatePath)
    }

    return [System.IO.Path]::GetFullPath((Join-Path $BasePath $CandidatePath))
}

function Convert-ToRepoRelativePath {
    param(
        [Parameter(Mandatory = $true)]
        [string]$PathValue,
        [Parameter(Mandatory = $true)]
        [string]$RepoRoot
    )

    $fullPath = [System.IO.Path]::GetFullPath($PathValue)
    $normalizedRoot = [System.IO.Path]::GetFullPath($RepoRoot).TrimEnd('\')

    if ($fullPath.StartsWith($normalizedRoot, [System.StringComparison]::OrdinalIgnoreCase)) {
        return $fullPath.Substring($normalizedRoot.Length).TrimStart('\').Replace('\', '/')
    }

    return $fullPath.Replace('\', '/')
}

function Write-JsonFile {
    param(
        [Parameter(Mandatory = $true)]
        [string]$Path,
        [Parameter(Mandatory = $true)]
        $Value
    )

    $parent = Split-Path -Parent $Path
    if (-not [string]::IsNullOrWhiteSpace($parent)) {
        New-Item -ItemType Directory -Path $parent -Force | Out-Null
    }

    $json = $Value | ConvertTo-Json -Depth 20
    Set-Content -Path $Path -Value $json -Encoding utf8
}

function Release-ComObject {
    param($ComObject)

    if ($null -ne $ComObject) {
        [void][System.Runtime.InteropServices.Marshal]::ReleaseComObject($ComObject)
    }
}

function Get-ExcelChannel {
    $configurationPaths = @(
        "HKLM:\SOFTWARE\Microsoft\Office\ClickToRun\Configuration",
        "HKLM:\SOFTWARE\WOW6432Node\Microsoft\Office\ClickToRun\Configuration"
    )

    foreach ($configurationPath in $configurationPaths) {
        try {
            $configuration = Get-ItemProperty -Path $configurationPath -ErrorAction Stop
            $channelSource = @(
                $configuration.UpdateChannel,
                $configuration.CDNBaseUrl,
                $configuration.AudienceData
            ) | Where-Object { -not [string]::IsNullOrWhiteSpace($_) } | Select-Object -First 1

            if ($null -eq $channelSource) {
                continue
            }

            $normalized = $channelSource.ToString().ToLowerInvariant()

            if ($normalized -match "current") {
                return "current"
            }

            if ($normalized -match "monthlyenterprise") {
                return "monthly_enterprise"
            }

            if ($normalized -match "semiannual") {
                return "semi_annual"
            }

            if ($normalized -match "beta") {
                return "beta"
            }

            return $normalized
        }
        catch {
        }
    }

    return "unresolved_local_install"
}

function Get-HostOs {
    try {
        return (Get-CimInstance -ClassName Win32_OperatingSystem -ErrorAction Stop).Caption
    }
    catch {
        return "Windows"
    }
}

function Get-HostArchitecture {
    if ([System.Environment]::Is64BitOperatingSystem) {
        return "x64"
    }

    return "x86"
}

function Ensure-BootstrapWorkbook {
    param(
        [Parameter(Mandatory = $true)]
        $Excel,
        [Parameter(Mandatory = $true)]
        $Scenario,
        [Parameter(Mandatory = $true)]
        [string]$WorkbookPath
    )

    if (Test-Path -LiteralPath $WorkbookPath) {
        return
    }

    if ($Scenario.scenario_id -ne "xlplay_capture_values_formulae_001") {
        throw "Workbook `$WorkbookPath` is missing and no bootstrap recipe exists for scenario `$($Scenario.scenario_id)`."
    }

    New-Item -ItemType Directory -Path (Split-Path -Parent $WorkbookPath) -Force | Out-Null

    $workbook = $null
    $worksheet = $null

    try {
        $workbook = $Excel.Workbooks.Add()
        $worksheet = $workbook.Worksheets.Item(1)
        $worksheet.Name = "Sheet1"
        $worksheet.Range("B1").Value2 = 10
        $worksheet.Range("B2").Value2 = 20
        $worksheet.Range("B3").Value2 = 12
        $worksheet.Range("A1").Formula = "=SUM(B1:B3)"
        $workbook.SaveAs($WorkbookPath, 51)
    }
    finally {
        if ($null -ne $workbook) {
            $workbook.Close($false)
        }

        Release-ComObject -ComObject $worksheet
        Release-ComObject -ComObject $workbook
    }
}

function Resolve-WorkbookPath {
    param(
        [Parameter(Mandatory = $true)]
        $Scenario,
        [Parameter(Mandatory = $true)]
        [string]$ScenarioPath,
        [Parameter(Mandatory = $true)]
        [string]$RepoRoot
    )

    if ([string]::IsNullOrWhiteSpace($Scenario.workbook_ref)) {
        throw "Scenario workbook_ref must not be blank."
    }

    if ($Scenario.workbook_ref.StartsWith(".\") -or $Scenario.workbook_ref.StartsWith("./")) {
        $scenarioDir = Split-Path -Parent $ScenarioPath
        return Resolve-FullPath -BasePath $scenarioDir -CandidatePath $Scenario.workbook_ref
    }

    return Resolve-FullPath -BasePath $RepoRoot -CandidatePath $Scenario.workbook_ref
}

function Resolve-SurfaceRange {
    param(
        [Parameter(Mandatory = $true)]
        $Workbook,
        [Parameter(Mandatory = $true)]
        [string]$Locator
    )

    $parts = $Locator -split "!", 2
    if ($parts.Count -ne 2) {
        throw "Unsupported locator `$Locator`. Expected `Sheet!Cell`."
    }

    $worksheet = $Workbook.Worksheets.Item($parts[0])
    try {
        return $worksheet.Range($parts[1])
    }
    finally {
        Release-ComObject -ComObject $worksheet
    }
}

function Get-WorkbookMediaType {
    param(
        [Parameter(Mandatory = $true)]
        [string]$WorkbookPath
    )

    switch ([System.IO.Path]::GetExtension($WorkbookPath).ToLowerInvariant()) {
        ".xml" { return "application/xml" }
        ".xlsx" { return "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet" }
        default { return "application/octet-stream" }
    }
}

function Convert-CellValueToString {
    param($Value)

    if ($null -eq $Value) {
        return $null
    }

    if ($Value -is [System.Array]) {
        return ($Value | ConvertTo-Json -Depth 20 -Compress)
    }

    if ($Value -is [bool]) {
        return $Value.ToString().ToLowerInvariant()
    }

    if ($Value -is [datetime]) {
        return $Value.ToUniversalTime().ToString("o")
    }

    return [string]$Value
}

function Convert-ExcelColorToHex {
    param($ColorValue)

    if ($null -eq $ColorValue) {
        return $null
    }

    try {
        $color = [int]$ColorValue
    }
    catch {
        return $null
    }

    if ($color -le 0) {
        return $null
    }

    $red = $color -band 0xFF
    $green = ($color -shr 8) -band 0xFF
    $blue = ($color -shr 16) -band 0xFF

    return "#{0:X2}{1:X2}{2:X2}" -f $red, $green, $blue
}

function Get-DynamicPropertyValue {
    param(
        [Parameter(Mandatory = $true)]
        $Object,
        [Parameter(Mandatory = $true)]
        [string]$PropertyName
    )

    if ($Object -is [System.Collections.IDictionary]) {
        if ($Object.Contains($PropertyName)) {
            return $Object[$PropertyName]
        }

        return $null
    }

    $property = $Object.PSObject.Properties[$PropertyName]
    if ($null -ne $property) {
        return $property.Value
    }

    return $null
}

function Get-DynamicPropertyBox {
    param(
        [Parameter(Mandatory = $true)]
        $Object,
        [Parameter(Mandatory = $true)]
        [string]$PropertyName
    )

    if ($Object -is [System.Collections.IDictionary]) {
        if ($Object.Contains($PropertyName)) {
            return [pscustomobject]@{
                value = $Object[$PropertyName]
            }
        }

        return $null
    }

    $property = $Object.PSObject.Properties[$PropertyName]
    if ($null -ne $property) {
        return [pscustomobject]@{
            value = $property.Value
        }
    }

    return $null
}

function Get-ObservedSurfaceValueText {
    param(
        [Parameter(Mandatory = $true)]
        $ObservedSurface
    )

    $valueRepr = Get-DynamicPropertyValue -Object $ObservedSurface -PropertyName "value_repr"
    if ($null -ne $valueRepr -and -not [string]::IsNullOrWhiteSpace([string]$valueRepr)) {
        return [string]$valueRepr
    }

    $valueJson = Get-DynamicPropertyValue -Object $ObservedSurface -PropertyName "value_json"
    if ($null -ne $valueJson) {
        return ($valueJson | ConvertTo-Json -Depth 20 -Compress)
    }

    return $null
}

function Get-RangeStyleName {
    param(
        [Parameter(Mandatory = $true)]
        $Range
    )

    try {
        $style = $Range.Style
        try {
            return [string]$style.Name
        }
        catch {
            return [string]$style
        }
    }
    catch {
        return $null
    }
}

function Get-RangeConditionalFormattingRules {
    param(
        [Parameter(Mandatory = $true)]
        $Range
    )

    $rules = @()
    $count = 0
    try {
        $count = [int]$Range.FormatConditions.Count
    }
    catch {
        return @()
    }

    if ($count -le 0) {
        return @()
    }

    foreach ($index in 1..$count) {
        $condition = $null
        try {
            $condition = $Range.FormatConditions.Item($index)
            $appliesTo = $null
            try {
                $appliesTo = [string]$condition.AppliesTo.Address($false, $false)
            }
            catch {
            }

            $rules += [ordered]@{
                index = $index
                applies_to = $appliesTo
                type = [string]$condition.Type
                operator = [string]$condition.Operator
                formula1 = [string]$condition.Formula1
                formula2 = [string]$condition.Formula2
                font_color = Convert-ExcelColorToHex $condition.Font.Color
                fill_color = Convert-ExcelColorToHex $condition.Interior.Color
            }
        }
        finally {
            Release-ComObject -ComObject $condition
        }
    }

    return ,@($rules)
}

function Get-RangeEffectiveStyleSnapshot {
    param(
        [Parameter(Mandatory = $true)]
        $Range
    )

    $displayFormat = $null
    try {
        $displayFormat = $Range.DisplayFormat
        return [ordered]@{
            number_format_code = [string]$displayFormat.NumberFormat
            font_color = Convert-ExcelColorToHex $displayFormat.Font.Color
            fill_color = Convert-ExcelColorToHex $displayFormat.Interior.Color
            effective_display_text = [string]$Range.Text
            applied_rule_indexes = @()
            source_projection = "excel_display_format_snapshot"
        }
    }
    catch {
        return [ordered]@{
            number_format_code = [string]$Range.NumberFormat
            font_color = Convert-ExcelColorToHex $Range.Font.Color
            fill_color = Convert-ExcelColorToHex $Range.Interior.Color
            effective_display_text = [string]$Range.Text
            applied_rule_indexes = @()
            source_projection = "excel_range_snapshot"
        }
    }
    finally {
        Release-ComObject -ComObject $displayFormat
    }
}

function Get-XmlAttributeValue {
    param(
        [Parameter(Mandatory = $true)]
        [System.Xml.XmlNode]$Node,
        [Parameter(Mandatory = $true)]
        [string]$LocalName
    )

    if ($null -eq $Node.Attributes) {
        return $null
    }

    foreach ($attribute in $Node.Attributes) {
        if ($attribute.LocalName -eq $LocalName) {
            return $attribute.Value
        }
    }

    return $null
}

function Get-XmlElementChildren {
    param(
        [Parameter(Mandatory = $true)]
        [System.Xml.XmlNode]$Node,
        [Parameter(Mandatory = $true)]
        [string]$LocalName
    )

    return @(
        $Node.ChildNodes |
        Where-Object {
            $_.NodeType -eq [System.Xml.XmlNodeType]::Element -and $_.LocalName -eq $LocalName
        }
    )
}

function Convert-A1ReferenceToCoordinates {
    param(
        [Parameter(Mandatory = $true)]
        [string]$CellRef
    )

    $letters = ""
    $digits = ""
    foreach ($character in $CellRef.ToCharArray()) {
        if ([char]::IsLetter($character)) {
            $letters += [char]::ToUpperInvariant($character)
        }
        elseif ([char]::IsDigit($character)) {
            $digits += $character
        }
    }

    if ([string]::IsNullOrWhiteSpace($letters) -or [string]::IsNullOrWhiteSpace($digits)) {
        throw "Invalid A1 reference `$CellRef`."
    }

    $column = 0
    foreach ($letter in $letters.ToCharArray()) {
        $column = ($column * 26) + ([int][char]$letter - [int][char]'A' + 1)
    }

    return [ordered]@{
        column = $column
        row = [int]$digits
    }
}

function Test-SpreadsheetMlRangeContainsReference {
    param(
        [Parameter(Mandatory = $true)]
        [string]$Range,
        [Parameter(Mandatory = $true)]
        [string]$TargetRef
    )

    $target = Convert-A1ReferenceToCoordinates -CellRef $TargetRef
    foreach ($segment in $Range.Split(",")) {
        $trimmed = $segment.Trim()
        if ([string]::IsNullOrWhiteSpace($trimmed)) {
            continue
        }

        if ($trimmed.Contains(":")) {
            $parts = $trimmed.Split(":", 2)
            $start = Convert-A1ReferenceToCoordinates -CellRef $parts[0]
            $end = Convert-A1ReferenceToCoordinates -CellRef $parts[1]
            if (
                $target.column -ge $start.column -and
                $target.column -le $end.column -and
                $target.row -ge $start.row -and
                $target.row -le $end.row
            ) {
                return $true
            }
        }
        elseif ($trimmed.Equals($TargetRef, [System.StringComparison]::OrdinalIgnoreCase)) {
            return $true
        }
    }

    return $false
}

function Resolve-SpreadsheetMlStyleInfo {
    param(
        [Parameter(Mandatory = $true)]
        [hashtable]$Styles,
        [Parameter(Mandatory = $true)]
        [string]$StyleId,
        [hashtable]$Visited = $null
    )

    if (-not $Styles.ContainsKey($StyleId)) {
        return $null
    }

    if ($null -eq $Visited) {
        $Visited = @{}
    }

    if ($Visited.ContainsKey($StyleId)) {
        throw "SpreadsheetML style inheritance cycle detected at style `$StyleId`."
    }

    $Visited[$StyleId] = $true
    $styleInfo = $Styles[$StyleId]
    $resolved = [ordered]@{
        style_id = $styleInfo.style_id
        parent_style_id = $styleInfo.parent_style_id
        number_format_code = $styleInfo.number_format_code
        font_color = $styleInfo.font_color
        fill_color = $styleInfo.fill_color
    }

    if (-not [string]::IsNullOrWhiteSpace($styleInfo.parent_style_id)) {
        $parentStyle = Resolve-SpreadsheetMlStyleInfo -Styles $Styles -StyleId ([string]$styleInfo.parent_style_id) -Visited $Visited
        if ($null -ne $parentStyle) {
            foreach ($propertyName in @("number_format_code", "font_color", "fill_color")) {
                if ([string]::IsNullOrWhiteSpace([string]$resolved[$propertyName])) {
                    $resolved[$propertyName] = $parentStyle[$propertyName]
                }
            }
        }
    }

    $Visited.Remove($StyleId) | Out-Null
    return $resolved
}

function Get-SpreadsheetMlContext {
    param(
        [Parameter(Mandatory = $true)]
        [string]$WorkbookPath,
        [Parameter(Mandatory = $true)]
        [string]$Locator
    )

    if ([System.IO.Path]::GetExtension($WorkbookPath).ToLowerInvariant() -ne ".xml") {
        return $null
    }

    [xml]$document = Get-Content -Raw -LiteralPath $WorkbookPath
    if ($null -eq $document.DocumentElement -or $document.DocumentElement.LocalName -ne "Workbook") {
        return $null
    }

    $locatorParts = $Locator -split "!", 2
    if ($locatorParts.Count -ne 2) {
        return $null
    }

    $worksheetName = $locatorParts[0]
    $cellRef = $locatorParts[1]
    $worksheet = $document.SelectNodes("//*[local-name()='Worksheet']") |
        Where-Object { (Get-XmlAttributeValue -Node $_ -LocalName "Name") -eq $worksheetName } |
        Select-Object -First 1

    if ($null -eq $worksheet) {
        return $null
    }

    $table = Get-XmlElementChildren -Node $worksheet -LocalName "Table" | Select-Object -First 1
    if ($null -eq $table) {
        return $null
    }

    $target = Convert-A1ReferenceToCoordinates -CellRef $cellRef
    $currentRow = 0
    $cellNode = $null
    foreach ($rowNode in (Get-XmlElementChildren -Node $table -LocalName "Row")) {
        $rowIndex = Get-XmlAttributeValue -Node $rowNode -LocalName "Index"
        if (-not [string]::IsNullOrWhiteSpace($rowIndex)) {
            $currentRow = [int]$rowIndex
        }
        else {
            $currentRow += 1
        }

        if ($currentRow -ne $target.row) {
            continue
        }

        $currentColumn = 0
        foreach ($candidateCell in (Get-XmlElementChildren -Node $rowNode -LocalName "Cell")) {
            $columnIndex = Get-XmlAttributeValue -Node $candidateCell -LocalName "Index"
            if (-not [string]::IsNullOrWhiteSpace($columnIndex)) {
                $currentColumn = [int]$columnIndex
            }
            else {
                $currentColumn += 1
            }

            if ($currentColumn -eq $target.column) {
                $cellNode = $candidateCell
                break
            }
        }

        if ($null -ne $cellNode) {
            break
        }
    }

    if ($null -eq $cellNode) {
        return $null
    }

    $styles = @{}
    foreach ($styleNode in $document.SelectNodes("//*[local-name()='Style']")) {
        $styleId = Get-XmlAttributeValue -Node $styleNode -LocalName "ID"
        if ([string]::IsNullOrWhiteSpace($styleId)) {
            continue
        }

        $numberFormatNode = Get-XmlElementChildren -Node $styleNode -LocalName "NumberFormat" | Select-Object -First 1
        $fontNode = Get-XmlElementChildren -Node $styleNode -LocalName "Font" | Select-Object -First 1
        $interiorNode = Get-XmlElementChildren -Node $styleNode -LocalName "Interior" | Select-Object -First 1

        $styles[$styleId] = [ordered]@{
            style_id = $styleId
            parent_style_id = Get-XmlAttributeValue -Node $styleNode -LocalName "Parent"
            number_format_code = if ($null -ne $numberFormatNode) { Get-XmlAttributeValue -Node $numberFormatNode -LocalName "Format" } else { $null }
            font_color = if ($null -ne $fontNode) { Get-XmlAttributeValue -Node $fontNode -LocalName "Color" } else { $null }
            fill_color = if ($null -ne $interiorNode) { Get-XmlAttributeValue -Node $interiorNode -LocalName "Color" } else { $null }
        }
    }

    $styleId = Get-XmlAttributeValue -Node $cellNode -LocalName "StyleID"
    $styleInfo = $null
    if (-not [string]::IsNullOrWhiteSpace($styleId) -and $styles.ContainsKey($styleId)) {
        $styleInfo = Resolve-SpreadsheetMlStyleInfo -Styles $styles -StyleId $styleId
    }

    $conditionalRules = @()
    foreach ($conditionalNode in (Get-XmlElementChildren -Node $worksheet -LocalName "ConditionalFormatting")) {
        $rangeValue = Get-XmlAttributeValue -Node $conditionalNode -LocalName "Range"
        if ([string]::IsNullOrWhiteSpace($rangeValue) -or -not (Test-SpreadsheetMlRangeContainsReference -Range $rangeValue -TargetRef $cellRef)) {
            continue
        }

        $conditionNode = $conditionalNode.SelectSingleNode(".//*[local-name()='Condition']")
        $ruleFontNode = $conditionalNode.SelectSingleNode(".//*[local-name()='Font']")
        $ruleInteriorNode = $conditionalNode.SelectSingleNode(".//*[local-name()='Interior']")

        $conditionalRules += [ordered]@{
            range = $rangeValue
            formula = if ($null -ne $conditionNode) { Get-XmlAttributeValue -Node $conditionNode -LocalName "Formula" } else { $null }
            value1 = if ($null -ne $conditionNode) { Get-XmlAttributeValue -Node $conditionNode -LocalName "Value1" } else { $null }
            value2 = if ($null -ne $conditionNode) { Get-XmlAttributeValue -Node $conditionNode -LocalName "Value2" } else { $null }
            operator = if ($null -ne $conditionNode) { Get-XmlAttributeValue -Node $conditionNode -LocalName "Operator" } else { $null }
            rule_kind = if ($null -ne $conditionNode) { Get-XmlAttributeValue -Node $conditionNode -LocalName "Type" } else { $null }
            font_color = if ($null -ne $ruleFontNode) { Get-XmlAttributeValue -Node $ruleFontNode -LocalName "Color" } else { $null }
            fill_color = if ($null -ne $ruleInteriorNode) { Get-XmlAttributeValue -Node $ruleInteriorNode -LocalName "Color" } else { $null }
        }
    }

    $date1904Node = $document.SelectSingleNode("//*[local-name()='Date1904']")
    $date1904 = $null
    if ($null -ne $date1904Node) {
        $date1904 = if ([string]::IsNullOrWhiteSpace($date1904Node.InnerText)) { $true } else { $date1904Node.InnerText -eq "1" }
    }

    return [ordered]@{
        workbook_kind = "spreadsheetml-2003-import"
        worksheet_name = $worksheetName
        cell_ref = $cellRef
        style_id = $styleId
        number_format_code = if ($null -ne $styleInfo) { $styleInfo.number_format_code } else { $null }
        font_color = if ($null -ne $styleInfo) { $styleInfo.font_color } else { $null }
        fill_color = if ($null -ne $styleInfo) { $styleInfo.fill_color } else { $null }
        conditional_rules = ,@($conditionalRules)
        date1904 = $date1904
    }
}

function Convert-ToConditionalRuleProjection {
    param(
        [Parameter(Mandatory = $true)]
        [object[]]$Rules
    )

    return ,@(
        $Rules | ForEach-Object {
            [ordered]@{
                range = $_.range
                formula = $_.formula
                value1 = $_.value1
                value2 = $_.value2
                operator = $_.operator
                rule_kind = $_.rule_kind
                font_color = $_.font_color
                fill_color = $_.fill_color
            }
        }
    )
}

function Convert-ToBooleanEvaluationResult {
    param($Value)

    if ($null -eq $Value) {
        return $false
    }

    if ($Value -is [bool]) {
        return $Value
    }

    if ($Value -is [int] -or $Value -is [long] -or $Value -is [double] -or $Value -is [decimal]) {
        return [double]$Value -ne 0
    }

    $text = [string]$Value
    if ([string]::IsNullOrWhiteSpace($text)) {
        return $false
    }

    if ($text.Equals("TRUE", [System.StringComparison]::OrdinalIgnoreCase)) {
        return $true
    }

    if ($text.Equals("FALSE", [System.StringComparison]::OrdinalIgnoreCase)) {
        return $false
    }

    return $text -ne "0"
}

function Get-SpreadsheetMlEffectiveStyleProjection {
    param(
        [Parameter(Mandatory = $true)]
        $Worksheet,
        [Parameter(Mandatory = $true)]
        $Range,
        [Parameter(Mandatory = $true)]
        $SpreadsheetMlContext
    )

    $effectiveNumberFormat = $SpreadsheetMlContext.number_format_code
    $effectiveFontColor = $SpreadsheetMlContext.font_color
    $effectiveFillColor = $SpreadsheetMlContext.fill_color
    $appliedRuleIndexes = @()

    for ($index = 0; $index -lt $SpreadsheetMlContext.conditional_rules.Count; $index++) {
        $rule = $SpreadsheetMlContext.conditional_rules[$index]
        if ([string]::IsNullOrWhiteSpace($rule.rule_kind) -or $rule.rule_kind -ne "Expression" -or [string]::IsNullOrWhiteSpace($rule.formula)) {
            continue
        }

        try {
            $evaluation = $Worksheet.Evaluate($rule.formula)
            if (Convert-ToBooleanEvaluationResult $evaluation) {
                $appliedRuleIndexes += ($index + 1)
                if (-not [string]::IsNullOrWhiteSpace($rule.font_color)) {
                    $effectiveFontColor = $rule.font_color
                }
                if (-not [string]::IsNullOrWhiteSpace($rule.fill_color)) {
                    $effectiveFillColor = $rule.fill_color
                }
            }
        }
        catch {
        }
    }

    return [ordered]@{
        number_format_code = $effectiveNumberFormat
        font_color = $effectiveFontColor
        fill_color = $effectiveFillColor
        effective_display_text = [string]$Range.Text
        applied_rule_indexes = @($appliedRuleIndexes)
        source_projection = "spreadsheetml_expression_rules_v1"
    }
}

function New-AutoExpandedSurfaceId {
    param(
        [Parameter(Mandatory = $true)]
        [string]$Locator,
        [Parameter(Mandatory = $true)]
        [string]$SurfaceKind
    )

    $locatorToken = ($Locator -replace "[^A-Za-z0-9]+", "_").Trim("_").ToLowerInvariant()
    return "{0}_{1}" -f $locatorToken, $SurfaceKind
}

function Expand-RequestedObservableSurfaces {
    param(
        [Parameter(Mandatory = $true)]
        $Scenario
    )

    $requestedObservationScopeProperty = $Scenario.PSObject.Properties["requested_observation_scope"]
    if ($null -eq $requestedObservationScopeProperty -or $null -eq $requestedObservationScopeProperty.Value) {
        return
    }

    $requestedObservationScope = $requestedObservationScopeProperty.Value
    $requiredSurfacesProperty = $requestedObservationScope.PSObject.Properties["oxxlplay_required_surfaces"]
    if ($null -eq $requiredSurfacesProperty -or $null -eq $requiredSurfacesProperty.Value) {
        return
    }

    $defaultLocator = $null
    $sourceCellLocatorProperty = $Scenario.PSObject.Properties["source_cell_locator"]
    if ($null -ne $sourceCellLocatorProperty -and -not [string]::IsNullOrWhiteSpace([string]$sourceCellLocatorProperty.Value)) {
        $defaultLocator = [string]$sourceCellLocatorProperty.Value
    }
    elseif ($Scenario.observable_surfaces.Count -gt 0) {
        $defaultLocator = [string]$Scenario.observable_surfaces[0].locator
    }

    if ([string]::IsNullOrWhiteSpace($defaultLocator)) {
        return
    }

    $existingKinds = @{}
    foreach ($surface in $Scenario.observable_surfaces) {
        $existingKinds[[string]$surface.surface_kind] = $true
    }

    foreach ($requestedKind in $requiredSurfacesProperty.Value) {
        $kindName = [string]$requestedKind
        if ($existingKinds.ContainsKey($kindName)) {
            continue
        }

        $Scenario.observable_surfaces += [pscustomobject]([ordered]@{
            surface_id = New-AutoExpandedSurfaceId -Locator $defaultLocator -SurfaceKind $kindName
            surface_kind = $kindName
            locator = $defaultLocator
            required = $true
        })
        $existingKinds[$kindName] = $true
    }
}

function New-ObservedSurfaceRecord {
    param(
        [Parameter(Mandatory = $true)]
        $Workbook,
        [Parameter(Mandatory = $true)]
        $Surface,
        $SpreadsheetMlContext
    )

    $status = "unavailable"
    $valueRepr = $null
    $valueJson = $null
    $captureLoss = "surface_not_captured"
    $uncertainty = "none"

    switch ($Surface.surface_kind) {
        "workbook_identity" {
            $status = "derived"
            $valueRepr = [System.IO.Path]::GetFileName($Workbook.FullName)
            $captureLoss = "none"
            $uncertainty = "workbook_identity_assumed"
        }
        "cell_value" {
            $range = Resolve-SurfaceRange -Workbook $Workbook -Locator $Surface.locator
            try {
                $value = Convert-CellValueToString $range.Value2
                if (-not [string]::IsNullOrWhiteSpace($value)) {
                    $status = "direct"
                    $valueRepr = $value
                    $captureLoss = "none"
                }
            }
            finally {
                Release-ComObject -ComObject $range
            }
        }
        "effective_display_text" {
            $range = Resolve-SurfaceRange -Workbook $Workbook -Locator $Surface.locator
            try {
                $text = [string]$range.Text
                if (-not [string]::IsNullOrWhiteSpace($text)) {
                    $status = "direct"
                    $valueRepr = $text
                    $captureLoss = "none"
                }
            }
            finally {
                Release-ComObject -ComObject $range
            }
        }
        "formula_text" {
            $range = Resolve-SurfaceRange -Workbook $Workbook -Locator $Surface.locator
            try {
                $formula = [string]$range.Formula
                if (-not [string]::IsNullOrWhiteSpace($formula) -and $formula.StartsWith("=")) {
                    $status = "direct"
                    $valueRepr = $formula
                    $captureLoss = "none"
                }
                else {
                    $captureLoss = "formula_unavailable"
                }
            }
            finally {
                Release-ComObject -ComObject $range
            }
        }
        "number_format_code" {
            if ($null -ne $SpreadsheetMlContext -and -not [string]::IsNullOrWhiteSpace($SpreadsheetMlContext.number_format_code)) {
                $status = "derived"
                $valueRepr = [string]$SpreadsheetMlContext.number_format_code
                $captureLoss = "none"
                $uncertainty = "post_processed"
            }
            else {
                $range = Resolve-SurfaceRange -Workbook $Workbook -Locator $Surface.locator
                try {
                    $formatCode = [string]$range.NumberFormat
                    if (-not [string]::IsNullOrWhiteSpace($formatCode)) {
                        $status = "direct"
                        $valueRepr = $formatCode
                        $captureLoss = "none"
                    }
                }
                finally {
                    Release-ComObject -ComObject $range
                }
            }
        }
        "style_id" {
            if ($null -ne $SpreadsheetMlContext -and -not [string]::IsNullOrWhiteSpace($SpreadsheetMlContext.style_id)) {
                $status = "derived"
                $valueRepr = [string]$SpreadsheetMlContext.style_id
                $captureLoss = "none"
                $uncertainty = "post_processed"
            }
            else {
                $range = Resolve-SurfaceRange -Workbook $Workbook -Locator $Surface.locator
                try {
                    $styleName = Get-RangeStyleName -Range $range
                    if (-not [string]::IsNullOrWhiteSpace($styleName)) {
                        $status = "direct"
                        $valueRepr = $styleName
                        $captureLoss = "none"
                    }
                }
                finally {
                    Release-ComObject -ComObject $range
                }
            }
        }
        "font_color" {
            if ($null -ne $SpreadsheetMlContext -and -not [string]::IsNullOrWhiteSpace($SpreadsheetMlContext.font_color)) {
                $status = "derived"
                $valueRepr = [string]$SpreadsheetMlContext.font_color
                $captureLoss = "none"
                $uncertainty = "post_processed"
            }
            else {
                $range = Resolve-SurfaceRange -Workbook $Workbook -Locator $Surface.locator
                try {
                    $color = Convert-ExcelColorToHex $range.Font.Color
                    if (-not [string]::IsNullOrWhiteSpace($color)) {
                        $status = "direct"
                        $valueRepr = $color
                        $captureLoss = "none"
                    }
                }
                finally {
                    Release-ComObject -ComObject $range
                }
            }
        }
        "fill_color" {
            if ($null -ne $SpreadsheetMlContext -and -not [string]::IsNullOrWhiteSpace($SpreadsheetMlContext.fill_color)) {
                $status = "derived"
                $valueRepr = [string]$SpreadsheetMlContext.fill_color
                $captureLoss = "none"
                $uncertainty = "post_processed"
            }
            else {
                $range = Resolve-SurfaceRange -Workbook $Workbook -Locator $Surface.locator
                try {
                    $color = Convert-ExcelColorToHex $range.Interior.Color
                    if (-not [string]::IsNullOrWhiteSpace($color)) {
                        $status = "direct"
                        $valueRepr = $color
                        $captureLoss = "none"
                    }
                }
                finally {
                    Release-ComObject -ComObject $range
                }
            }
        }
        "conditional_formatting_rules" {
            if ($null -ne $SpreadsheetMlContext) {
                $status = "derived"
                $valueJson = Convert-ToConditionalRuleProjection -Rules $SpreadsheetMlContext.conditional_rules
                $captureLoss = "none"
                $uncertainty = "post_processed"
            }
            else {
                $range = Resolve-SurfaceRange -Workbook $Workbook -Locator $Surface.locator
                try {
                    $status = "direct"
                    $valueJson = Get-RangeConditionalFormattingRules -Range $range
                    $captureLoss = "none"
                }
                finally {
                    Release-ComObject -ComObject $range
                }
            }
        }
        "conditional_formatting_effective_style" {
            $range = Resolve-SurfaceRange -Workbook $Workbook -Locator $Surface.locator
            try {
                if ($null -ne $SpreadsheetMlContext) {
                    $worksheet = $null
                    try {
                        $locatorParts = $Surface.locator -split "!", 2
                        $worksheet = $Workbook.Worksheets.Item($locatorParts[0])
                        $status = "derived"
                        $valueJson = Get-SpreadsheetMlEffectiveStyleProjection -Worksheet $worksheet -Range $range -SpreadsheetMlContext $SpreadsheetMlContext
                        $captureLoss = "none"
                        $uncertainty = "post_processed"
                    }
                    finally {
                        Release-ComObject -ComObject $worksheet
                    }
                }
                else {
                    $status = "direct"
                    $valueJson = Get-RangeEffectiveStyleSnapshot -Range $range
                    $captureLoss = "none"
                }
            }
            finally {
                Release-ComObject -ComObject $range
            }
        }
        default {
            $captureLoss = "surface_not_captured"
        }
    }

    $record = [ordered]@{
        surface = [ordered]@{
            surface_id = [string]$Surface.surface_id
            surface_kind = [string]$Surface.surface_kind
            locator = [string]$Surface.locator
            required = [bool]$Surface.required
        }
        status = $status
        value_repr = $valueRepr
    }

    if ($null -ne $valueJson) {
        $record.value_json = $valueJson
    }

    $record.capture_loss = $captureLoss
    $record.uncertainty = $uncertainty

    return $record
}

function Get-OxReplayCaptureLossStatus {
    param(
        [string[]]$CaptureLossSummary
    )

    if ($null -eq $CaptureLossSummary -or $CaptureLossSummary.Count -eq 0) {
        return "none"
    }

    return "downgraded_instrumentation"
}

function New-InterpretationLimit {
    param(
        [Parameter(Mandatory = $true)]
        [string]$Kind,
        [Parameter(Mandatory = $true)]
        [string]$Detail
    )

    return [ordered]@{
        kind = $Kind
        detail = $Detail
    }
}

function New-ProjectedSurfaceEntry {
    param(
        [Parameter(Mandatory = $true)]
        $ObservedSurface
    )

    $entry = [ordered]@{
        surface_id = [string]$ObservedSurface.surface.surface_id
        surface_kind = [string]$ObservedSurface.surface.surface_kind
        locator = [string]$ObservedSurface.surface.locator
        status = [string]$ObservedSurface.status
        capture_loss = [string]$ObservedSurface.capture_loss
        uncertainty = [string]$ObservedSurface.uncertainty
        value_repr = $ObservedSurface.value_repr
    }

    $valueJsonBox = Get-DynamicPropertyBox -Object $ObservedSurface -PropertyName "value_json"
    $valueJson = if ($null -ne $valueJsonBox) { $valueJsonBox.value } else { $null }
    if ($null -ne $valueJson) {
        if ($valueJson -is [System.Array]) {
            $entry.value_json = @($valueJson)
        }
        else {
            $entry.value_json = $valueJson
        }
    }

    return $entry
}

function New-CaptureView {
    param(
        [Parameter(Mandatory = $true)]
        [string]$ScenarioId,
        [Parameter(Mandatory = $true)]
        [string]$ViewFamily,
        [Parameter(Mandatory = $true)]
        [object[]]$ObservedSurfaces
    )

    return [ordered]@{
        scenario_id = $ScenarioId
        view_family = $ViewFamily
        surfaces = @($ObservedSurfaces | ForEach-Object { New-ProjectedSurfaceEntry -ObservedSurface $_ })
    }
}

function Get-ObservedSurfaceComparisonValue {
    param(
        [Parameter(Mandatory = $true)]
        $ObservedSurface
    )

    $valueRepr = Get-DynamicPropertyValue -Object $ObservedSurface -PropertyName "value_repr"
    if ($null -ne $valueRepr) {
        return $valueRepr
    }

    $valueJsonBox = Get-DynamicPropertyBox -Object $ObservedSurface -PropertyName "value_json"
    $valueJson = if ($null -ne $valueJsonBox) { $valueJsonBox.value } else { $null }
    if ($null -eq $valueJson) {
        return $null
    }

    if ($valueJson -is [System.Array]) {
        return @($valueJson)
    }

    return $valueJson
}

function Get-FirstObservedSurfaceByKind {
    param(
        [Parameter(Mandatory = $true)]
        [object[]]$ObservedSurfaces,
        [Parameter(Mandatory = $true)]
        [string]$SurfaceKind
    )

    return @(
        $ObservedSurfaces |
        Where-Object {
            $_.surface.surface_kind -eq $SurfaceKind -and $_.status -ne "unavailable"
        } |
        Select-Object -First 1
    ) | Select-Object -First 1
}

function Convert-ToArrayValue {
    param($Value)

    if ($null -eq $Value) {
        return @()
    }

    if ($Value -is [System.Array]) {
        return @($Value)
    }

    return @($Value)
}

function Convert-ToComparisonRuleKind {
    param(
        [string]$RuleKind
    )

    if ([string]::IsNullOrWhiteSpace($RuleKind)) {
        return $null
    }

    return $RuleKind.ToLowerInvariant()
}

function New-FormattingComparisonViewValue {
    param(
        [Parameter(Mandatory = $true)]
        [object[]]$ObservedSurfaces
    )

    $value = [ordered]@{}
    foreach ($surfaceKind in @("number_format_code", "style_id", "font_color", "fill_color")) {
        $surface = Get-FirstObservedSurfaceByKind -ObservedSurfaces $ObservedSurfaces -SurfaceKind $surfaceKind
        if ($null -eq $surface) {
            continue
        }

        $comparisonValue = Get-ObservedSurfaceComparisonValue -ObservedSurface $surface
        if ($null -ne $comparisonValue) {
            $value[$surfaceKind] = $comparisonValue
        }
    }

    if ($value.Count -eq 0) {
        return $null
    }

    return $value
}

function Convert-ToConditionalFormattingRuleComparisonValue {
    param(
        [Parameter(Mandatory = $true)]
        $Rule
    )

    return [ordered]@{
        range = Get-DynamicPropertyValue -Object $Rule -PropertyName "range"
        formula = Get-DynamicPropertyValue -Object $Rule -PropertyName "formula"
        value1 = Get-DynamicPropertyValue -Object $Rule -PropertyName "value1"
        value2 = Get-DynamicPropertyValue -Object $Rule -PropertyName "value2"
        operator = Get-DynamicPropertyValue -Object $Rule -PropertyName "operator"
        rule_kind = Convert-ToComparisonRuleKind -RuleKind ([string](Get-DynamicPropertyValue -Object $Rule -PropertyName "rule_kind"))
        font_color = Get-DynamicPropertyValue -Object $Rule -PropertyName "font_color"
        fill_color = Get-DynamicPropertyValue -Object $Rule -PropertyName "fill_color"
    }
}

function New-ConditionalFormattingComparisonViewValue {
    param(
        [Parameter(Mandatory = $true)]
        [object[]]$ObservedSurfaces
    )

    $value = [ordered]@{}

    $rulesSurface = Get-FirstObservedSurfaceByKind -ObservedSurfaces $ObservedSurfaces -SurfaceKind "conditional_formatting_rules"
    if ($null -ne $rulesSurface) {
        $rulesValue = Get-ObservedSurfaceComparisonValue -ObservedSurface $rulesSurface
        $normalizedRules = @(
            foreach ($rule in (Convert-ToArrayValue -Value $rulesValue)) {
                Convert-ToConditionalFormattingRuleComparisonValue -Rule $rule
            }
        )

        if ($normalizedRules.Count -gt 0) {
            $value["rules"] = $normalizedRules
        }
    }

    $effectiveStyleSurface = Get-FirstObservedSurfaceByKind -ObservedSurfaces $ObservedSurfaces -SurfaceKind "conditional_formatting_effective_style"
    if ($null -ne $effectiveStyleSurface) {
        $effectiveStyleValue = Get-ObservedSurfaceComparisonValue -ObservedSurface $effectiveStyleSurface
        if ($null -ne $effectiveStyleValue) {
            $normalizedEffectiveStyle = [ordered]@{}
            foreach ($propertyName in @(
                    "number_format_code",
                    "font_color",
                    "fill_color",
                    "effective_display_text",
                    "applied_rule_indexes",
                    "source_projection"
                )) {
                $propertyBox = Get-DynamicPropertyBox -Object $effectiveStyleValue -PropertyName $propertyName
                $propertyValue = if ($null -ne $propertyBox) { $propertyBox.value } else { $null }
                if ($null -ne $propertyValue) {
                    if ($propertyName -eq "applied_rule_indexes") {
                        $normalizedEffectiveStyle[$propertyName] = @($propertyValue)
                    }
                    elseif ($propertyValue -is [System.Array]) {
                        $normalizedEffectiveStyle[$propertyName] = @($propertyValue)
                    }
                    else {
                        $normalizedEffectiveStyle[$propertyName] = $propertyValue
                    }
                }
            }

            if ($normalizedEffectiveStyle.Count -gt 0) {
                $value["effective_style"] = $normalizedEffectiveStyle
            }
        }
    }

    if ($value.Count -eq 0) {
        return $null
    }

    return $value
}

function New-ReplayComparisonViews {
    param(
        [Parameter(Mandatory = $true)]
        [object[]]$ObservedSurfaces
    )

    $comparisonViews = @()

    $visibleValueSurface = Get-FirstObservedSurfaceByKind -ObservedSurfaces $ObservedSurfaces -SurfaceKind "cell_value"
    if ($null -ne $visibleValueSurface) {
        $visibleValue = Get-ObservedSurfaceComparisonValue -ObservedSurface $visibleValueSurface
        if ($null -ne $visibleValue) {
            $comparisonViews += [ordered]@{
                view_family = "visible_value"
                value = $visibleValue
            }
        }
    }

    $effectiveDisplaySurface = Get-FirstObservedSurfaceByKind -ObservedSurfaces $ObservedSurfaces -SurfaceKind "effective_display_text"
    if ($null -ne $effectiveDisplaySurface) {
        $effectiveDisplayValue = Get-ObservedSurfaceComparisonValue -ObservedSurface $effectiveDisplaySurface
        if ($null -ne $effectiveDisplayValue) {
            $comparisonViews += [ordered]@{
                view_family = "effective_display_text"
                value = $effectiveDisplayValue
            }
        }
    }

    $formattingValue = New-FormattingComparisonViewValue -ObservedSurfaces $ObservedSurfaces
    if ($null -ne $formattingValue) {
        $comparisonViews += [ordered]@{
            view_family = "formatting_view"
            value = $formattingValue
        }
    }

    $conditionalFormattingValue = New-ConditionalFormattingComparisonViewValue -ObservedSurfaces $ObservedSurfaces
    if ($null -ne $conditionalFormattingValue) {
        $comparisonViews += [ordered]@{
            view_family = "conditional_formatting_view"
            value = $conditionalFormattingValue
        }
    }

    return @($comparisonViews)
}

function New-ReplaySourceMetadata {
    param(
        [Parameter(Mandatory = $true)]
        $Scenario,
        [Parameter(Mandatory = $true)]
        $Provenance,
        [Parameter(Mandatory = $true)]
        [object[]]$ObservedSurfaces,
        [Parameter(Mandatory = $true)]
        [AllowEmptyCollection()]
        [object[]]$ComparisonViews,
        [Parameter(Mandatory = $true)]
        [AllowEmptyCollection()]
        [string[]]$CaptureLossSummary
    )

    $workbookKind = $null
    $workbookKindProperty = $Scenario.PSObject.Properties["workbook_kind"]
    if ($null -ne $workbookKindProperty -and -not [string]::IsNullOrWhiteSpace([string]$workbookKindProperty.Value)) {
        $workbookKind = [string]$workbookKindProperty.Value
    }

    $unavailableSurfaces = @(
        $ObservedSurfaces |
        Where-Object { $_.status -eq "unavailable" } |
        ForEach-Object {
            [ordered]@{
                surface_id = [string]$_.surface.surface_id
                surface_kind = [string]$_.surface.surface_kind
                locator = [string]$_.surface.locator
                capture_loss = [string]$_.capture_loss
            }
        }
    )

    return [ordered]@{
        source_schema_id = "oxxlplay.normalized_replay.v1"
        projection_status = "lossy"
        capture_mode = "excel_black_box_observation"
        workbook_ref = [string]$Provenance.workbook_ref
        workbook_fingerprint = [string]$Provenance.workbook_fingerprint
        workbook_kind = $workbookKind
        trigger = [string]$Scenario.trigger
        run_id = [string]$Provenance.run_id
        automation_policy = [string]$Provenance.automation_policy
        macro_mode = [string]$Provenance.macro_mode
        captured_at_utc = [string]$Provenance.captured_at_utc
        excel = [ordered]@{
            version = [string]$Provenance.excel_version
            build = [string]$Provenance.excel_build
            channel = [string]$Provenance.excel_channel
        }
        host = [ordered]@{
            os = [string]$Provenance.host_os
            architecture = [string]$Provenance.host_architecture
            timezone = [string]$Provenance.timezone
        }
        capture_loss_summary = @($CaptureLossSummary)
        capture_loss = Get-OxReplayCaptureLossStatus -CaptureLossSummary $CaptureLossSummary
        uncertainty_summary = @($Provenance.uncertainty_summary)
        bridge_influenced = @($Provenance.bridge.interpretation_limits).Count -gt 0
        interpretation_limits = @($Provenance.bridge.interpretation_limits)
        unavailable_surfaces = @($unavailableSurfaces)
        comparison_view_families = @($ComparisonViews | ForEach-Object { [string]$_.view_family })
    }
}

function Convert-ToReplayNormalizedFamily {
    param(
        [Parameter(Mandatory = $true)]
        $ObservedSurface
    )

    $surface = $ObservedSurface.surface
    $prefix = "excel.surface.{0}" -f [string]$surface.surface_kind
    $valueText = Get-ObservedSurfaceValueText -ObservedSurface $ObservedSurface
    switch ([string]$ObservedSurface.status) {
        "direct" {
            if ($null -ne $valueText) {
                return "{0}.direct:{1}={2}" -f $prefix, [string]$surface.locator, $valueText
            }

            return "{0}.direct:{1}" -f $prefix, [string]$surface.locator
        }
        "derived" {
            if ($null -ne $valueText) {
                return "{0}.derived:{1}={2}" -f $prefix, [string]$surface.locator, $valueText
            }

            return "{0}.derived:{1}" -f $prefix, [string]$surface.locator
        }
        default {
            return "{0}.unavailable:{1}:{2}" -f $prefix, [string]$surface.locator, [string]$ObservedSurface.capture_loss
        }
    }
}

$repoRoot = Resolve-FullPath -BasePath $PSScriptRoot -CandidatePath ".."
$resolvedScenarioPath = Resolve-FullPath -BasePath $repoRoot -CandidatePath $ScenarioPath
$resolvedOutputDir = Resolve-FullPath -BasePath $repoRoot -CandidatePath $OutputDir

if (-not (Test-Path -LiteralPath $resolvedScenarioPath)) {
    throw "Scenario path `$resolvedScenarioPath` does not exist."
}

New-Item -ItemType Directory -Path $resolvedOutputDir -Force | Out-Null

$scenario = Get-Content -Raw -Path $resolvedScenarioPath | ConvertFrom-Json -Depth 20
Expand-RequestedObservableSurfaces -Scenario $scenario
$excel = $null
$workbook = $null
$macroMode = "force_disable_requested"

try {
    $excel = New-Object -ComObject Excel.Application
    $excel.Visible = $false
    $excel.DisplayAlerts = $false
    $excel.AskToUpdateLinks = $false

    try {
        $excel.AutomationSecurity = 3
    }
    catch {
        $macroMode = "automation_security_unavailable"
    }

    $resolvedWorkbookPath = Resolve-WorkbookPath -Scenario $scenario -ScenarioPath $resolvedScenarioPath -RepoRoot $repoRoot
    Ensure-BootstrapWorkbook -Excel $excel -Scenario $scenario -WorkbookPath $resolvedWorkbookPath

    $workbook = $excel.Workbooks.Open($resolvedWorkbookPath, 0, $true)

    try {
        $excel.CalculateFullRebuild()
    }
    catch {
        $excel.Calculate()
    }

    $capturedAtUtc = (Get-Date).ToUniversalTime().ToString("yyyy-MM-ddTHH:mm:ssZ")
    $timezone = (Get-TimeZone).Id
    $workbookFingerprint = "sha256:{0}" -f (Get-FileHash -LiteralPath $resolvedWorkbookPath -Algorithm SHA256).Hash.ToLowerInvariant()
    $bridgeVersion = "w006-powershell-com.v1"
    $commandChannel = "json-file"
    $workbookRepoPath = Convert-ToRepoRelativePath -PathValue $resolvedWorkbookPath -RepoRoot $repoRoot
    $outputRepoPath = Convert-ToRepoRelativePath -PathValue $resolvedOutputDir -RepoRoot $repoRoot
    $declaredSurfaceIds = @($scenario.observable_surfaces | ForEach-Object { [string]$_.surface_id })
    $spreadsheetMlContextByLocator = @{}
    foreach ($observableSurface in $scenario.observable_surfaces) {
        $locatorKey = [string]$observableSurface.locator
        if (-not $spreadsheetMlContextByLocator.ContainsKey($locatorKey)) {
            $spreadsheetMlContextByLocator[$locatorKey] = Get-SpreadsheetMlContext -WorkbookPath $resolvedWorkbookPath -Locator $locatorKey
        }
    }
    $observedSurfaces = @($scenario.observable_surfaces | ForEach-Object {
        New-ObservedSurfaceRecord -Workbook $workbook -Surface $_ -SpreadsheetMlContext $spreadsheetMlContextByLocator[[string]$_.locator]
    })
    $captureLossSummary = @(
        $observedSurfaces |
        Where-Object { $_.capture_loss -ne "none" } |
        ForEach-Object { $_.capture_loss } |
        Select-Object -Unique
    )
    $uncertaintySummary = @(
        $observedSurfaces |
        Where-Object { $_.uncertainty -ne "none" } |
        ForEach-Object { $_.uncertainty } |
        Select-Object -Unique
    )
    $interpretationLimits = @()
    if (@($observedSurfaces | Where-Object { $_.surface.surface_kind -eq "effective_display_text" }).Count -gt 0) {
        $interpretationLimits += New-InterpretationLimit `
            -Kind "effective_display_host_rendered" `
            -Detail "effective_display_text reflects host-rendered Excel text on the observation machine."
    }
    if (@($spreadsheetMlContextByLocator.Values | Where-Object { $null -ne $_ }).Count -gt 0) {
        $interpretationLimits += New-InterpretationLimit `
            -Kind "spreadsheet_ml_source_projection" `
            -Detail "SpreadsheetML 2003 style, format, and conditional-formatting rule surfaces are retained from the source workbook as derived observation artifacts when Excel import does not preserve those identifiers directly."
        $interpretationLimits += New-InterpretationLimit `
            -Kind "conditional_formatting_rule_projection" `
            -Detail "conditional_formatting_effective_style is derived for SpreadsheetML expression rules by combining source-declared rule payloads with Excel formula evaluation on the target cell."
    }

    $bridge = [ordered]@{
        scenario_id = [string]$scenario.scenario_id
        bridge_kind = "external_process"
        bridge_version = $bridgeVersion
        executable_identity = "pwsh:scripts/invoke-excel-observation.ps1"
        command_channel = $commandChannel
        invocation_mode = "com_automation"
        interpretation_limits = @($interpretationLimits)
    }

    $capture = [ordered]@{
        surfaces = $observedSurfaces
        interpretation = [ordered]@{
            bridge_influenced = @($interpretationLimits).Count -gt 0
            interpretation_limits = @($interpretationLimits)
        }
    }

    $provenance = [ordered]@{
        scenario_id = [string]$scenario.scenario_id
        run_id = "run_{0}_{1}" -f [string]$scenario.scenario_id, (Get-Date -Format "yyyyMMdd_HHmmss")
        workbook_ref = $workbookRepoPath
        workbook_fingerprint = $workbookFingerprint
        excel_version = [string]$excel.Version
        excel_build = [string]$excel.Build
        excel_channel = Get-ExcelChannel
        host_os = Get-HostOs
        host_architecture = Get-HostArchitecture
        macro_mode = $macroMode
        automation_policy = "clean_room_declared"
        captured_at_utc = $capturedAtUtc
        timezone = $timezone
        declared_surface_ids = $declaredSurfaceIds
        capture_loss_summary = $captureLossSummary
        uncertainty_summary = $uncertaintySummary
        bridge = $bridge
    }

    $environment = [ordered]@{
        scenario_id = [string]$scenario.scenario_id
        excel = [ordered]@{
            version = [string]$excel.Version
            build = [string]$excel.Build
            channel = $provenance.excel_channel
        }
        host = [ordered]@{
            os = $provenance.host_os
            architecture = $provenance.host_architecture
        }
        bridge = [ordered]@{
            kind = $bridge.bridge_kind
            version = $bridge.bridge_version
            command_channel = $bridge.command_channel
            invocation_mode = $bridge.invocation_mode
        }
        macro_mode = $macroMode
        automation_policy = "clean_room_declared"
        captured_at_utc = $capturedAtUtc
        timezone = $timezone
        workbook = [ordered]@{
            ref = $workbookRepoPath
            fingerprint = $workbookFingerprint
        }
    }

    $bundle = $null
    if ($EmitBundle) {
        $bundle = [ordered]@{
            bundle_schema = "oxxlplay.replay_bundle_seed.v1"
            scenario = $scenario
            provenance = $provenance
            capture = $capture
            sidecars = @(
                [ordered]@{
                    kind = "environment_fingerprint"
                    path = "{0}/environment.json" -f $outputRepoPath
                    media_type = "application/json"
                },
                [ordered]@{
                    kind = "bridge_envelope"
                    path = "{0}/bridge.json" -f $outputRepoPath
                    media_type = "application/json"
                },
                [ordered]@{
                    kind = "captured_workbook"
                    path = $workbookRepoPath
                    media_type = Get-WorkbookMediaType -WorkbookPath $resolvedWorkbookPath
                }
            )
            handoff = [ordered]@{
                intended_replay_consumers = @("OxReplay")
                intended_diff_consumers = @("OxCalc", "DnaOneCalc")
                capability_hints = @("O3.bundle_seed_valid", "O5.stable_driver_valid", "O6.spreadsheetml_observation_valid")
                pack_hints = @(
                    "PACK.replay.appliance",
                    "PACK.diff.cross_engine.continuous",
                    "PACK.trace.forensic_plane"
                )
            }
        }
    }

    $visibleValueSurfaces = @($observedSurfaces | Where-Object { $_.surface.surface_kind -eq "cell_value" })
    $effectiveDisplaySurfaces = @($observedSurfaces | Where-Object { $_.surface.surface_kind -eq "effective_display_text" })
    $formattingSurfaces = @(
        $observedSurfaces |
        Where-Object {
            $_.surface.surface_kind -in @("number_format_code", "style_id", "font_color", "fill_color")
        }
    )
    $conditionalFormattingSurfaces = @(
        $observedSurfaces |
        Where-Object {
            $_.surface.surface_kind -in @("conditional_formatting_rules", "conditional_formatting_effective_style")
        }
    )
    $comparisonViews = @(New-ReplayComparisonViews -ObservedSurfaces $observedSurfaces)

    $normalizedReplay = [ordered]@{
        scenario_id = [string]$scenario.scenario_id
        lane_id = "oxxlplay"
        events = @(
            $observedSurfaces | ForEach-Object {
                [ordered]@{
                    event_id = [string]$_.surface.surface_id
                    source_label = "{0}:{1}:{2}" -f [string]$_.surface.surface_kind, [string]$_.surface.locator, [string]$_.status
                    normalized_family = Convert-ToReplayNormalizedFamily -ObservedSurface $_
                }
            }
        )
        registry_refs = @()
        comparison_views = @($comparisonViews)
        source_metadata = New-ReplaySourceMetadata `
            -Scenario $scenario `
            -Provenance $provenance `
            -ObservedSurfaces $observedSurfaces `
            -ComparisonViews $comparisonViews `
            -CaptureLossSummary $captureLossSummary
    }

    $viewArtifacts = @(
        [ordered]@{
            artifact_family = "normalized_replay"
            path = "views/normalized-replay.json"
            value = $normalizedReplay
        }
    )
    if ($visibleValueSurfaces.Count -gt 0) {
        $viewArtifacts += [ordered]@{
            artifact_family = "visible_value"
            path = "views/visible-value.json"
            value = New-CaptureView -ScenarioId ([string]$scenario.scenario_id) -ViewFamily "visible_value" -ObservedSurfaces $visibleValueSurfaces
        }
    }
    if ($effectiveDisplaySurfaces.Count -gt 0) {
        $viewArtifacts += [ordered]@{
            artifact_family = "effective_display_text"
            path = "views/effective-display-text.json"
            value = New-CaptureView -ScenarioId ([string]$scenario.scenario_id) -ViewFamily "effective_display_text" -ObservedSurfaces $effectiveDisplaySurfaces
        }
    }
    if ($formattingSurfaces.Count -gt 0) {
        $viewArtifacts += [ordered]@{
            artifact_family = "formatting_view"
            path = "views/formatting-view.json"
            value = New-CaptureView -ScenarioId ([string]$scenario.scenario_id) -ViewFamily "formatting_view" -ObservedSurfaces $formattingSurfaces
        }
    }
    if ($conditionalFormattingSurfaces.Count -gt 0) {
        $viewArtifacts += [ordered]@{
            artifact_family = "conditional_formatting_view"
            path = "views/conditional-formatting-view.json"
            value = New-CaptureView -ScenarioId ([string]$scenario.scenario_id) -ViewFamily "conditional_formatting_view" -ObservedSurfaces $conditionalFormattingSurfaces
        }
    }

    $oxReplayManifest = [ordered]@{
        bundle_id = "oxxlplay-{0}" -f [string]$scenario.scenario_id
        scenario_id = [string]$scenario.scenario_id
        bundle_schema = "replay.bundle.v1"
        source_schema = "oxxlplay.replay_bundle_seed.v1"
        lane_id = "oxxlplay"
        adapter_id = "oxxlplay.observation.replay.v1"
        capture_mode = "excel_black_box_observation"
        registry_refs = @()
        projection_status = "lossy"
        capture_loss = Get-OxReplayCaptureLossStatus -CaptureLossSummary $captureLossSummary
        sidecars = @(
            [ordered]@{
                artifact_family = "oxxlplay_observation_bundle_seed"
                path = "bundle.json"
            },
            [ordered]@{
                artifact_family = "observation_capture"
                path = "capture.json"
            },
            [ordered]@{
                artifact_family = "observation_provenance"
                path = "provenance.json"
            },
            [ordered]@{
                artifact_family = "environment_fingerprint"
                path = "environment.json"
            },
            [ordered]@{
                artifact_family = "bridge_envelope"
                path = "bridge.json"
            }
        )
        views = @(
            $viewArtifacts | ForEach-Object {
                [ordered]@{
                    artifact_family = [string]$_.artifact_family
                    path = [string]$_.path
                }
            }
        )
    }

    Write-JsonFile -Path (Join-Path $resolvedOutputDir "capture.json") -Value $capture
    Write-JsonFile -Path (Join-Path $resolvedOutputDir "provenance.json") -Value $provenance
    Write-JsonFile -Path (Join-Path $resolvedOutputDir "bridge.json") -Value $bridge
    Write-JsonFile -Path (Join-Path $resolvedOutputDir "environment.json") -Value $environment

    if ($EmitBundle -and $null -ne $bundle) {
        Write-JsonFile -Path (Join-Path $resolvedOutputDir "bundle.json") -Value $bundle
        Write-JsonFile -Path (Join-Path $resolvedOutputDir "oxreplay-manifest.json") -Value $oxReplayManifest
        foreach ($viewArtifact in $viewArtifacts) {
            Write-JsonFile -Path (Join-Path $resolvedOutputDir $viewArtifact.path) -Value $viewArtifact.value
        }
    }

    $emittedFiles = @(
        ("{0}/capture.json" -f $outputRepoPath),
        ("{0}/provenance.json" -f $outputRepoPath),
        ("{0}/bridge.json" -f $outputRepoPath),
        ("{0}/environment.json" -f $outputRepoPath)
    )
    if ($EmitBundle) {
        $emittedFiles += ("{0}/bundle.json" -f $outputRepoPath)
        $emittedFiles += ("{0}/oxreplay-manifest.json" -f $outputRepoPath)
        $emittedFiles += @(
            $viewArtifacts |
            ForEach-Object { "{0}/{1}" -f $outputRepoPath, [string]$_.path }
        )
    }

    $driverRun = [ordered]@{
        scenario_path = Convert-ToRepoRelativePath -PathValue $resolvedScenarioPath -RepoRoot $repoRoot
        output_dir = $outputRepoPath
        workbook_ref = $workbookRepoPath
        emitted_files = $emittedFiles
    }
    Write-JsonFile -Path (Join-Path $resolvedOutputDir "driver-run.json") -Value $driverRun

    Write-Host ("emitted stable observation bundle to {0}" -f (Convert-ToRepoRelativePath -PathValue $resolvedOutputDir -RepoRoot $repoRoot))
}
finally {
    if ($null -ne $workbook) {
        $workbook.Close($false)
    }

    if ($null -ne $excel) {
        $excel.Quit()
    }

    Release-ComObject -ComObject $workbook
    Release-ComObject -ComObject $excel
    [GC]::Collect()
    [GC]::WaitForPendingFinalizers()
}
