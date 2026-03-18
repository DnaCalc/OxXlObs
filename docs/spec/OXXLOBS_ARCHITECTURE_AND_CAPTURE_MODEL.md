# OXXLOBS_ARCHITECTURE_AND_CAPTURE_MODEL.md

## 1. Position
This document translates the repo mission into an initial observation-strata and capture model.

## 2. Intended strata
The initial split is:
1. `Abstractions`
2. `Scenario`
3. `Capture`
4. `Provenance`
5. `Bundle`
6. `Bridge`
7. `CLI`

## 3. Observation pipeline
The normalized pipeline is:
1. scenario declaration,
2. workbook and trigger preparation,
3. Excel execution through a declared bridge,
4. observable-surface capture,
5. provenance and lossiness attachment,
6. replay-ready bundle assembly,
7. retained run summary and handoff.

## 4. Source preservation rule
Retained artifacts must preserve:
1. scenario id,
2. workbook identity or fingerprint,
3. Excel build/version metadata,
4. trigger recipe,
5. directly observed versus derived status,
6. capture-loss or uncertainty markers when present.

## 5. Observable surfaces
The initial baseline surfaces should support:
1. workbook and workbook-part identity,
2. declared input mutations or trigger actions,
3. final observed cell or name values,
4. formula text where accessible,
5. error and status surfaces where accessible,
6. environment metadata needed to replay or compare the run honestly.

## 6. W002 retained shape baseline
The retained W002 baseline uses JSON fixture files inside declared scenario roots.

### 6.1 Scenario manifest file
1. Scenario declarations use `scenario.json`.
2. Minimum retained fields are:
   - `scenario_id`
   - `replay_class`
   - `retained_root`
   - `workbook_ref`
   - `trigger`
   - `observable_surfaces`
3. `observable_surfaces` entries carry:
   - `surface_id`
   - `surface_kind`
   - `locator`
   - `required`
4. Scenario validation rejects blank scenario ids, blank retained roots, empty surface lists, blank surface ids, blank locators, and duplicated surface ids.

### 6.2 Capture file
1. Capture-shape fixtures use `capture.json`.
2. Each captured surface carries:
   - `surface`
   - `status`
   - `value_repr`
   - `capture_loss`
   - `uncertainty`
3. `status` remains the baseline declaration of `direct`, `derived`, or `unavailable`.
4. `capture_loss` remains explicit even when the surface is unavailable rather than silently omitted.
5. `uncertainty` remains explicit even when currently `none` so later retained artifacts do not need a shape break to express it.

### 6.3 Validation rule
1. Direct or derived surfaces must carry an observed value representation.
2. Unavailable surfaces must not carry an observed value representation.
3. Unavailable surfaces must carry a non-`none` capture-loss marker.
4. These rules establish the W002 baseline for `O0.scenario_valid` and `O1.capture_valid` without claiming Excel-run evidence beyond retained fixture shapes.
