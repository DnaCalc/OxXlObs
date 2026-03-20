# NOTES_FOR_OXREPLAY

## Purpose
Record `OxXlObs` observations that materially affect the `OxReplay` intake seam, canonical bundle alignment, and the ownership boundary around live Excel execution.

## Acknowledgement
1. `OxXlObs` acknowledges the `OxReplay` seam packet in `../OxReplay/docs/spec/OXREPLAY_OXXLOBS_OBSERVATION_SEAM.md`.
2. We agree with the preference for direct canonical replay-bundle emission over opaque automation logs.
3. We agree that capture-loss, downgraded instrumentation, uncertainty, and source identity must remain explicit in replay-facing artifacts.

## Response
1. `OxXlObs` is now aligning on a dual-artifact model:
   - a rich `OxXlObs` observation bundle remains the source-observation contract,
   - an `OxReplay`-canonical `replay.bundle.v1` manifest is emitted as the replay-facing intake artifact over that richer source bundle.
2. The current retained example is:
   - canonical manifest: `states/excel/xlobs_capture_values_formulae_001/oxreplay-manifest.json`
   - normalized replay view: `states/excel/xlobs_capture_values_formulae_001/views/normalized-replay.json`
   - source observation bundle: `states/excel/xlobs_capture_values_formulae_001/bundle.json`
3. The current canonical-manifest choices are:
   - `lane_id`: `oxxlobs`
   - `adapter_id`: `oxxlobs.observation.replay.v1`
   - `source_schema`: `oxxlobs.replay_bundle_seed.v1`
   - `capture_mode`: `excel_black_box_observation`
   - `projection_status`: `lossy`

## Clarifications
1. `OxXlObs` should remain the primary Excel-driving subsystem.
2. `OxReplay` should replay retained artifacts and may orchestrate re-execution requests, but live Excel re-execution itself should route back through `OxXlObs` rather than moving Excel-driving logic into `OxReplay`.
3. In this seam, `lane_id = oxxlobs` should be read as an observation-source lane id for replay intake, not as a claim that `OxXlObs` has become the semantic owner of Excel behavior.
4. The current normalized replay view is intentionally a first-pass projection over observed surfaces; it is useful for ingest and replay-path activation, but it is not yet the final shape for broad Excel-vs-DNA equivalence claims.

## Current Evidence
1. `OxReplay` validates the emitted canonical manifest successfully through:
   - `dna-recalc validate-bundle --bundle ../OxXlObs/states/excel/xlobs_capture_values_formulae_001/oxreplay-manifest.json --format json`
2. `OxReplay` also loads the emitted normalized replay view successfully through:
   - `dna-recalc replay --bundle ../OxXlObs/states/excel/xlobs_capture_values_formulae_001/views/normalized-replay.json --kind normalized-replay`
3. The retained validation outputs are:
   - `states/excel/xlobs_capture_values_formulae_001/oxreplay-validate-bundle-report.json`
   - `states/excel/xlobs_capture_values_formulae_001/oxreplay-replay-report.json`

## Minimum Invariants
1. `OxReplay` should not become the owner of Excel-driving or COM-automation logic.
2. `OxXlObs` should keep direct observation, derived observation, unavailable surfaces, and capture-loss explicit in the source artifact family.
3. The replay-facing canonical manifest should preserve traceability back to the source observation bundle and scenario id.
4. If a replay view is a lossy projection, that lossiness must remain explicit in the canonical manifest and must not erase the richer source-observation sidecars.

## Proposed First Stable Scenario Set
1. minimal valid replay-facing scenario:
   - `xlobs_capture_values_formulae_001`
2. first capture-loss scenario:
   - `xlobs_capture_loss_formula_unavailable_001`
3. first comparison-ready scenario against a DNA lane:
   - start with the same `xlobs_capture_values_formulae_001` workbook family projected into a simple equality/diff target for `OxCalc`

## Open Questions For OxReplay
1. Is `lane_id = oxxlobs` acceptable as the canonical intake id for an observation-source seam that is not a semantic lane?
2. Is `projection_status = lossy` the right first-pass declaration for the normalized replay view emitted over richer observation sidecars?
3. Is an empty `registry_refs` list acceptable for the first Excel-origin intake pass, or should we pin at least `capability_level` from the first pass?
4. Does `OxReplay` want the first acceptance packet to continue as a direct canonical-manifest path, or should we also add a formal adapter manifest immediately?
5. Should the first value-sensitive Excel-vs-DNA diff continue to encode observed values into normalized replay families until the shared diff surface grows more structure?
