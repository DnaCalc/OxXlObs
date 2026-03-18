# W004_REPLAY_READY_BUNDLE_EMISSION_AND_HANDOFF

## Objective
Emit canonical replay-ready bundles for `OxReplay`.

## Scope
1. bundle seed assembly,
2. sidecar refs,
3. handoff validation output.

## Dependencies
1. `W003_ENVIRONMENT_FINGERPRINT_AND_BRIDGE_ENVELOPE`

## Exit Gate
1. replay-ready bundle seed format exists,
2. retained bundle seed fixture exists,
3. handoff validation output exists.

## Expected capability impact
1. `O3.bundle_seed_valid`

## Expected pack impact
1. groundwork for `PACK.replay.appliance`

## Environment Preconditions
1. Rust, Cargo, PowerShell.

## Evidence Layout
1. canonical roots: `docs/test-corpus/bundles/`, `docs/test-runs/`.

## Scenario Readiness
1. replay classes: `bundle_seed_basic`
2. scenario id: `xlobs_bundle_seed_handoff_001`

## Pack-Evidence Traceability
1. `PACK.replay.appliance`
   - replay class: `bundle_seed_basic`
