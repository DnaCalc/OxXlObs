# W002_SCENARIO_AND_CAPTURE_CONTRACT_BASELINE

- Date: 2026-03-18
- Execution state: `in_progress`
- Scope status: retained manifest and capture-shape fixtures are present; Excel-driven observation evidence is still absent.

## Commands
1. `pwsh ./scripts/meta-check.ps1`

## Retained fixture roots exercised
1. `docs/test-corpus/excel/xlobs_manifest_minimal_valid_001/`
2. `docs/test-corpus/excel/xlobs_manifest_invalid_missing_surface_001/`
3. `docs/test-corpus/excel/xlobs_capture_values_formulae_001/`
4. `docs/test-corpus/excel/xlobs_capture_loss_formula_unavailable_001/`

## Validation coverage
1. Scenario validation accepts the minimal valid fixture shape.
2. Scenario validation rejects the missing-surface fixture shape.
3. Capture validation accepts direct value and formula-text fixture shapes.
4. Capture validation accepts an unavailable formula surface only when a non-`none` capture-loss marker is present.

## Current limits
1. No retained Excel-driven observation run is present yet.
2. `provenance_minimal`, `bundle_seed_basic`, and `witness_seed_diff` roots remain scaffolded for later worksets.
