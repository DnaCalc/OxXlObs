# OXXLOBS_BUNDLE_EMISSION_AND_HANDOFF_MODEL.md

## 1. Position
This document defines how OxXlObs should turn retained observations into replay-ready evidence for `OxReplay`.

## 2. Primary output rule
The primary output is a replay-ready bundle seed, not an opaque automation dump.

The bundle seed should package:
1. scenario declaration,
2. observed surfaces,
3. provenance metadata,
4. capture-loss metadata,
5. sidecar refs for larger retained artifacts,
6. handoff metadata naming intended replay and differential consumers.

## 3. Handoff contract
Bundle output must be shaped so `OxReplay` can later:
1. ingest and validate it deterministically,
2. compare it against DNA lane bundles,
3. explain divergences with preserved provenance,
4. distill retained failures into smaller witnesses.

## 4. W004 retained bundle baseline
The retained W004 baseline uses canonical JSON fixtures for replay-ready bundle seeds and handoff validation output.

### 4.1 Bundle fixture
1. The `bundle_seed_basic` scenario root retains `bundle.json`.
2. Minimum retained fields are:
   - `bundle_schema`
   - `scenario`
   - `provenance`
   - `capture`
   - `sidecars`
   - `handoff`
3. Each sidecar ref retains:
   - `kind`
   - `path`
   - `media_type`
4. Sidecar refs must remain repo-relative.

### 4.2 Handoff metadata
1. Handoff metadata retains:
   - `intended_replay_consumers`
   - `intended_diff_consumers`
   - `capability_hints`
   - `pack_hints`
2. At least one replay consumer must be declared.
3. The W004 baseline names `OxReplay` explicitly as the replay consumer.

### 4.3 Handoff validation output
1. The `bundle_seed_basic` scenario root also retains `handoff-validation.json`.
2. Handoff validation output retains:
   - `bundle_schema`
   - `checked_consumers`
   - `valid`
   - `notes`
3. The validation output is a retained handoff witness, not a semantics claim.

## 5. W005 retained witness baseline
The retained W005 baseline uses canonical JSON fixtures for differential witness seeds.

### 5.1 Witness fixture
1. The `witness_seed_diff` scenario root retains `witness-seed.json`.
2. Minimum retained fields are:
   - `witness_schema`
   - `source_bundle`
   - `comparison_refs`
   - `divergences`
   - `lifecycle_state`
   - `quarantine_reason`
3. `source_bundle` preserves the originating Excel-side provenance and capture-loss lanes.

### 5.2 DNA-side comparison refs
1. Comparison refs retain:
   - `lane_id`
   - `producer_id`
   - `artifact_ref`
   - `adapter_id`
   - `adapter_version`
   - `capability_level`
   - `engine_config_ref`
2. W005 uses retained comparison refs rather than live sibling-repo execution.

### 5.3 Divergence records
1. Divergence records retain:
   - `surface_id`
   - `mismatch_kind`
   - `severity`
   - `excel_value_repr`
   - `comparison_value_repr`
   - `comparison_capture_loss_note`
   - `explanatory_note`
2. The retained divergence fixture must preserve enough provenance and lossiness to remain replay/diff/explain-ready without claiming semantic authority.

## 6. W006 live bundle retention
1. `W006` retains a live replay-ready bundle under `states/excel/xlobs_capture_values_formulae_001/bundle.json`.
2. The live bundle preserves the stable driver bridge provenance emitted during the retained Excel run rather than reusing the fixture-backed W004 provenance.
3. Sidecar refs for live driver bundles remain repo-relative and may point into `states/excel/` for retained environment and bridge witnesses.
