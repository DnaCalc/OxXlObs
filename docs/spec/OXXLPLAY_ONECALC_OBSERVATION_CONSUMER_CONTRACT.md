# OXXLPLAY_ONECALC_OBSERVATION_CONSUMER_CONTRACT.md

## 1. Position
This document defines the observation-consumer contract between `OxXlPlay` and `DNA OneCalc`.

It is the canonical `OxXlPlay`-side reference for how `DNA OneCalc` should consume, interpret, label, and present `OxXlPlay` observation evidence in product surfaces, comparison workflows, and retained artifacts.

It does not define replay doctrine, comparison semantics, or product UI behavior. Those remain owned by `OxReplay` and `DNA OneCalc` respectively.

## 2. Ownership Rule
1. `OxXlPlay` owns observation evidence, retained provenance, capture-loss metadata, and replay-ready bundle emission.
2. `OxXlPlay` does not own replay doctrine, comparison judgment, witness lifecycle governance, or Excel semantic truth.
3. `DNA OneCalc` consumes `OxXlPlay` evidence through `OxReplay` replay-manifest intake and through direct retained-artifact access for provenance and lossiness inspection.
4. `DNA OneCalc` must not locally reinterpret `OxXlPlay` capture-loss markers or lossiness declarations.

## 3. First Comparison-Ready Observation Family
The first honest comparison-ready observation family is:

| Field | Value |
|---|---|
| Scenario id | `xlplay_capture_values_formulae_001` |
| Replay class | `capture_surface_basic` |
| Capability floor | `O5.stable_driver_valid` |
| Workset | `W006` (live driver), `W007` (replay-manifest alignment) |
| Retained root | `states/excel/xlplay_capture_values_formulae_001/` |
| Platform | Windows-only (live Excel COM automation) |
| Bridge | `external_process` / `w006-powershell-com.v1` |

### 3.1 What This Family Provides
1. Directly observed cell values for declared surfaces in the scenario manifest.
2. Directly observed formula text where accessible through the COM bridge.
3. Retained provenance including Excel build, host OS, bridge identity, and macro mode.
4. Retained capture-loss markers for every observed surface.
5. A replay-ready bundle with repo-relative sidecar refs.
6. An `OxReplay`-canonical replay manifest (`oxreplay-manifest.json`).
7. A normalized replay view (`views/normalized-replay.json`).

### 3.2 What This Family Does Not Provide
1. Broad scenario coverage beyond a single workbook and surface set.
2. Formatting, conditional-formatting, or display-state observation.
3. Multi-sheet or cross-reference observation.
4. Error-surface or diagnostic-surface observation beyond the declared baseline.
5. Semantic equivalence claims against DNA lane outputs.

## 4. Observation Surface Classification
Every `OxXlPlay` captured surface carries a `status` field. `DNA OneCalc` must preserve and surface this classification.

| Status | Meaning | Comparison use |
|---|---|---|
| `direct` | The surface value was directly observed from Excel through the declared bridge. | Eligible for direct value comparison. |
| `derived` | The surface value was computed or inferred from other direct observations. | Must be labeled `derived` in any comparison surface. |
| `unavailable` | The surface could not be observed. The capture-loss marker explains why. | Must not be used as comparison input. Must be shown as unavailable with the capture-loss reason. |

### 4.1 Capture-Loss And Uncertainty Fields
1. Every captured surface carries an explicit `capture_loss` field, even when the value is `none`.
2. Every captured surface carries an explicit `uncertainty` field, even when the value is `none`.
3. `DNA OneCalc` must surface non-`none` capture-loss markers in comparison UIs and retained comparison artifacts.
4. An `unavailable` surface always carries a non-`none` capture-loss marker.
5. A `direct` or `derived` surface may also carry a non-`none` capture-loss marker when observation fidelity is reduced but a value was still obtained.

### 4.2 Bridge Influence
1. If the bridge constrains interpretation, the capture output declares `bridge_influenced: true`.
2. When `bridge_influenced` is true, at least one interpretation limit is retained in the bridge provenance.
3. `DNA OneCalc` should treat bridge-influenced surfaces as carrying an implicit provisionality qualifier in comparison contexts.

## 5. First Comparison Envelope
The comparison envelope is the set of surfaces from a single observation family that `DNA OneCalc` may present as comparison-eligible.

### 5.1 Current Comparison Envelope For `xlplay_capture_values_formulae_001`

| Surface kind | Status | Comparison eligibility | Notes |
|---|---|---|---|
| Cell value (`sheet1_a1_value`) | `direct` | Eligible for direct value comparison | Observed through COM bridge |
| Formula text (`sheet1_a1_formula`) | `direct` | Eligible for direct formula-text comparison | Observed through COM bridge |

