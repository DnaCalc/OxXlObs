# OXXLPLAY_IMPLEMENTATION_BASELINE.md

## 1. Position
This document freezes the current Rust-first implementation baseline without pretending the final package graph is permanent.

## 2. Active implementation direction
1. OxXlPlay is Rust-first.
2. The active implementation lives under `src/` as a repo-root Cargo workspace.
3. A narrow external bridge seam remains allowed for Windows-specific Excel driving where required.
4. The current W006 bridge seam is a repo-local PowerShell COM driver invoked from the Rust CLI rather than an embedded semantic engine.

## 3. Initial crate responsibilities
1. `oxxlplay-abstractions`
2. `oxxlplay-scenario`
3. `oxxlplay-capture`
4. `oxxlplay-provenance`
5. `oxxlplay-bundle`
6. `oxxlplay-bridge`
7. `oxxlplay-cli`

## 4. Validation floor
1. `cargo fmt --all --check`
2. `cargo clippy --workspace --all-targets --all-features -- -D warnings`
3. `cargo test --workspace`
4. `pwsh ./scripts/meta-check.ps1`
