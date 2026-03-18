#![forbid(unsafe_code)]

use oxxlobs_abstractions::{BridgeKind, ScenarioId};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ObservationProvenance {
    pub scenario_id: ScenarioId,
    pub excel_version: String,
    pub excel_build: String,
    pub host_os: String,
    pub bridge_kind: BridgeKind,
    pub bridge_version: String,
}