### 5.2 Surfaces Not In The Current Envelope
The following surface kinds are not yet observed or retained by the current family and must not be presented as available for comparison:
1. Formatting and number-format surfaces.
2. Conditional-formatting surfaces.
3. Error and diagnostic surfaces beyond direct cell-value observation.
4. Display-state and effective-display surfaces.
5. Multi-cell range or structured-reference surfaces.
6. Named-range or name-manager surfaces.

### 5.3 Envelope Expansion Rule
When future observation families add surfaces, the comparison envelope for that family expands. `DNA OneCalc` should derive comparison eligibility from the retained `capture.json` surface classification rather than from a hardcoded surface list.

## 6. Lossy Replay-Facing Normalized Views
The `OxReplay`-facing normalized replay view at `views/normalized-replay.json` is a projection over the richer observation bundle.

### 6.1 Labeling Rule
1. The normalized replay view is explicitly `lossy`. It does not preserve the full observation fidelity of the source bundle.
2. The canonical replay manifest (`oxreplay-manifest.json`) must declare the projection status. If the replay-facing view is only a partial or lossy projection, the manifest must state that explicitly.
3. The richer `OxXlPlay` observation bundle and its sidecars remain retained alongside the normalized view and must not be discarded.

### 6.2 Interpretation Rule For DNA OneCalc
1. `DNA OneCalc` must not present the normalized replay view as a complete semantic equivalence surface.
2. When showing comparison results derived from the normalized replay view, `DNA OneCalc` must label them as `lossy` in comparison-reliability badges.
3. For richer comparison fidelity, `DNA OneCalc` should consult the source observation bundle and its provenance sidecars rather than relying solely on the normalized view.
4. The long-term direction is for the normalized view to gain fidelity. The current `lossy` label reflects the present floor, not a permanent design choice.

## 7. Platform Scope: Windows-Only Live Capture
1. Live Excel observation and capture through `OxXlPlay` is Windows-only.
2. The current bridge (`w006-powershell-com.v1`) requires Windows, PowerShell, and a locally installed Excel instance with COM automation available.
3. `DNA OneCalc` on non-Windows platforms must not imply live Excel comparison availability.
4. Retained observation artifacts (bundles, manifests, normalized views, sidecars) are platform-neutral JSON and may be consumed on any platform.
5. The distinction is: live capture is Windows-only; retained evidence consumption is cross-platform.

## 8. Authoritative Doc Set For DNA OneCalc
`DNA OneCalc` should use the following `OxXlPlay` documents as its authoritative reference set for observation evidence consumption:

| Document | Role |
|---|---|
| `docs/spec/OXXLPLAY_ONECALC_OBSERVATION_CONSUMER_CONTRACT.md` | This document. Canonical consumer contract. |
| `docs/spec/OXXLPLAY_SCOPE_AND_BOUNDARY.md` | Observation-vs-replay ownership split. |
| `docs/spec/OXXLPLAY_ARCHITECTURE_AND_CAPTURE_MODEL.md` | Observable surfaces, capture shapes, and live driver baseline. |
| `docs/spec/OXXLPLAY_ENVIRONMENT_AND_PROVENANCE_MODEL.md` | Provenance and bridge contract. |
| `docs/spec/OXXLPLAY_BUNDLE_EMISSION_AND_HANDOFF_MODEL.md` | Bundle and replay-manifest emission contract. |
| `docs/spec/OXXLPLAY_CAPABILITY_AND_PACK_TRACEABILITY.md` | Local capability ladder (`O0`..`O5`). |
| `docs/spec/OXXLPLAY_SCENARIO_REGISTER.md` | Stable scenario ids and retained-root map. |
| `docs/IN_PROGRESS_FEATURE_WORKLIST.md` | Current workset status and consumer readiness. |
| `docs/test-runs/W006_STABLE_WINDOWS_EXECUTION_DRIVER.md` | Best evidence for what is exercised live. |
| `docs/test-runs/W007_FIRST_CROSS_REPO_REPLAY_AND_DIFF_CONSUMPTION.md` | Best evidence for replay-facing readiness and limits. |

## 9. Current Gaps And Honest Limits
1. Only one observation family (`xlplay_capture_values_formulae_001`) has been exercised live with retained evidence.
2. The comparison envelope is narrow: cell value and formula text for a single workbook.
3. No formatting, conditional-formatting, display-state, or error-surface observations exist yet.
4. The `OxCalc` comparison leg of `W007` remains open. Broad differential comparison against DNA lane outputs is not yet exercised.
5. The normalized replay view is explicitly lossy.
6. Adapter-manifest expectations between `OxXlPlay` and `OxReplay` still have open clarification items.
7. The `richer OxXlPlay diff or equality envelope` noted in the `DNA OneCalc` spec (Section 17.2) remains provisional scope, not current capability.
