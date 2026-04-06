# IN_PROGRESS_FEATURE_WORKLIST.md — OxXlPlay

## Active bootstrap worksets

1. `W001_REPO_BOOTSTRAP_AND_BOUNDARY`
   - status: complete
   - objective: lock repo skeleton, observation boundary, and first package map.
   - current baseline: doctrine surfaces, spec index, local execution doctrine, canonical retained roots, and first Rust workspace model are explicit.
2. `W002_SCENARIO_AND_CAPTURE_CONTRACT_BASELINE`
   - status: complete
   - objective: stand up scenario declarations, observable surfaces, and lossiness markers.
   - current baseline: retained manifest and capture-shape fixtures exist under `docs/test-corpus/excel/`; `capture_surface_basic` now also has a stable live-driver family exercised under `states/excel/`.
3. `W003_ENVIRONMENT_FINGERPRINT_AND_BRIDGE_ENVELOPE`
   - status: complete
   - objective: pin Excel build, host environment, and bridge metadata for retained runs.
   - current baseline: retained provenance, bridge, and environment fingerprint fixtures exist and validate for the declared W003 scope; live driver exercise remains deferred to `W006`.
4. `W004_REPLAY_READY_BUNDLE_EMISSION_AND_HANDOFF`
   - status: complete
   - objective: emit canonical replay-ready bundles for `OxReplay`.
   - current baseline: canonical bundle seed and handoff validation fixtures exist and validate for the declared W004 scope.
5. `W005_DIFFERENTIAL_WITNESS_SEED_BASELINE`
   - status: complete
   - objective: shape Excel-vs-DNA divergences into replay/diff/explain-ready witness seeds.
   - current baseline: canonical witness-seed fixture exists and validates for the declared W005 scope.
6. `W006_STABLE_WINDOWS_EXECUTION_DRIVER`
   - status: complete
   - objective: stand up the first stable Windows execution path for repeatable observation runs.
   - current baseline: `dna-xl-play capture-run` drives Excel through the retained PowerShell COM bridge and emits replay-ready retained evidence under `states/excel/xlplay_capture_values_formulae_001/`.
7. `W007_FIRST_CROSS_REPO_REPLAY_AND_DIFF_CONSUMPTION`
   - status: in_progress
   - objective: stand up the first cross-repo consumption pass through `OxReplay` and `OxCalc` over retained live Excel evidence.
   - current baseline: `OxReplay` now validates an `OxXlPlay`-emitted canonical replay manifest and replay-loads the first normalized replay view from `states/excel/xlplay_capture_values_formulae_001/`; the `OxCalc` comparison leg remains open.
8. `W008_SPREADSHEETML_VERIFICATION_OBSERVATION_EXPANSION`
   - status: in_progress
   - objective: stand up the first SpreadsheetML 2003 observation family for XML-backed downstream verification consumers.
   - current baseline: `capture-run` now retains the first SpreadsheetML family at `states/excel/xlplay_capture_spreadsheetml_formatting_001/`, widens DnaOneCalc-style XML scenarios through `requested_observation_scope.oxxlplay_required_surfaces`, and emits direct display plus derived formatting and conditional-formatting evidence with replay-adjacent view files.

## Activation note
1. The Rust-first stack is declared for the repo.
2. OxXlPlay is centered on observation-to-replay compilation from the first workset.
3. `W006` remains sequenced after `W005`; activation still depends on explicit scope, named capability/pack impact, and declared dependencies.
4. `W006` now retains the first live Excel-driven capture family and associated replay-ready bundle evidence.
5. `W007` is now active over the first `OxReplay` ingestion pass and remains open until the `OxCalc` comparison leg and seam clarifications are retained.
6. `W008` is now active for SpreadsheetML 2003 display/formatting observation expansion and remains open until sibling replay/view consumption clarifications are retained.

## Downstream consumer contract status
1. `DNA OneCalc` observation-consumer contract is now explicit at `docs/spec/OXXLPLAY_ONECALC_OBSERVATION_CONSUMER_CONTRACT.md`.
2. The contract defines the first comparison-ready observation family, comparison envelope, surface classification, lossy-view labeling, and platform scope.
3. Open consumer-side gaps: the `OxCalc` comparison leg of `W007` remains open, the comparison envelope is narrow (cell value and formula text only), no formatting or display-state observation exists, and the richer `OxXlPlay` diff or equality envelope is provisional scope.
4. The first SpreadsheetML observation family now widens the `DNA OneCalc` envelope with direct `effective_display_text` plus derived `number_format_code`, `style_id`, `font_color`, `fill_color`, `conditional_formatting_rules`, and `conditional_formatting_effective_style`.
5. Honest limits remain explicit: those widened SpreadsheetML formatting and conditional-formatting surfaces are currently retained as `derived`, not `direct`, and the CF effective-style projection is scoped to the admitted expression-rule subset.

## Reserved follow-on lane entry
1. `OxReplay` is the first consumer expected to validate bundle quality and replay readiness.
2. `OxCalc` is the first DNA lane expected to use OxXlPlay evidence for broad differential growth.
3. `OxFml` and `OxVba` should join through narrower initial scenario families.

## Activation rule
Move a workset to `in_progress` only when:
1. scope is explicit,
2. dependencies are known,
3. capability and pack impact are named,
4. no semantic-ownership drift is introduced.
