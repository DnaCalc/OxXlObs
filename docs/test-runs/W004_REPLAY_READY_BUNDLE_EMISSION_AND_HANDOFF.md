# W004_REPLAY_READY_BUNDLE_EMISSION_AND_HANDOFF

- Date: 2026-03-18
- Execution state: `complete`
- Scope status: canonical bundle seed and handoff validation fixtures are present and validated for the declared W004 scope; live bundle emission from a Windows Excel run remains outside this workset.

## Commands
1. `pwsh ./scripts/meta-check.ps1`

## Retained fixture roots exercised
1. `docs/test-corpus/bundles/xlplay_bundle_seed_handoff_001/`

## Validation coverage
1. Bundle validation requires a bundle schema, validated scenario/provenance/capture payloads, repo-relative sidecar refs, and at least one replay consumer.
2. Handoff validation emits a retained summary of bundle schema presence, replay-consumer presence, and sidecar path admissibility.
3. The retained bundle fixture names `OxReplay` as replay consumer and `OxCalc` as diff consumer.

## Current limits
1. The W004 bundle remains fixture-backed rather than emitted from a live Excel run because live driver exercise belongs to later worksets.
2. Diff-seed shaping remains outside W004 scope and is deferred to a later workset.
