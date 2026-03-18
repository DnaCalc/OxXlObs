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
