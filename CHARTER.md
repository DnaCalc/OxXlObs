# CHARTER.md — OxXlPlay Charter

## 1. Mission
OxXlPlay defines, implements, and proves the Excel observation harness for DNA Calc.

It owns the reusable observation substrate for scenario planning, Excel-run fingerprinting, observable-surface capture, lossiness reporting, replay-ready bundle emission, and differential witness seeding while preserving Foundation and lane ownership of semantics and replay governance.

## 2. Precedence
When guidance conflicts, precedence is:
1. `../Foundation/CHARTER.md`
2. `../Foundation/ARCHITECTURE_AND_REQUIREMENTS.md`
3. `../Foundation/OPERATIONS.md`
4. `../Foundation/REPLAY_APPLIANCE.md`
5. this `CHARTER.md`
6. this repo `OPERATIONS.md`

## 3. Scope
In scope:
1. Scenario declarations for reproducible Excel observation runs.
2. Workbook, environment, Excel-build, and trigger fingerprint capture.
3. Stable capture of observable values, formulas, diagnostics, and other declared surfaces.
4. Lossiness and uncertainty labeling for every retained observation artifact.
5. Compilation of canonical replay-ready bundles and bundle sidecars for `OxReplay`.
6. Differential witness seeding for Excel-vs-DNA investigation.

Out of scope:
1. Semantic ownership of Excel behavior.
2. Replay doctrine, lifecycle governance, or pack policy authority.
3. Lane-local semantic interpretation for `OxFunc`, `OxFml`, `OxCalc`, or `OxVba`.
4. Generic Office automation unrelated to observation-to-replay evidence flow.

## 4. Ownership boundary rule
1. OxXlPlay may observe and record Excel behavior through declared clean-room harnesses.
2. OxXlPlay may normalize and package observations for replay use.
3. OxXlPlay may not redefine Excel semantics or assert semantic truth beyond retained observation evidence.
4. Foundation owns doctrine; `OxReplay` owns shared replay runtime; lane repos own DNA Calc semantics.

## 5. Observation-to-replay rule
1. The primary output of OxXlPlay is not raw logs or pass/fail status.
2. The primary output is a replay-ready evidence bundle with explicit provenance and capture-loss metadata.
3. Every retained divergence should be shaped so it can later be replayed, diffed, explained, and, where suitable, distilled.

## 6. Clean-room rule
Allowed sources:
1. public specifications and documentation,
2. published research,
3. reproducible black-box observations of Excel behavior.

Disallowed sources:
1. proprietary or restricted sources,
2. reverse engineering of Excel internals,
3. decompilation/disassembly of Excel internals.

## 7. Definition of done
An observation-harness change is done only when:
1. repo-local spec text is updated,
2. relevant Foundation doctrine links still hold,
3. capability and pack impact are stated,
4. affected replay-handoff evidence is updated,
5. the change does not widen OxXlPlay into semantic ownership or replay-governance ownership.
