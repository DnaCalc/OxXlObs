# W003_ENVIRONMENT_FINGERPRINT_AND_BRIDGE_ENVELOPE

## Objective
Pin Excel build, host environment, and bridge metadata for retained runs.

## Scope
1. provenance records,
2. bridge envelope contract,
3. retained environment fingerprint fixtures.

## Dependencies
1. `W002_SCENARIO_AND_CAPTURE_CONTRACT_BASELINE`

## Exit Gate
1. retained provenance fixture exists,
2. bridge kind/version fields are explicit,
3. capture outputs can state whether a bridge influenced interpretation limits.

## Expected capability impact
1. `O2.provenance_valid`

## Expected pack impact
1. indirect support for `PACK.trace.forensic_plane`

## Environment Preconditions
1. Rust, Cargo, PowerShell.
2. Windows-specific driver tools may remain optional until `W006`.

## Evidence Layout
1. canonical roots: `docs/test-corpus/excel/`, `states/excel/`, `docs/test-runs/`.

## Scenario Readiness
1. replay classes: `provenance_minimal`
2. scenario id: `xlplay_provenance_excel_build_001`

## Pack-Evidence Traceability
1. `PACK.trace.forensic_plane`
   - replay classes: `provenance_minimal`
