# OXXLPLAY_SCENARIO_REGISTER.md

## 1. Position
This document assigns stable scenario ids for the first OxXlPlay bootstrap worksets.

## 2. Scenario register

| Replay class | Scenario id | Retained root |
|---|---|---|
| `scenario_manifest_valid` | `xlplay_manifest_minimal_valid_001` | `docs/test-corpus/excel/xlplay_manifest_minimal_valid_001/` |
| `scenario_manifest_invalid` | `xlplay_manifest_invalid_missing_surface_001` | `docs/test-corpus/excel/xlplay_manifest_invalid_missing_surface_001/` |
| `capture_surface_basic` | `xlplay_capture_values_formulae_001` | `docs/test-corpus/excel/xlplay_capture_values_formulae_001/` |
| `capture_surface_spreadsheetml_formatting` | `xlplay_capture_spreadsheetml_formatting_001` | `docs/test-corpus/excel/xlplay_capture_spreadsheetml_formatting_001/` |
| `capture_loss_marked` | `xlplay_capture_loss_formula_unavailable_001` | `docs/test-corpus/excel/xlplay_capture_loss_formula_unavailable_001/` |
| `provenance_minimal` | `xlplay_provenance_excel_build_001` | `docs/test-corpus/excel/xlplay_provenance_excel_build_001/` |
| `bundle_seed_basic` | `xlplay_bundle_seed_handoff_001` | `docs/test-corpus/bundles/xlplay_bundle_seed_handoff_001/` |
| `witness_seed_diff` | `xlplay_witness_seed_divergence_001` | `docs/test-corpus/bundles/xlplay_witness_seed_divergence_001/` |

## 3. W002 retained files
1. `scenario_manifest_valid` and `scenario_manifest_invalid` roots retain `scenario.json`.
2. `capture_surface_basic` and `capture_loss_marked` roots retain both `scenario.json` and `capture.json`.
3. Later-workset roots may remain scaffolded until their worksets advance beyond planning.
4. `capture_surface_spreadsheetml_formatting` retains `scenario.json`, `capture.json`, and `workbook.xml` for the first SpreadsheetML 2003 observation family.

## 4. W003 retained files
1. `provenance_minimal` retains `provenance.json` and `bridge.json`.
2. `provenance_minimal` also retains `states/excel/xlplay_provenance_excel_build_001/environment.json`.

## 5. W004 retained files
1. `bundle_seed_basic` retains `bundle.json` and `handoff-validation.json`.

## 6. W005 retained files
1. `witness_seed_diff` retains `witness-seed.json`.

## 7. W006 retained files
1. `capture_surface_basic` also retains `workbook.xlsx` under `docs/test-corpus/excel/xlplay_capture_values_formulae_001/`.
2. `capture_surface_basic` retains live driver outputs under `states/excel/xlplay_capture_values_formulae_001/`:
   - `capture.json`
   - `provenance.json`
   - `bridge.json`
   - `environment.json`
   - `driver-run.json`
   - `bundle.json`

## 8. W007 retained files
1. `capture_surface_basic` now also retains the first `OxReplay`-facing canonical intake artifacts under `states/excel/xlplay_capture_values_formulae_001/`:
   - `oxreplay-manifest.json`
   - `views/normalized-replay.json`
   - `oxreplay-validate-bundle-report.json`
   - `oxreplay-replay-report.json`

## 9. W008 retained files
1. `capture_surface_spreadsheetml_formatting` retains live driver outputs under `states/excel/xlplay_capture_spreadsheetml_formatting_001/`:
   - `capture.json`
   - `provenance.json`
   - `bridge.json`
   - `environment.json`
   - `driver-run.json`
   - `bundle.json`
   - `oxreplay-manifest.json`
   - `views/normalized-replay.json`
   - `views/visible-value.json`
   - `views/effective-display-text.json`
   - `views/formatting-view.json`
   - `views/conditional-formatting-view.json`
