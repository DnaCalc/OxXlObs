# W008_SPREADSHEETML_VERIFICATION_OBSERVATION_EXPANSION

## Objective
Stand up the first retained SpreadsheetML 2003 observation family for downstream XML-backed verification consumers, starting with `DNA OneCalc`.

## Scope
1. admit SpreadsheetML-targeted observation surfaces for display, formatting, and conditional-formatting comparison support,
2. widen `capture-run` so consumer scenarios can request those surfaces through `requested_observation_scope.oxxlplay_required_surfaces`,
3. retain a tracked SpreadsheetML workbook scenario and live Excel-driven outputs under `states/excel/`,
4. emit machine-readable replay-facing `comparison_views` plus replay-adjacent sidecar views for `visible_value`, `effective_display_text`, `formatting_view`, and `conditional_formatting_view`,
5. retain an outbound handoff packet describing delivered scope and current projection limits for `DNA OneCalc` and `OxReplay`.

## Dependencies
1. `W006_STABLE_WINDOWS_EXECUTION_DRIVER`
2. `W007_FIRST_CROSS_REPO_REPLAY_AND_DIFF_CONSUMPTION`
3. local Windows Excel COM automation availability
4. consumer SpreadsheetML handoff evidence from `DnaOneCalc`

## Exit Gate
1. a retained SpreadsheetML scenario exists under `docs/test-corpus/excel/`,
2. `capture-run` emits retained live outputs for that scenario under `states/excel/`,
3. the DnaOneCalc-generated XML scenario shape can be widened through `requested_observation_scope.oxxlplay_required_surfaces` without repo-local scenario surgery in the sibling repo,
4. current direct-versus-derived status and projection limits are documented in the OneCalc consumer contract and outbound handoff.

## Expected capability impact
1. establishes `O6.spreadsheetml_observation_valid` over the first XML-backed display/formatting observation family.

## Expected pack impact
1. widened exercised support for `PACK.trace.forensic_plane`
2. richer replay-adjacent view emission for `PACK.replay.appliance`
3. richer comparison-ready observation output for `PACK.diff.cross_engine.continuous`

## Environment Preconditions
1. Windows host with Excel COM automation
2. PowerShell 7 available as `pwsh`
3. local sibling repo `DnaOneCalc` present for consumer-shape verification

## Evidence Layout
1. tracked scenario root: `docs/test-corpus/excel/xlplay_capture_spreadsheetml_formatting_001/`
2. tracked live output root: `states/excel/xlplay_capture_spreadsheetml_formatting_001/`
3. local scratch consumer check: `.tmp/onecalc-xml-probe/`
4. retained run summary and outbound handoff: `docs/test-runs/`, `docs/handoffs/`

## Scenario Readiness
1. scenario id: `xlplay_capture_spreadsheetml_formatting_001`
2. workbook kind: `SpreadsheetML 2003`
3. target locator: `Input!A1`
4. current admitted widened surfaces:
   - `cell_value`
   - `formula_text`
   - `effective_display_text`
   - `number_format_code`
   - `style_id`
   - `font_color`
   - `fill_color`
   - `conditional_formatting_rules`
   - `conditional_formatting_effective_style`

## Pack-Evidence Traceability
1. `PACK.trace.forensic_plane`
   - retained direct and derived surface evidence in `capture.json`, `provenance.json`, and `bridge.json`
2. `PACK.replay.appliance`
   - replay-facing manifest plus `comparison_views`/`source_metadata` in `views/normalized-replay.json` and sidecar view families under `views/`
3. `PACK.diff.cross_engine.continuous`
   - richer comparison-ready source evidence for XML-backed consumer differential workflows
