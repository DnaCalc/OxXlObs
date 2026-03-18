# OXXLOBS_SCENARIO_REGISTER.md

## 1. Position
This document assigns stable scenario ids for the first OxXlObs bootstrap worksets.

## 2. Scenario register

| Replay class | Scenario id | Retained root |
|---|---|---|
| `scenario_manifest_valid` | `xlobs_manifest_minimal_valid_001` | `docs/test-corpus/excel/xlobs_manifest_minimal_valid_001/` |
| `scenario_manifest_invalid` | `xlobs_manifest_invalid_missing_surface_001` | `docs/test-corpus/excel/xlobs_manifest_invalid_missing_surface_001/` |
| `capture_surface_basic` | `xlobs_capture_values_formulae_001` | `docs/test-corpus/excel/xlobs_capture_values_formulae_001/` |
| `capture_loss_marked` | `xlobs_capture_loss_formula_unavailable_001` | `docs/test-corpus/excel/xlobs_capture_loss_formula_unavailable_001/` |
| `provenance_minimal` | `xlobs_provenance_excel_build_001` | `docs/test-corpus/excel/xlobs_provenance_excel_build_001/` |
| `bundle_seed_basic` | `xlobs_bundle_seed_handoff_001` | `docs/test-corpus/bundles/xlobs_bundle_seed_handoff_001/` |
| `witness_seed_diff` | `xlobs_witness_seed_divergence_001` | `docs/test-corpus/bundles/xlobs_witness_seed_divergence_001/` |

## 3. W002 retained files
1. `scenario_manifest_valid` and `scenario_manifest_invalid` roots retain `scenario.json`.
2. `capture_surface_basic` and `capture_loss_marked` roots retain both `scenario.json` and `capture.json`.
3. Later-workset roots may remain scaffolded until their worksets advance beyond planning.

## 4. W003 retained files
1. `provenance_minimal` retains `provenance.json` and `bridge.json`.
2. `provenance_minimal` also retains `states/excel/xlobs_provenance_excel_build_001/environment.json`.

## 5. W004 retained files
1. `bundle_seed_basic` retains `bundle.json` and `handoff-validation.json`.
