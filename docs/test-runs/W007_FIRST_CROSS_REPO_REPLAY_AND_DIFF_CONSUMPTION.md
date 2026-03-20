# W007_FIRST_CROSS_REPO_REPLAY_AND_DIFF_CONSUMPTION

- Date: 2026-03-20
- Execution state: `in_progress`
- Scope status: the first `OxReplay` ingestion and replay-path activation pass is retained; the `OxCalc` comparison leg remains open.

## Commands
1. `cargo run -p oxxlobs-cli -- capture-run --scenario docs/test-corpus/excel/xlobs_capture_values_formulae_001/scenario.json --output-dir states/excel/xlobs_capture_values_formulae_001`
2. `cargo run -p oxreplay-dnarecalc-cli -- validate-bundle --bundle ../OxXlObs/states/excel/xlobs_capture_values_formulae_001/oxreplay-manifest.json --format json`
3. `cargo run -p oxreplay-dnarecalc-cli -- replay --bundle ../OxXlObs/states/excel/xlobs_capture_values_formulae_001/views/normalized-replay.json --kind normalized-replay`
4. `pwsh ./scripts/meta-check.ps1`

## Retained roots exercised
1. `states/excel/xlobs_capture_values_formulae_001/`
2. `docs/upstream/NOTES_FOR_OXREPLAY.md`

## Current W007 baseline
1. `OxXlObs` now emits an `OxReplay`-canonical replay manifest at `states/excel/xlobs_capture_values_formulae_001/oxreplay-manifest.json`.
2. The canonical manifest validates successfully through `OxReplay` and resolves the retained sidecars and normalized replay view.
3. `OxReplay` also accepts the first normalized replay view at `states/excel/xlobs_capture_values_formulae_001/views/normalized-replay.json`.
4. The richer `OxXlObs` observation bundle remains retained alongside the canonical replay-facing manifest rather than being discarded.

## Current limits
1. The `OxCalc` comparison leg of `W007` remains open.
2. The first normalized replay view is a lossy projection over observed Excel surfaces and is not yet a broad semantic equivalence surface.
3. Adapter-manifest expectations for the `OxXlObs` seam are still open for clarification with `OxReplay`.
