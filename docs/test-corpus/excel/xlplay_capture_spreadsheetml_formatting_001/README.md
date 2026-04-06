# xlplay_capture_spreadsheetml_formatting_001

- Replay class: `capture_surface_spreadsheetml_formatting`
- Status: `partial`
- Retained root declared in: `docs/spec/OXXLPLAY_SCENARIO_REGISTER.md`
- Retained files present: `scenario.json`, `capture.json`, `workbook.xml`

This directory carries the first SpreadsheetML 2003 observation family for OxXlPlay.
It widens the retained target-cell envelope from direct formula/value capture into:
- direct Excel `cell_value`
- direct Excel `effective_display_text`
- SpreadsheetML-source-derived style and formatting facts
- SpreadsheetML-source-derived conditional-formatting rule and effective-style projections for the admitted expression-rule subset

The tracked workbook fixture now exercises SpreadsheetML style inheritance explicitly:
- cell style `calc` inherits formatting from parent style `calcBase`
- retained `number_format_code`, `font_color`, and `fill_color` therefore depend on parent-style resolution rather than flat local-style parsing

The Excel-driven retained outputs for this scenario family are emitted under `states/excel/xlplay_capture_spreadsheetml_formatting_001/`.
