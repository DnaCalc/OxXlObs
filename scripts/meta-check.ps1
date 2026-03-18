param(
    [switch]$NoFmt,
    [switch]$NoClippy,
    [switch]$NoTest
)

$ErrorActionPreference = "Stop"

if (-not $NoFmt) {
    cargo fmt --all --check
}

if (-not $NoClippy) {
    cargo clippy --workspace --all-targets --all-features -- -D warnings
}

if (-not $NoTest) {
    cargo test --workspace
}
