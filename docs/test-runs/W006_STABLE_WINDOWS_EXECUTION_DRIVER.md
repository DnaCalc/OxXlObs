# W006_STABLE_WINDOWS_EXECUTION_DRIVER

- Date: 2026-03-18
- Execution state: `complete`
- Scope status: the first stable Windows Excel execution path is present and exercised through the repo CLI for the declared `capture_surface_basic` scenario family.

## Commands
1. `cargo run -p oxxlobs-cli -- capture-run --scenario docs/test-corpus/excel/xlobs_capture_values_formulae_001/scenario.json --output-dir states/excel/xlobs_capture_values_formulae_001`
2. `pwsh ./scripts/meta-check.ps1`

## Retained roots exercised
1. `docs/test-corpus/excel/xlobs_capture_values_formulae_001/`
2. `states/excel/xlobs_capture_values_formulae_001/`

## Driver evidence summary
1. The stable path runs through `dna-xl-obs capture-run`, which invokes `scripts/invoke-excel-observation.ps1` as a narrow external bridge seam.
2. The retained live run captured `sheet1_a1_value = 42` and `sheet1_a1_formula = =SUM(B1:B3)` from a tracked workbook input.
3. The retained live provenance records Excel `16.0` build `19725`, Windows `Microsoft Windows 11 Pro` on `x64`, macro mode `force_disable_requested`, and explicit COM bridge provenance.
4. The retained live bundle preserves repo-relative sidecars for the environment and bridge witnesses under `states/excel/xlobs_capture_values_formulae_001/`.

## Current limits
1. The stable driver path is currently exercised only for `xlobs_capture_values_formulae_001`.
2. The repo-local PowerShell bridge currently supports the bootstrap surface family needed for `capture_surface_basic`; broader scenario families remain later work.
3. The retained Office channel field may carry the raw local Click-to-Run channel string when the host reports a URL rather than a normalized label.
