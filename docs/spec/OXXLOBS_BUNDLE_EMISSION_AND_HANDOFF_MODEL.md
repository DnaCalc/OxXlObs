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
