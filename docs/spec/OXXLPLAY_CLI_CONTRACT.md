# OXXLPLAY_CLI_CONTRACT.md

## 1. Position
This document defines the initial command families for the OxXlPlay CLI.

## 2. Initial command families
1. `validate-scenario`
2. `capture-run`
3. `fingerprint-env`
4. `emit-bundle`
5. `emit-diff-seed`
6. `validate-handoff`

## 3. W006 stable path
1. The first stable Windows execution path is `capture-run`.
2. Initial invocation contract:
   - `dna-xl-play capture-run --scenario <scenario-json-path> [--output-dir <repo-relative-output-dir>]`
3. `capture-run` invokes the declared repo-local PowerShell bridge at `scripts/invoke-excel-observation.ps1`.
4. The initial stable output family emits `capture.json`, `provenance.json`, `bridge.json`, `environment.json`, `driver-run.json`, and `bundle.json`.
5. The current `W007` alignment pass also emits:
   - `oxreplay-manifest.json`
   - `views/normalized-replay.json`
6. These replay-facing files are emitted as canonical shared-runtime intake artifacts over the richer `OxXlPlay` observation bundle rather than replacing it.
7. Other command families remain scaffolded until later worksets advance them beyond planning.

## 4. W008 widened capture-run behavior
1. `capture-run` now honors `requested_observation_scope.oxxlplay_required_surfaces` when that consumer-side hint is present in the scenario payload.
2. The first widened consumer path is the SpreadsheetML 2003 XML-backed verification scenario family used by `DNA OneCalc`.
3. `views/normalized-replay.json` may now carry machine-readable `comparison_views` and replay-facing `source_metadata` when the declared observation surfaces are sufficient to populate them honestly.
4. The current widened output family may additionally emit:
   - `views/visible-value.json`
   - `views/effective-display-text.json`
   - `views/formatting-view.json`
   - `views/conditional-formatting-view.json`
5. These widened replay-facing artifacts remain support projections and must not be mistaken for a complete semantic-equivalence contract on their own.
