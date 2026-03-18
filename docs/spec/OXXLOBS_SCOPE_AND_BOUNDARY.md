# OXXLOBS_SCOPE_AND_BOUNDARY.md

## 1. Position
This document defines what OxXlObs is allowed to own and what it must leave to Foundation, `OxReplay`, and the DNA Calc lane repos.

## 2. Repo purpose
OxXlObs is the shared implementation substrate for Excel observation and observation-to-replay compilation.

It exists to provide reusable mechanics for:
1. scenario planning,
2. Excel-run observation,
3. provenance capture,
4. lossiness reporting,
5. replay-ready bundle emission,
6. witness-seed preparation for later replay analysis.

## 3. In scope
1. Shared observation abstractions and scenario types.
2. Controlled Excel trigger and capture recipes.
3. Provenance and environment fingerprint handling.
4. Replay-ready bundle assembly and sidecar emission.
5. Differential witness-seed scaffolding for Excel-vs-DNA comparisons.
6. CLI and local tool surfaces for repeatable observation runs.

## 4. Out of scope
1. Semantic ownership of Excel behavior.
2. Replay execution, diff, explain, and witness lifecycle governance.
3. Lane-local semantic ownership for DNA Calc repos.
4. Broad Office automation unrelated to retained observation evidence.

## 5. Ownership split
1. Foundation owns doctrine and clean-room governance.
2. OxXlObs owns Excel observation mechanics and replay-ready evidence compilation.
3. `OxReplay` owns shared replay runtime and evidence analysis.
4. Lane repos own DNA Calc semantics and adapter meaning.
