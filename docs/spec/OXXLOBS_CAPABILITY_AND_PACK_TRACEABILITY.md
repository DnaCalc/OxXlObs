# OXXLOBS_CAPABILITY_AND_PACK_TRACEABILITY.md

## 1. Position
This document maps OxXlObs bootstrap surfaces to local capabilities and Foundation pack touchpoints.

## 2. Local capability ladder
1. `O0.scenario_valid`
2. `O1.capture_valid`
3. `O2.provenance_valid`
4. `O3.bundle_seed_valid`
5. `O4.diff_seed_valid`
6. `O5.stable_driver_valid`

These are local observation capabilities, not Foundation replay capability claims.

## 3. Capability traceability

| Local capability | Workset | Replay classes | Artifact roots |
|---|---|---|---|
| `O0.scenario_valid` | `W002` | `scenario_manifest_valid`, `scenario_manifest_invalid` | `docs/test-corpus/excel/`, `docs/test-runs/` |
| `O1.capture_valid` | `W002` | `capture_surface_basic`, `capture_loss_marked` | `docs/test-corpus/excel/`, `docs/test-runs/` |
| `O2.provenance_valid` | `W003` | `provenance_minimal` | `docs/test-corpus/excel/`, `states/excel/`, `docs/test-runs/` |
| `O3.bundle_seed_valid` | `W004` | `bundle_seed_basic` | `docs/test-corpus/bundles/`, `docs/test-runs/` |
| `O4.diff_seed_valid` | `W005` | `witness_seed_diff` | `docs/test-corpus/bundles/`, `docs/test-runs/` |
| `O5.stable_driver_valid` | `W006` | `capture_surface_basic`, `bundle_seed_basic`, `witness_seed_diff` | `docs/test-runs/`, `states/excel/` |

## 4. Pack traceability

| Pack | Workset | Replay classes | Artifact roots |
|---|---|---|---|
| `PACK.replay.appliance` | `W004`, `W005` | `bundle_seed_basic`, `witness_seed_diff` | `docs/test-corpus/bundles/`, `docs/test-runs/` |
| `PACK.diff.cross_engine.continuous` | `W005`, `W006` | `witness_seed_diff` | `docs/test-corpus/bundles/`, `docs/test-runs/` |
| `PACK.trace.forensic_plane` | `W002` through `W006` | `capture_surface_basic`, `capture_loss_marked`, `provenance_minimal`, `witness_seed_diff` | `docs/test-corpus/excel/`, `docs/test-runs/`, `states/excel/` |
