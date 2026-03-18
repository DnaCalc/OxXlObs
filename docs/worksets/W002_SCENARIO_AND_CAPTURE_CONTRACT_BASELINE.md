# W002_SCENARIO_AND_CAPTURE_CONTRACT_BASELINE

## Objective
Stand up scenario declarations, observable surfaces, and explicit lossiness markers.

## Scope
1. scenario manifest baseline,
2. observable-surface declarations,
3. capture-loss and uncertainty enums,
4. retained baseline fixtures for scenario and capture shapes.

## Dependencies
1. `W001_REPO_BOOTSTRAP_AND_BOUNDARY`

## Exit Gate
1. scenario ids are bound to retained roots,
2. manifest validation exists,
3. capture records can state direct versus derived versus unavailable surfaces.

## Expected capability impact
1. `O0.scenario_valid`
2. `O1.capture_valid`

## Expected pack impact
1. indirect groundwork for `PACK.trace.forensic_plane`

## Environment Preconditions
1. Rust, Cargo, PowerShell.

## Evidence Layout
1. canonical roots: `docs/test-corpus/excel/`, `docs/test-runs/`.

## Scenario Readiness
1. replay classes requiring corpus scenarios before activation:
   - `scenario_manifest_valid`
   - `scenario_manifest_invalid`
   - `capture_surface_basic`
   - `capture_loss_marked`

## Pack-Evidence Traceability
1. `PACK.trace.forensic_plane`
   - replay classes: `capture_surface_basic`, `capture_loss_marked`
