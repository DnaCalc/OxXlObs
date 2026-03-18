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
