#![forbid(unsafe_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScenarioId(pub String);

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum BridgeKind {
    PureRust,
    ExternalProcess,
    DotNetBridge,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SurfaceStatus {
    Direct,
    Derived,
    Unavailable,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CaptureLoss {
    None,
    FormulaUnavailable,
    DiagnosticUnavailable,
    EnvironmentPartial,
}
