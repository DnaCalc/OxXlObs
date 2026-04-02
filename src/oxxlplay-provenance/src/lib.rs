#![forbid(unsafe_code)]

use oxxlplay_abstractions::{CaptureLoss, ObservationUncertainty, ScenarioId};
use oxxlplay_bridge::{BridgeEnvelope, BridgeEnvelopeValidationError, validate_bridge_envelope};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ObservationProvenance {
    pub scenario_id: ScenarioId,
    pub run_id: String,
    pub workbook_ref: String,
    pub workbook_fingerprint: Option<String>,
    pub excel_version: String,
    pub excel_build: String,
    pub excel_channel: String,
    pub host_os: String,
    pub host_architecture: String,
    pub macro_mode: String,
    pub automation_policy: String,
    pub captured_at_utc: String,
    pub timezone: String,
    pub declared_surface_ids: Vec<String>,
    pub capture_loss_summary: Vec<CaptureLoss>,
    pub uncertainty_summary: Vec<ObservationUncertainty>,
    pub bridge: BridgeEnvelope,
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum ProvenanceValidationError {
    #[error("provenance scenario id must not be blank")]
    BlankScenarioId,
    #[error("provenance run id must not be blank")]
    BlankRunId,
    #[error("provenance workbook reference must not be blank")]
    BlankWorkbookRef,
    #[error("provenance Excel version must not be blank")]
    BlankExcelVersion,
    #[error("provenance Excel build must not be blank")]
    BlankExcelBuild,
    #[error("provenance Excel channel must not be blank")]
    BlankExcelChannel,
    #[error("provenance host OS must not be blank")]
    BlankHostOs,
    #[error("provenance host architecture must not be blank")]
    BlankHostArchitecture,
    #[error("provenance macro mode must not be blank")]
    BlankMacroMode,
    #[error("provenance automation policy must not be blank")]
    BlankAutomationPolicy,
    #[error("provenance capture timestamp must not be blank")]
    BlankCapturedAtUtc,
    #[error("provenance timezone must not be blank")]
    BlankTimezone,
    #[error("provenance must declare at least one observable surface id")]
    MissingDeclaredSurfaceId,
    #[error("provenance capture-loss summary must not include `none`")]
    InvalidCaptureLossSummary,
    #[error("provenance uncertainty summary must not include `none`")]
    InvalidUncertaintySummary,
    #[error(transparent)]
    InvalidBridgeEnvelope(#[from] BridgeEnvelopeValidationError),
}

pub fn validate_provenance(
    provenance: &ObservationProvenance,
) -> Result<(), ProvenanceValidationError> {
    if provenance.scenario_id.0.trim().is_empty() {
        return Err(ProvenanceValidationError::BlankScenarioId);
    }

    if provenance.run_id.trim().is_empty() {
        return Err(ProvenanceValidationError::BlankRunId);
    }

    if provenance.workbook_ref.trim().is_empty() {
        return Err(ProvenanceValidationError::BlankWorkbookRef);
    }

    if provenance.excel_version.trim().is_empty() {
        return Err(ProvenanceValidationError::BlankExcelVersion);
    }

    if provenance.excel_build.trim().is_empty() {
        return Err(ProvenanceValidationError::BlankExcelBuild);
    }

    if provenance.excel_channel.trim().is_empty() {
        return Err(ProvenanceValidationError::BlankExcelChannel);
    }

    if provenance.host_os.trim().is_empty() {
        return Err(ProvenanceValidationError::BlankHostOs);
    }

    if provenance.host_architecture.trim().is_empty() {
        return Err(ProvenanceValidationError::BlankHostArchitecture);
    }

    if provenance.macro_mode.trim().is_empty() {
        return Err(ProvenanceValidationError::BlankMacroMode);
    }

    if provenance.automation_policy.trim().is_empty() {
        return Err(ProvenanceValidationError::BlankAutomationPolicy);
    }

    if provenance.captured_at_utc.trim().is_empty() {
        return Err(ProvenanceValidationError::BlankCapturedAtUtc);
    }

    if provenance.timezone.trim().is_empty() {
        return Err(ProvenanceValidationError::BlankTimezone);
    }

    if provenance.declared_surface_ids.is_empty() {
        return Err(ProvenanceValidationError::MissingDeclaredSurfaceId);
    }

    if provenance.capture_loss_summary.contains(&CaptureLoss::None) {
        return Err(ProvenanceValidationError::InvalidCaptureLossSummary);
    }

    if provenance
        .uncertainty_summary
        .contains(&ObservationUncertainty::None)
    {
        return Err(ProvenanceValidationError::InvalidUncertaintySummary);
    }

    validate_bridge_envelope(&provenance.bridge)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{ObservationProvenance, ProvenanceValidationError, validate_provenance};

    const PROVENANCE_FIXTURE: &str = include_str!(
        "../../../docs/test-corpus/excel/xlplay_provenance_excel_build_001/provenance.json"
    );

    fn load_fixture() -> ObservationProvenance {
        serde_json::from_str(PROVENANCE_FIXTURE).expect("fixture should deserialize")
    }

    #[test]
    fn validates_provenance_fixture() {
        let provenance = load_fixture();
        validate_provenance(&provenance).expect("expected valid provenance fixture");
        assert_eq!(provenance.bridge.command_channel, "stdio-json");
    }

    #[test]
    fn rejects_blank_run_id() {
        let mut provenance = load_fixture();
        provenance.run_id.clear();
        let err = validate_provenance(&provenance).expect_err("expected blank run id to fail");
        assert_eq!(err, ProvenanceValidationError::BlankRunId);
    }
}
