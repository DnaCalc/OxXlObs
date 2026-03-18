# OXXLOBS_CLI_CONTRACT.md

## 1. Position
This document defines the initial command families for the OxXlObs CLI.

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
   - `dna-xl-obs capture-run --scenario <scenario-json-path> [--output-dir <repo-relative-output-dir>]`
3. `capture-run` invokes the declared repo-local PowerShell bridge at `scripts/invoke-excel-observation.ps1`.
4. The initial stable output family emits `capture.json`, `provenance.json`, `bridge.json`, `environment.json`, `driver-run.json`, and `bundle.json`.
5. Other command families remain scaffolded until later worksets advance them beyond planning.
