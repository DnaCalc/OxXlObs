# W005_DIFFERENTIAL_WITNESS_SEED_BASELINE

- Date: 2026-03-18
- Execution state: `complete`
- Scope status: canonical differential witness-seed fixture is present and validated for the declared W005 scope; live cross-engine differential execution remains outside this workset.

## Commands
1. `pwsh ./scripts/meta-check.ps1`

## Retained fixture roots exercised
1. `docs/test-corpus/bundles/xlplay_witness_seed_divergence_001/`

## Validation coverage
1. Witness validation requires a witness schema, a validated source bundle, at least one DNA-side comparison ref, and at least one retained divergence.
2. Comparison refs retain lane id, artifact ref, adapter identity/version, capability level, and engine config ref.
3. Divergence records retain mismatch kind, severity, comparison note, and explanatory note while preserving the originating Excel-side provenance and lossiness inside the source bundle.
4. Quarantined witness lifecycle states require an explicit quarantine reason.

## Current limits
1. The W005 witness seed remains fixture-backed rather than emitted from a live differential run because stable driver execution is a later workset.
2. Sibling inbound observation ledgers were checked for this workset and were absent locally.
3. Stable Windows driver execution remains outside W005 scope and is deferred to a later workset.
