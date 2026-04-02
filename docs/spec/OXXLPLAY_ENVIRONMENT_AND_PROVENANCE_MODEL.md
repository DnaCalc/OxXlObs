# OXXLPLAY_ENVIRONMENT_AND_PROVENANCE_MODEL.md

## 1. Position
This document defines the retained provenance required for trustworthy Excel observation artifacts.

## 2. Required provenance fields
Every retained observation artifact should carry:
1. scenario id,
2. run id,
3. workbook ref or workbook fingerprint,
4. Excel version/build/channel metadata,
5. host OS and architecture metadata,
6. bridge kind and bridge version,
7. macro/security mode and automation policy where relevant,
8. timestamp and timezone metadata,
9. declared observable surfaces,
10. capture-loss and uncertainty summary.

## 3. Bridge provenance rule
If a non-Rust bridge is used, the retained artifact must state:
1. bridge kind,
2. bridge version,
3. executable or assembly identity when applicable,
4. transport or invocation mode,
5. known bridge limits that affect interpretation.

## 4. W003 retained provenance baseline
The retained W003 baseline uses JSON fixture files to pin environment and bridge facts before stable live execution is introduced.

### 4.1 Provenance fixture
1. The `provenance_minimal` scenario root retains `provenance.json`.
2. Minimum retained fields are:
   - `scenario_id`
   - `run_id`
   - `workbook_ref`
   - `workbook_fingerprint`
   - `excel_version`
   - `excel_build`
   - `excel_channel`
   - `host_os`
   - `host_architecture`
   - `macro_mode`
   - `automation_policy`
   - `captured_at_utc`
   - `timezone`
   - `declared_surface_ids`
   - `capture_loss_summary`
   - `uncertainty_summary`
   - `bridge`
3. `capture_loss_summary` and `uncertainty_summary` are summary lanes and therefore do not retain `none`; an empty list is the no-summary case.

### 4.2 Bridge fixture
1. The `provenance_minimal` scenario root also retains `bridge.json`.
2. Bridge envelopes retain:
   - `scenario_id`
   - `bridge_kind`
   - `bridge_version`
   - `executable_identity`
   - `command_channel`
   - `invocation_mode`
   - `interpretation_limits`
3. Non-`pure_rust` bridges must state an executable or assembly identity explicitly.

### 4.3 Environment fingerprint fixture
1. Environment fingerprints for W003 retain a parallel `states/excel/<scenario_id>/environment.json` snapshot.
2. The state snapshot is a retained environment witness, not a semantic authority.

### 4.4 Capture influence rule
1. Capture outputs may declare `bridge_influenced: true` when a bridge constrains interpretation.
2. If `bridge_influenced` is true, at least one interpretation limit must be retained.

## 5. W006 live provenance baseline
1. `W006` retains live driver provenance under `states/excel/xlplay_capture_values_formulae_001/`.
2. The current stable bridge envelope is:
   - `bridge_kind`: `external_process`
   - `bridge_version`: `w006-powershell-com.v1`
   - `executable_identity`: `pwsh:scripts/invoke-excel-observation.ps1`
   - `command_channel`: `json-file`
   - `invocation_mode`: `com_automation`
3. Live provenance may retain the raw local Office channel string when the host reports a Click-to-Run URL rather than a normalized channel label.
4. The retained state snapshot remains a host-bound environment witness and must keep workbook fingerprint, Excel build, macro mode, and bridge provenance explicit.
