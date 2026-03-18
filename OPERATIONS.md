# OPERATIONS.md — OxXlObs Operations

## 1. Purpose
Define day-to-day execution rules for the Excel observation harness and its replay-ready evidence outputs.

## 2. Operating Principles
1. Observation artifacts must preserve provenance, uncertainty, and capture-loss explicitly.
2. Every retained run should move toward replay-ready evidence rather than one-off diagnostics.
3. Excel automation is subordinate to evidence quality; fast capture that cannot be trusted is a failure.
4. Clean-room admissibility rules are first-class runtime concerns, not later documentation chores.
5. Shared replay governance remains outside this repo; OxXlObs feeds the replay plane but does not own it.

## 3. Working Strata
Initial implementation strata are:
1. `Abstractions`
2. `Scenario`
3. `Capture`
4. `Provenance`
5. `Bundle`
6. `Bridge`
7. `CLI`

## 4. Required Packs (baseline)
1. `PACK.replay.appliance`
2. `PACK.diff.cross_engine.continuous`
3. `PACK.trace.forensic_plane`

## 5. Cross-Repo Handoff Rule
When observation changes affect replay bundle contracts, lane comparison expectations, or Foundation doctrine:
1. confirm the change is observational/mechanical rather than semantic,
2. identify affected capability and pack surfaces,
3. record required migration or fallback behavior,
4. route doctrine-affecting changes back to Foundation through managed-run promotion packets,
5. file handoff packets to `OxReplay` or affected lane repos when evidence-shape obligations change.

## 6. Promotion Gate
No OxXlObs promotion without:
1. updated spec text for affected strata,
2. declared replay-handoff, capability, and pack impact,
3. explicit clean-room admissibility check,
4. retained artifact paths for changed behavior,
5. explicit check that no semantic authority drift was introduced.

## 7. Pre-Closure Verification Checklist

Before claiming any workset or feature item as complete, answer each item yes or no.
All items must be "yes" for a completion claim. Any "no" means the item is `in_progress`.

| # | Check | Yes/No |
|---|-------|--------|
| 1 | Spec text updated for all in-scope items? | |
| 2 | Capture-loss, provenance, and replay-handoff impacts stated? | |
| 3 | At least one deterministic retained observation artifact exists per in-scope runtime behavior? | |
| 4 | Replay-ready bundle or handoff evidence updated where observation output changed? | |
| 5 | Cross-repo impact assessed and handoff filed if needed? | |
| 6 | Pack impact stated for affected packs? | |
| 7 | All required tests pass? | |
| 8 | No semantic-ownership drift remains in declared scope? | |
| 9 | `docs/IN_PROGRESS_FEATURE_WORKLIST.md` updated? | |
| 10 | `CURRENT_BLOCKERS.md` updated (new/resolved)? | |

## 8. Expanded Definition of Done

A workset or feature item is done for its declared scope only when all of the following hold:
1. all in-scope observation, provenance, and replay-handoff text is updated,
2. affected provenance and admissibility rules are stated and exercised,
3. at least one deterministic retained observation artifact exists per in-scope behavior,
4. changed replay-ready bundle output is backed by retained evidence or explicitly marked unsupported,
5. affected `OxReplay` or lane assumptions are assessed,
6. pack impact is updated,
7. no semantic or replay-governance ownership drift exists,
8. the completion report includes the required three-axis status.

## 9. Completion Claim Self-Audit

Before submitting a completion claim, the agent must perform this self-audit and include the results.

### Step 1: Scope Re-Read
Re-read the workset scope declaration. Any missing in-scope item = `in_progress`.

### Step 2: Gate Criteria Re-Read
Re-read the workset gate criteria. Any unmet criterion = gate open.

### Step 3: Silent Scope Reduction Check
Compare the original scope declaration with what was actually delivered. Any unreported narrowing is a doctrine violation.

### Step 4: "Looks Done But Is Not" Pattern Check
Check for these patterns:
- compile-only package splits reported as real observation support,
- bridge wrappers reported as trustworthy capture without provenance or lossiness,
- replay-ready bundle claims without retained artifacts,
- empirical outputs that cannot be reproduced from tracked inputs and declared environment assumptions.

### Step 5: Include Result
Include the self-audit result in the completion report with explicit pass/fail for each step.

## 10. Report-Back Completeness Contract

Every completion report must include:
1. `execution_state`: `planned` | `in_progress` | `blocked` | `complete`
2. `scope_completeness`: `scope_complete` | `scope_partial`
3. `target_completeness`: `target_complete` | `target_partial`
4. `integration_completeness`: `integrated` | `partial`
5. `open_lanes`: explicit list when any completeness axis is partial
6. `capability_impact`: `none` or a short explicit statement
7. `pack_impact`: `none` or a short explicit statement

## 11. Carried-Forward Operating Lessons

### Lesson 1: Observation Without Provenance Is Not Evidence
If Excel build, workbook fingerprint, trigger recipe, or bridge version is missing, the artifact is not replay-grade evidence.

### Lesson 2: Capture Loss Must Be Explicit
If a surface is unavailable, unstable, sampled, inferred, or post-processed, the artifact must state that explicitly.

### Lesson 3: Retained Scenarios Come Before Broad Automation
Do not widen the driver surface before scenario ids, observable surfaces, and naming rules are stable enough to retain.

### Lesson 4: Bridge Seams Must Stay Narrow
Any non-Rust or Windows-specific bridge must be treated as an explicit seam, not as silent ambient dependency.

### Lesson 5: Raw Dumps Are Not The Product
The repo exists to emit replay-ready evidence bundles, not to accumulate unstructured Excel logs.

## 12. Upstream Observation Ledger Protocol

### 12.1 Purpose
Repos that consume Excel-backed evidence discover constraints on observation shape, bundle quality, and capture coverage through their own work. Those observations must flow back to OxXlObs through a structured channel.

### 12.2 Inbound Observation Sources
OxXlObs must check for inbound observation ledgers from sibling repos at the start of any adapter, replay-handoff, or differential workset. Known source locations are:

| Source repo | Ledger location | Relationship |
|-------------|----------------|--------------|
| OxReplay | `../OxReplay/docs/upstream/NOTES_FOR_OXXLOBS.md` | replay bundle and witness-seed expectations |
| OxCalc | `../OxCalc/docs/upstream/NOTES_FOR_OXXLOBS.md` | core-engine differential expectations |
| OxFml | `../OxFml/docs/upstream/NOTES_FOR_OXXLOBS.md` | formula/evaluator seam expectations |
| OxVba | `../OxVba/docs/upstream/NOTES_FOR_OXXLOBS.md` | Excel/VBA host-observation expectations |

## 13. Emitted Artifact Protocol
1. Declare a canonical artifact root before implementation begins.
2. Tracked artifacts must use repo-relative paths only.
3. Validation runs must not mutate tracked evidence in place.
4. Initial retained roots should prefer:
   - `docs/test-corpus/excel/`
   - `docs/test-corpus/bundles/`
   - `docs/test-runs/`
   - `states/excel/`

## 14. Execution Packet Minimums
Any workset that acts as an execution packet must include:
1. environment preconditions,
2. evidence layout,
3. scenario readiness,
4. pack-evidence traceability.

## 15. Local Doctrine Reference
OxXlObs-local execution lessons live at `docs/LOCAL_EXECUTION_DOCTRINE.md`.
