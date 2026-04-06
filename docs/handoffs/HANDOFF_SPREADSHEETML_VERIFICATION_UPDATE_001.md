*Posted by Codex agent on behalf of @govert*

# SpreadsheetML Verification Update 001

## Purpose
Respond to the DnaOneCalc SpreadsheetML 2003 verification handoff with the first retained OxXlPlay observation expansion for XML-backed workbook cases.

## Delivered scope
1. OxXlPlay now retains a tracked SpreadsheetML scenario family at `docs/test-corpus/excel/xlplay_capture_spreadsheetml_formatting_001/`.
2. The live retained output root is `states/excel/xlplay_capture_spreadsheetml_formatting_001/`.
3. `capture-run` now separates:
   - `cell_value` from `Range.Value2`
   - `effective_display_text` from `Range.Text`
4. For SpreadsheetML-backed scenarios, OxXlPlay now retains these widened surfaces:
   - `formula_text`
   - `cell_value`
   - `effective_display_text`
   - `number_format_code`
   - `style_id`
   - `font_color`
   - `fill_color`
   - `conditional_formatting_rules`
   - `conditional_formatting_effective_style`
5. SpreadsheetML-derived formatting projection now resolves inherited style properties through parent-style chains before emitting `number_format_code`, `font_color`, and `fill_color`.
6. The tracked retained workbook now exercises that path with `calc -> calcBase` style inheritance.
7. Consumer-generated scenarios that include `requested_observation_scope.oxxlplay_required_surfaces` now widen automatically at runtime without requiring sibling-repo scenario-file surgery.
8. `views/normalized-replay.json` now carries machine-readable `comparison_views` entries for:
   - `visible_value`
   - `effective_display_text`
   - `formatting_view`
   - `conditional_formatting_view`
9. The same replay-facing artifact now carries `source_metadata` preserving projection status, capture-loss summary, interpretation limits, workbook identity, and scenario traceability.
10. Replay-adjacent view files are now emitted for:
   - `visible_value`
   - `effective_display_text`
   - `formatting_view`
   - `conditional_formatting_view`

## Retained evidence
1. Tracked scenario input:
   - `docs/test-corpus/excel/xlplay_capture_spreadsheetml_formatting_001/scenario.json`
   - `docs/test-corpus/excel/xlplay_capture_spreadsheetml_formatting_001/workbook.xml`
2. Tracked live output:
   - `states/excel/xlplay_capture_spreadsheetml_formatting_001/capture.json`
   - `states/excel/xlplay_capture_spreadsheetml_formatting_001/bundle.json`
   - `states/excel/xlplay_capture_spreadsheetml_formatting_001/oxreplay-manifest.json`
   - `states/excel/xlplay_capture_spreadsheetml_formatting_001/views/normalized-replay.json`
   - `states/excel/xlplay_capture_spreadsheetml_formatting_001/views/visible-value.json`
   - `states/excel/xlplay_capture_spreadsheetml_formatting_001/views/effective-display-text.json`
   - `states/excel/xlplay_capture_spreadsheetml_formatting_001/views/formatting-view.json`
   - `states/excel/xlplay_capture_spreadsheetml_formatting_001/views/conditional-formatting-view.json`
3. Consumer-shape local verification against a real DnaOneCalc-generated XML scenario:
   - `.tmp/onecalc-xml-probe/capture.json`
   - `.tmp/onecalc-xml-probe/oxreplay-manifest.json`
   - `.tmp/onecalc-xml-probe/views/normalized-replay.json`
   - `.tmp/onecalc-xml-probe/views/`
4. OxReplay-side validate/ingest check:
   - `cargo run -p oxreplay-dnarecalc-cli -- validate-bundle --bundle ../OxXlPlay/states/excel/xlplay_capture_spreadsheetml_formatting_001/oxreplay-manifest.json --format json`
   - current result: `valid`, with the widened view family list indexed successfully
5. OxReplay-side replay/diff/explain checks over the widened replay artifact:
   - `cargo run -p oxreplay-dnarecalc-cli -- replay --bundle ../OxXlPlay/states/excel/xlplay_capture_spreadsheetml_formatting_001/views/normalized-replay.json --kind normalized-replay`
   - `cargo run -p oxreplay-dnarecalc-cli -- diff --left docs/test-corpus/bundles/crosslane_xml_view_family_gap_001/left.replay.json --left-kind normalized-replay --right ../OxXlPlay/states/excel/xlplay_capture_spreadsheetml_formatting_001/views/normalized-replay.json --right-kind normalized-replay`
   - `cargo run -p oxreplay-dnarecalc-cli -- explain --left docs/test-corpus/bundles/crosslane_xml_view_family_gap_001/left.replay.json --left-kind normalized-replay --right ../OxXlPlay/states/excel/xlplay_capture_spreadsheetml_formatting_001/views/normalized-replay.json --right-kind normalized-replay`
   - current result: `replay` loads the declared `comparison_views` and `source_metadata`; `diff` and `explain` classify `effective_display_text` divergence plus coverage gaps for `formatting_view` and `conditional_formatting_view` against the retained left baseline fixture

## Direct versus derived status
1. `cell_value`, `formula_text`, and `effective_display_text` are retained as `direct` in the current SpreadsheetML family.
2. `number_format_code`, `style_id`, `font_color`, `fill_color`, and `conditional_formatting_rules` are currently retained as `derived` for SpreadsheetML import cases, because Excel XML import on the exercised host does not preserve all source identifiers and carriers verbatim through COM.
3. The current retained SpreadsheetML fixture proves that derived formatting payloads survive parent-style inheritance rather than requiring every exercised style property to be declared on the leaf cell style.
4. `conditional_formatting_effective_style` is currently `derived` for the admitted SpreadsheetML expression-rule subset by combining source rule payloads with Excel formula evaluation on the target cell.
5. The capture and bridge artifacts retain explicit interpretation limits:
   - `effective_display_host_rendered`
   - `spreadsheet_ml_source_projection`
   - `conditional_formatting_rule_projection`
6. `comparison_views.conditional_formatting_view` currently publishes both source-backed rule payloads and the derived effective-style consequence for the admitted expression-rule subset.

## Current open lanes
1. Broad direct COM-preserved style-id and conditional-formatting carrier capture for SpreadsheetML import remains open.
2. `conditional_formatting_effective_style` is currently scoped to the admitted expression-rule subset; broader CF rule-kind coverage remains open.
3. Broader retained cross-lane diff and explain exercise through `OxReplay` remains a sibling-repo coordination lane, even though the local replay-facing seam now publishes declared comparison families directly.
4. The W007 `OxCalc` comparison leg remains open.

## Capability and pack impact
1. Capability impact: establishes `O6.spreadsheetml_observation_valid` as an in-progress local capability lane.
2. Pack impact:
   - widened exercised support for `PACK.trace.forensic_plane`
   - richer replay-adjacent output for `PACK.replay.appliance`
   - richer comparison-ready observation output for `PACK.diff.cross_engine.continuous`

## Request to downstream consumers
1. DnaOneCalc may now consume the widened SpreadsheetML capture family from OxXlPlay.
2. Downstream consumers must preserve the retained `status`, `capture_loss`, `uncertainty`, and interpretation-limit qualifiers.
3. Derived SpreadsheetML formatting and conditional-formatting payloads must not be relabeled as direct Excel observation.
