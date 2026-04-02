# W005_DIFFERENTIAL_WITNESS_SEED_BASELINE

## Objective
Shape Excel-vs-DNA divergences into replay/diff/explain-ready witness seeds.

## Scope
1. differential seed record,
2. DNA-side comparison refs,
3. retained divergence fixture.

## Dependencies
1. `W004_REPLAY_READY_BUNDLE_EMISSION_AND_HANDOFF`

## Exit Gate
1. witness-seed output exists,
2. retained divergence fixture exists,
3. divergence output preserves provenance and lossiness.

## Expected capability impact
1. `O4.diff_seed_valid`

## Expected pack impact
1. groundwork for `PACK.diff.cross_engine.continuous`
2. indirect support for `PACK.replay.appliance`

## Environment Preconditions
1. Rust, Cargo, PowerShell.
2. DNA-side comparison inputs provided by sibling repos or `OxReplay`.

## Evidence Layout
1. canonical roots: `docs/test-corpus/bundles/`, `docs/test-runs/`.

## Scenario Readiness
1. replay classes: `witness_seed_diff`
2. scenario id: `xlplay_witness_seed_divergence_001`

## Pack-Evidence Traceability
1. `PACK.diff.cross_engine.continuous`
   - replay class: `witness_seed_diff`
