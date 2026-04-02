#![forbid(unsafe_code)]

use oxxlplay_abstractions::{BridgeKind, InterpretationLimit, ScenarioId};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BridgeInvocationMode {
    InProcess,
    OutOfProcess,
    ComAutomation,
    DotNetInterop,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BridgeEnvelope {
    pub scenario_id: ScenarioId,
    pub bridge_kind: BridgeKind,
    pub bridge_version: String,
    pub executable_identity: Option<String>,
    pub command_channel: String,
    pub invocation_mode: BridgeInvocationMode,
    pub interpretation_limits: Vec<InterpretationLimit>,
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum BridgeEnvelopeValidationError {
    #[error("bridge envelope scenario id must not be blank")]
    BlankScenarioId,
    #[error("bridge version must not be blank")]
    BlankBridgeVersion,
    #[error("bridge command channel must not be blank")]
    BlankCommandChannel,
    #[error("non-pure-rust bridges must declare an executable identity")]
    MissingExecutableIdentity,
}

pub fn validate_bridge_envelope(
    envelope: &BridgeEnvelope,
) -> Result<(), BridgeEnvelopeValidationError> {
    if envelope.scenario_id.0.trim().is_empty() {
        return Err(BridgeEnvelopeValidationError::BlankScenarioId);
    }

    if envelope.bridge_version.trim().is_empty() {
        return Err(BridgeEnvelopeValidationError::BlankBridgeVersion);
    }

    if envelope.command_channel.trim().is_empty() {
        return Err(BridgeEnvelopeValidationError::BlankCommandChannel);
    }

    if envelope.bridge_kind != BridgeKind::PureRust
        && envelope
            .executable_identity
            .as_ref()
            .map(|value| value.trim().is_empty())
            .unwrap_or(true)
    {
        return Err(BridgeEnvelopeValidationError::MissingExecutableIdentity);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{
        BridgeEnvelope, BridgeEnvelopeValidationError, BridgeInvocationMode,
        validate_bridge_envelope,
    };
    use oxxlplay_abstractions::{BridgeKind, ScenarioId};

    const BRIDGE_FIXTURE: &str = include_str!(
        "../../../docs/test-corpus/excel/xlplay_provenance_excel_build_001/bridge.json"
    );

    fn load_fixture() -> BridgeEnvelope {
        serde_json::from_str(BRIDGE_FIXTURE).expect("fixture should deserialize")
    }

    #[test]
    fn validates_bridge_fixture() {
        let envelope = load_fixture();
        validate_bridge_envelope(&envelope).expect("expected valid bridge fixture");
        assert_eq!(
            envelope.invocation_mode,
            BridgeInvocationMode::DotNetInterop
        );
    }

    #[test]
    fn rejects_non_pure_rust_bridge_without_executable_identity() {
        let envelope = BridgeEnvelope {
            scenario_id: ScenarioId("xlplay_provenance_excel_build_001".to_owned()),
            bridge_kind: BridgeKind::DotNetBridge,
            bridge_version: "0.3.0".to_owned(),
            executable_identity: None,
            command_channel: "stdio-json".to_owned(),
            invocation_mode: BridgeInvocationMode::DotNetInterop,
            interpretation_limits: Vec::new(),
        };

        let err = validate_bridge_envelope(&envelope)
            .expect_err("expected missing executable identity to fail");
        assert_eq!(
            err,
            BridgeEnvelopeValidationError::MissingExecutableIdentity
        );
    }
}
