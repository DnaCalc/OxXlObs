#![forbid(unsafe_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ScenarioId(pub String);

impl ScenarioId {
    pub fn is_blank(&self) -> bool {
        self.0.trim().is_empty()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReplayClass {
    ScenarioManifestValid,
    ScenarioManifestInvalid,
    CaptureSurfaceBasic,
    CaptureSurfaceSpreadsheetmlFormatting,
    CaptureLossMarked,
    ProvenanceMinimal,
    BundleSeedBasic,
    WitnessSeedDiff,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BridgeKind {
    PureRust,
    ExternalProcess,
    DotNetBridge,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ObservableSurfaceKind {
    WorkbookIdentity,
    CellValue,
    FormulaText,
    EffectiveDisplayText,
    NumberFormatCode,
    StyleId,
    FontColor,
    FillColor,
    ConditionalFormattingRules,
    ConditionalFormattingEffectiveStyle,
    DefinedNameValue,
    ErrorValue,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ObservableSurface {
    pub surface_id: String,
    pub surface_kind: ObservableSurfaceKind,
    pub locator: String,
    pub required: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InterpretationLimitKind {
    FormulaTextUnavailable,
    MacroSecurityOpaque,
    WorkbookFingerprintPartial,
    TransportOpaque,
    ValueRenderingNormalized,
    EffectiveDisplayHostRendered,
    SpreadsheetMlSourceProjection,
    ConditionalFormattingRuleProjection,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InterpretationLimit {
    pub kind: InterpretationLimitKind,
    pub detail: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SurfaceStatus {
    Direct,
    Derived,
    Unavailable,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CaptureLoss {
    None,
    FormulaUnavailable,
    DiagnosticUnavailable,
    EnvironmentPartial,
    SurfaceNotCaptured,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ObservationUncertainty {
    None,
    Sampled,
    PostProcessed,
    WorkbookIdentityAssumed,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct CaptureInterpretation {
    pub bridge_influenced: bool,
    pub interpretation_limits: Vec<InterpretationLimit>,
}
