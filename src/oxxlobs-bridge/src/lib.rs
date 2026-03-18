#![forbid(unsafe_code)]

use oxxlobs_abstractions::{BridgeKind, ScenarioId};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BridgeEnvelope {
    pub scenario_id: ScenarioId,
    pub bridge_kind: BridgeKind,
    pub bridge_version: String,
    pub command_channel: String,
}
