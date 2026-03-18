# OXXLOBS_ENVIRONMENT_AND_PROVENANCE_MODEL.md

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
