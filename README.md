# OxXlObs

OxXlObs is the Excel observation harness repo for DNA Calc.

It exists to turn controlled Excel runs into replay-ready, schema-checked evidence bundles that feed the Replay appliance without turning Excel into a new semantics lane.

## Core responsibilities
1. Scenario planning for reproducible Excel observation runs.
2. Environment, build, workbook, and trigger fingerprint capture.
3. Stable capture of observable Excel surfaces and explicit lossiness markers.
4. Compilation of observations into canonical replay-ready bundles for `OxReplay`.
5. Differential-run seeding for DNA-vs-Excel investigation and witness growth.
6. CLI and local tooling for repeatable observation collection.

## Not this repo
1. Not a new semantics lane.
2. Not the owner of Excel semantic truth.
3. Not the owner of replay doctrine or witness lifecycle governance.
4. Not a generic Office automation dumping ground.

## Startup docs
`AGENTS.md` is the authoritative startup path for agents and doctrinal work.

Minimum repo-orientation read order:
1. `README.md`
2. `AGENTS.md`
3. `CHARTER.md`
4. `OPERATIONS.md`
5. `CURRENT_BLOCKERS.md`
6. `docs/IN_PROGRESS_FEATURE_WORKLIST.md`
7. `docs/worksets/README.md`
8. `docs/spec/README.md`

## Bootstrap workspace layout
1. `docs/spec/`
   - repo-owned mutable specs for observation, provenance, capture, and replay-handoff boundaries.
2. `docs/worksets/`
   - execution packets for staged delivery.
3. `docs/handoffs/`
   - structured cross-repo or Foundation-facing handoff packets.
4. `docs/upstream/`
   - outbound observation ledgers for sibling repos and hosts.
5. `docs/test-corpus/`
   - retained Excel fixtures, observation scenarios, and replay-ready bundle seeds.
6. `docs/test-runs/`
   - retained human-readable validation runs.
7. `src/`, `tests/`, `tools/`, `scripts/`, `states/`, `formal/`
   - code, harness, state, and formalization roots to be populated as implementation advances.

## Bootstrap status
The repo starts doc-first and Rust-first so the clean-room observation boundary and replay handoff contract are fixed before heavy Excel-driving work lands.
The bootstrap packet is now `complete` through `W006`, with the first retained live Excel-driven capture family exercised under `states/excel/`.
The next lane widens from local bootstrap into cross-repo replay and differential consumption.
The current `W007` direction is to emit an `OxReplay`-canonical replay manifest over the richer `OxXlObs` observation bundle rather than collapsing the observation contract into a thin shared-runtime shape.

## Implementation Direction
1. OxXlObs is Rust-first for scenario planning, provenance handling, bundle assembly, CLI surfaces, and most harness logic.
2. The active implementation lives under `src/` as a Cargo workspace with crate boundaries that follow the declared observation strata.
3. A narrow external bridge seam is allowed where stable Excel automation requires Windows-specific interop that is better hosted outside pure Rust. That seam must remain explicit, versioned, and evidence-bearing.
4. New execution should follow the local Rust quality floor:
   - `cargo fmt --all --check`
   - `cargo clippy --workspace --all-targets --all-features -- -D warnings`
   - `cargo test --workspace`
   - `pwsh ./scripts/meta-check.ps1`

## Foundation alignment
Precedence and constitutional constraints are inherited from:
1. `../Foundation/CHARTER.md`
2. `../Foundation/ARCHITECTURE_AND_REQUIREMENTS.md`
3. `../Foundation/OPERATIONS.md`
4. `../Foundation/REPLAY_APPLIANCE.md`

## Dependency constitution
1. May depend on shared schema and evidence contracts.
2. May emit replay-ready artifacts for `OxReplay`.
3. Must use clean-room admissible evidence sources only.
4. Must not become a second semantic authority for Excel or for DNA Calc lanes.
