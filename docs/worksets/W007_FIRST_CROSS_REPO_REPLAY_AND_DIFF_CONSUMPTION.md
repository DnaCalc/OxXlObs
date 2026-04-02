# W007_FIRST_CROSS_REPO_REPLAY_AND_DIFF_CONSUMPTION

## Objective
Stand up the first cross-repo consumption pass for retained Excel evidence through `OxReplay` and `OxCalc`.

## Scope
1. first `OxReplay` ingestion and validation pass against the retained live Excel bundle,
2. first `OxCalc` comparison pass against the same retained Excel scenario family,
3. inbound observation ledgers from sibling repos where new bundle or comparison constraints are discovered,
4. retained local run summary and handoff notes for the first cross-repo pass.

## Dependencies
1. `W004_REPLAY_READY_BUNDLE_EMISSION_AND_HANDOFF`
2. `W005_DIFFERENTIAL_WITNESS_SEED_BASELINE`
3. `W006_STABLE_WINDOWS_EXECUTION_DRIVER`
4. sibling repos available locally when execution advances beyond planning

## Exit Gate
1. a retained live Excel bundle from `states/excel/` has been consumed by `OxReplay`,
2. the same scenario family has a first retained comparison artifact or explicit comparison handoff path for `OxCalc`,
3. any newly discovered replay or diff-shape constraints are recorded back into OxXlPlay through inbound ledgers or a retained local handoff note.

## Expected capability impact
1. establishes the first post-bootstrap cross-repo consumption baseline over `O3.bundle_seed_valid`, `O4.diff_seed_valid`, and `O5.stable_driver_valid`.

## Expected pack impact
1. direct exercised support for `PACK.replay.appliance`
2. first integrated exercised support for `PACK.diff.cross_engine.continuous`
3. continued support for `PACK.trace.forensic_plane`

## Environment Preconditions
1. local sibling repos present for `OxReplay` and `OxCalc`
2. retained W006 live bundle present under `states/excel/`
3. Rust, Cargo, Git, and PowerShell

## Evidence Layout
1. canonical roots: `states/excel/`, `docs/test-runs/`, `docs/handoffs/`, `docs/upstream/`

## Scenario Readiness
1. initial scenario family: `capture_surface_basic`
2. initial retained source observation bundle: `states/excel/xlplay_capture_values_formulae_001/bundle.json`
3. initial canonical replay-facing manifest: `states/excel/xlplay_capture_values_formulae_001/oxreplay-manifest.json`
4. initial replay classes touched: `capture_surface_basic`, `bundle_seed_basic`, `witness_seed_diff`

## Pack-Evidence Traceability
1. `PACK.replay.appliance`
   - retained `OxReplay` ingestion and validation record over the live bundle
2. `PACK.diff.cross_engine.continuous`
   - retained `OxCalc` comparison artifact or comparison handoff over the same scenario family
3. `PACK.trace.forensic_plane`
   - preserved Excel-side provenance and capture-loss evidence carried forward from `states/excel/`
