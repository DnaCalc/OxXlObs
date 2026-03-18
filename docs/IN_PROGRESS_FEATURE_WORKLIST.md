# IN_PROGRESS_FEATURE_WORKLIST.md — OxXlObs

## Active bootstrap worksets

1. `W001_REPO_BOOTSTRAP_AND_BOUNDARY`
   - status: in_progress
   - objective: lock repo skeleton, observation boundary, and first package map.
2. `W002_SCENARIO_AND_CAPTURE_CONTRACT_BASELINE`
   - status: in_progress
   - objective: stand up scenario declarations, observable surfaces, and lossiness markers.
   - current baseline: retained manifest and capture-shape fixtures exist under `docs/test-corpus/excel/`; Excel-driven observation evidence is still absent.
3. `W003_ENVIRONMENT_FINGERPRINT_AND_BRIDGE_ENVELOPE`
   - status: complete
   - objective: pin Excel build, host environment, and bridge metadata for retained runs.
   - current baseline: retained provenance, bridge, and environment fingerprint fixtures exist and validate for the declared W003 scope; live driver exercise remains deferred to `W006`.
4. `W004_REPLAY_READY_BUNDLE_EMISSION_AND_HANDOFF`
   - status: planned
   - objective: emit canonical replay-ready bundles for `OxReplay`.
5. `W005_DIFFERENTIAL_WITNESS_SEED_BASELINE`
   - status: planned
   - objective: shape Excel-vs-DNA divergences into replay/diff/explain-ready witness seeds.
6. `W006_STABLE_WINDOWS_EXECUTION_DRIVER`
   - status: planned
   - objective: stand up the first stable Windows execution path for repeatable observation runs.

## Activation note
1. The Rust-first stack is declared for the repo.
2. OxXlObs is centered on observation-to-replay compilation from the first workset.
3. `W004` through `W006` remain sequenced after `W003`; activation still depends on explicit scope, named capability/pack impact, and declared dependencies.

## Reserved follow-on lane entry
1. `OxReplay` is the first consumer expected to validate bundle quality and replay readiness.
2. `OxCalc` is the first DNA lane expected to use OxXlObs evidence for broad differential growth.
3. `OxFml` and `OxVba` should join through narrower initial scenario families.

## Activation rule
Move a workset to `in_progress` only when:
1. scope is explicit,
2. dependencies are known,
3. capability and pack impact are named,
4. no semantic-ownership drift is introduced.
