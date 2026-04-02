# AGENTS.md — OxXlPlay Agent Instructions

## 1. Context Loading Order

On session start, read in this order:
1. `README.md`
2. `CHARTER.md`
3. `OPERATIONS.md`
4. `CURRENT_BLOCKERS.md`
5. `docs/IN_PROGRESS_FEATURE_WORKLIST.md`
6. `docs/worksets/README.md`
7. `docs/spec/README.md`
8. `docs/spec/OXXLPLAY_SCOPE_AND_BOUNDARY.md`
9. `docs/spec/OXXLPLAY_ARCHITECTURE_AND_CAPTURE_MODEL.md`
10. `docs/spec/OXXLPLAY_ENVIRONMENT_AND_PROVENANCE_MODEL.md`
11. `docs/spec/OXXLPLAY_BUNDLE_EMISSION_AND_HANDOFF_MODEL.md`
12. `docs/spec/OXXLPLAY_IMPLEMENTATION_BASELINE.md`
13. `docs/spec/OXXLPLAY_SCENARIO_REGISTER.md`
14. `docs/spec/OXXLPLAY_CAPABILITY_AND_PACK_TRACEABILITY.md`
15. `docs/spec/OXXLPLAY_CLI_CONTRACT.md`
16. Inbound observation ledgers from sibling repos, if present:
   - `../OxReplay/docs/upstream/NOTES_FOR_OXXLPLAY.md`
   - `../OxCalc/docs/upstream/NOTES_FOR_OXXLPLAY.md`
   - `../OxFml/docs/upstream/NOTES_FOR_OXXLPLAY.md`
   - `../OxVba/docs/upstream/NOTES_FOR_OXXLPLAY.md`
17. Foundation doctrine docs (`../Foundation/CHARTER.md`, `../Foundation/ARCHITECTURE_AND_REQUIREMENTS.md`, `../Foundation/OPERATIONS.md`, `../Foundation/REPLAY_APPLIANCE.md`)

## 2. Source-of-Truth Precedence

When guidance conflicts, precedence is:
1. `../Foundation/CHARTER.md`
2. `../Foundation/ARCHITECTURE_AND_REQUIREMENTS.md`
3. `../Foundation/OPERATIONS.md`
4. `../Foundation/REPLAY_APPLIANCE.md`
5. this repo `CHARTER.md`
6. this repo `OPERATIONS.md`
7. this repo `docs/spec/*`

## 3. Anti-Premature-Completion Doctrine

### Rule 1: Restricted Completion Language
The words "implemented", "closed", "done", and "complete" are forbidden when describing:
- partial subsets of declared scope,
- scaffolding, stubs, or compile-only code,
- host shells without exercised retained evidence,
- spec text without replay-ready observation evidence.

Use "in-progress", "partial", or "scaffolded" instead.

### Rule 2: Self-Audit Required Before Completion Claims
Before ANY completion claim, the agent must:
1. Run the Pre-Closure Verification Checklist from `OPERATIONS.md` Section 7.
2. Run the Completion Claim Self-Audit from `OPERATIONS.md` Section 9.
3. Include the checklist and self-audit results in the completion report.

### Rule 3: Three-Axis Reporting Mandatory
Every status report must include:
- `scope_completeness` (`scope_complete` | `scope_partial`)
- `target_completeness` (`target_complete` | `target_partial`)
- `integration_completeness` (`integrated` | `partial`)
- explicit `open_lanes` list when any axis is partial

### Rule 4: Scaffolding Is Not Implementation
Stubs, empty bridge wrappers, compile-only code, and placeholder harnesses are scaffolding.
Scaffolding is never reported as implementation. Report it as `scaffolded`.

### Rule 5: Spec Text Without Evidence Is Not Done
Spec or contract text without at least one deterministic retained observation artifact proving intended behavior is not done. Report it as `spec_drafted`.

### Rule 6: Clean-Room Admissibility Is Mandatory
If a proposed harness flow depends on non-admissible evidence or hidden reverse engineering, it is a doctrine failure.

### Rule 7: Default to In-Progress
When uncertain whether work meets completion criteria, report `in_progress`.

### Rule 8: Observation Is Not Semantic Authority
If a proposed convenience change causes OxXlPlay to infer or assert semantic meaning beyond retained black-box evidence, the change is not complete.

## 4. Continuation Behavior

Mode: **checkpoint-at-gates** (conservative).

1. Agent must pause and report status at each workset gate boundary.
2. AutoRun is disabled by default.
3. AutoRun may only be enabled when explicitly requested by the user for a specific declared scope.
4. Between gates, the agent may proceed autonomously within the declared workset scope.

## 5. Blocker Handling

When a blocker is encountered:
1. Create or update `CURRENT_BLOCKERS.md` with a structured `BLK-XLOBS-NNN` entry.
2. Continue with other non-blocked work within scope.
3. If all paths are blocked, emit a structured summary with blocker ids, state, exact unblock steps, and recommendation.

## 6. Public Attribution Doctrine

For any external/public-facing message authored by an agent, the first line must be:

*Posted by [Agent] agent on behalf of @govert*

## 7. Change Discipline

1. Keep changes minimal, explicit, and testable.
2. Keep OxXlPlay from absorbing semantic ownership or replay-governance ownership.
3. Treat Excel-driving seams as observation boundaries, not truth sources.
4. Place new local policy in the most specific spec file practical.
5. Require explicit capability and pack impact notes for changes touching replay-handoff surfaces.
6. When proposing changes that affect shared replay doctrine, route the normative delta back through Foundation rather than silently reassigning authority locally.
