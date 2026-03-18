param(
    [switch]$NoFmt,
    [switch]$NoClippy,
    [switch]$NoTest
)

$ErrorActionPreference = "Stop"

function Invoke-Checked {
    param(
        [string]$FilePath,
        [string[]]$ArgumentList
    )

    & $FilePath @ArgumentList
    if ($LASTEXITCODE -ne 0) {
        exit $LASTEXITCODE
    }
}

if (-not $NoFmt) {
    Invoke-Checked cargo @("fmt", "--all", "--check")
}

if (-not $NoClippy) {
    Invoke-Checked cargo @("clippy", "--workspace", "--all-targets", "--all-features", "--", "-D", "warnings")
}

if (-not $NoTest) {
    Invoke-Checked cargo @("test", "--workspace")
}
