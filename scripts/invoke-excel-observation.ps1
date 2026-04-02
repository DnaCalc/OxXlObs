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

function New-ObservedSurfaceRecord {
    param(
        [Parameter(Mandatory = $true)]
        $Workbook,
        [Parameter(Mandatory = $true)]
        $Surface
    )

    $status = "unavailable"
    $valueRepr = $null
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
        default {
            $captureLoss = "surface_not_captured"
        }
    }

    return [ordered]@{
        surface = [ordered]@{
            surface_id = [string]$Surface.surface_id
            surface_kind = [string]$Surface.surface_kind
            locator = [string]$Surface.locator
            required = [bool]$Surface.required
        }
        status = $status
        value_repr = $valueRepr
        capture_loss = $captureLoss
        uncertainty = $uncertainty
    }
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

function Convert-ToReplayNormalizedFamily {
    param(
        [Parameter(Mandatory = $true)]
        $ObservedSurface
    )

    $surface = $ObservedSurface.surface
    $prefix = "excel.surface.{0}" -f [string]$surface.surface_kind
    switch ([string]$ObservedSurface.status) {
        "direct" {
            if ($null -ne $ObservedSurface.value_repr) {
                return "{0}.direct:{1}={2}" -f $prefix, [string]$surface.locator, [string]$ObservedSurface.value_repr
            }

            return "{0}.direct:{1}" -f $prefix, [string]$surface.locator
        }
        "derived" {
            if ($null -ne $ObservedSurface.value_repr) {
                return "{0}.derived:{1}={2}" -f $prefix, [string]$surface.locator, [string]$ObservedSurface.value_repr
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
    $observedSurfaces = @($scenario.observable_surfaces | ForEach-Object {
        New-ObservedSurfaceRecord -Workbook $workbook -Surface $_
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

    $bridge = [ordered]@{
        scenario_id = [string]$scenario.scenario_id
        bridge_kind = "external_process"
        bridge_version = $bridgeVersion
        executable_identity = "pwsh:scripts/invoke-excel-observation.ps1"
        command_channel = $commandChannel
        invocation_mode = "com_automation"
        interpretation_limits = @()
    }

    $capture = [ordered]@{
        surfaces = $observedSurfaces
        interpretation = [ordered]@{
            bridge_influenced = $false
            interpretation_limits = @()
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
                    media_type = "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet"
                }
            )
            handoff = [ordered]@{
                intended_replay_consumers = @("OxReplay")
                intended_diff_consumers = @("OxCalc")
                capability_hints = @("O3.bundle_seed_valid", "O5.stable_driver_valid")
                pack_hints = @(
                    "PACK.replay.appliance",
                    "PACK.diff.cross_engine.continuous",
                    "PACK.trace.forensic_plane"
                )
            }
        }
    }

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
            [ordered]@{
                artifact_family = "normalized_replay"
                path = "views/normalized-replay.json"
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
        Write-JsonFile -Path (Join-Path $resolvedOutputDir "views/normalized-replay.json") -Value $normalizedReplay
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
        $emittedFiles += ("{0}/views/normalized-replay.json" -f $outputRepoPath)
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
