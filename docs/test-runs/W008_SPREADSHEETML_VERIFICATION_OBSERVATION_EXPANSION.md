# W008_SPREADSHEETML_VERIFICATION_OBSERVATION_EXPANSION

- Date: 2026-04-06
- Execution state: `in_progress`
- Scope status: the first SpreadsheetML 2003 observation family is retained and exercised locally; downstream OxReplay intake and broader consumer rollout remain open.

## Commands
1. `cargo run -p oxxlplay-cli -- capture-run --scenario docs/test-corpus/excel/xlplay_capture_spreadsheetml_formatting_001/scenario.json --output-dir states/excel/xlplay_capture_spreadsheetml_formatting_001`
2. `cargo run -p oxxlplay-cli -- capture-run --scenario ..\DnaOneCalc\target\onecalc-verification\manual-xml-case-1\cases\xml-case-1\scenario.json --output-dir .tmp/onecalc-xml-probe`
3. `cargo run -p oxreplay-dnarecalc-cli -- validate-bundle --bundle ../OxXlPlay/states/excel/xlplay_capture_spreadsheetml_formatting_001/oxreplay-manifest.json --format json`
4. `cargo run -p oxreplay-dnarecalc-cli -- replay --bundle ../OxXlPlay/states/excel/xlplay_capture_spreadsheetml_formatting_001/views/normalized-replay.json --kind normalized-replay`
5. `cargo run -p oxreplay-dnarecalc-cli -- diff --left docs/test-corpus/bundles/crosslane_xml_view_family_gap_001/left.replay.json --left-kind normalized-replay --right ../OxXlPlay/states/excel/xlplay_capture_spreadsheetml_formatting_001/views/normalized-replay.json --right-kind normalized-replay`
6. `cargo run -p oxreplay-dnarecalc-cli -- explain --left docs/test-corpus/bundles/crosslane_xml_view_family_gap_001/left.replay.json --left-kind normalized-replay --right ../OxXlPlay/states/excel/xlplay_capture_spreadsheetml_formatting_001/views/normalized-replay.json --right-kind normalized-replay`

## Retained roots exercised
1. `docs/test-corpus/excel/xlplay_capture_spreadsheetml_formatting_001/`
2. `states/excel/xlplay_capture_spreadsheetml_formatting_001/`
3. `.tmp/onecalc-xml-probe/`

## Current W008 baseline
1. `capture-run` now separates `cell_value` from `effective_display_text`; the former is captured from `Value2`, the latter from host-rendered `Text`.
2. SpreadsheetML style, number-format, and conditional-formatting rule payloads are retained as `derived` surfaces from the source XML workbook when Excel import does not preserve those identifiers directly through COM.
3. The tracked SpreadsheetML fixture now exercises inherited style resolution through `calc -> calcBase`, and the derived formatting payload still resolves to `$#,##0.00`, `#112233`, and `#445566`.
4. `conditional_formatting_effective_style` is currently derived for the admitted SpreadsheetML expression-rule subset by combining source rule payloads with Excel formula evaluation on the target cell.
5. `views/normalized-replay.json` now publishes machine-readable `comparison_views` for `visible_value`, `effective_display_text`, `formatting_view`, and `conditional_formatting_view`, plus replay-facing `source_metadata` carrying projection status and provenance summaries.
6. The retained replay-adjacent artifact set also includes sidecar views:
   - `views/visible-value.json`
   - `views/effective-display-text.json`
   - `views/formatting-view.json`
   - `views/conditional-formatting-view.json`
7. A real DnaOneCalc-generated XML scenario now widens correctly through `requested_observation_scope.oxxlplay_required_surfaces` when executed against the updated OxXlPlay bridge.
8. `OxReplay` replay now loads the widened `comparison_views` and `source_metadata` directly from `views/normalized-replay.json`.
9. `OxReplay` diff and explain now classify `effective_display_text`, `formatting_view`, and `conditional_formatting_view` against the widened OxXlPlay artifact without requiring downstream mining of raw normalized event strings.
10. `OxReplay` validate-bundle now accepts the widened manifest and indexes the additional view files without requiring local replay-side mutation in this repo.

## Current limits
1. Style, number-format, color, and conditional-formatting rule surfaces in the SpreadsheetML family are retained as `derived`, not `direct`, because Excel XML import does not preserve all source identifiers and formatting carriers verbatim through COM on the exercised host.
2. `conditional_formatting_effective_style` is currently scoped to the admitted SpreadsheetML expression-rule subset; broader CF rule-kind coverage remains open.
3. The replay-facing seam now publishes declared comparison families directly, but broader cross-lane retained diff and explain exercise remains a sibling-repo coordination lane.
