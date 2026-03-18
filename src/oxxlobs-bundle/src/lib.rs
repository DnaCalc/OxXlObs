#![forbid(unsafe_code)]

use oxxlobs_capture::{CaptureValidationError, ObservationCapture, validate_capture};
use oxxlobs_provenance::{ObservationProvenance, ProvenanceValidationError, validate_provenance};
use oxxlobs_scenario::{ObservationScenario, ScenarioValidationError, validate_scenario};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReplayReadyBundleSeed {
    pub scenario: ObservationScenario,
    pub provenance: ObservationProvenance,
    pub capture: ObservationCapture,
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum BundleSeedError {
    #[error(transparent)]
    InvalidScenario(#[from] ScenarioValidationError),
    #[error(transparent)]
    InvalidCapture(#[from] CaptureValidationError),
    #[error(transparent)]
    InvalidProvenance(#[from] ProvenanceValidationError),
}

pub fn validate_bundle_seed(seed: &ReplayReadyBundleSeed) -> Result<(), BundleSeedError> {
    validate_scenario(&seed.scenario)?;
    validate_capture(&seed.capture)?;
    validate_provenance(&seed.provenance)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{BundleSeedError, ReplayReadyBundleSeed, validate_bundle_seed};
    use oxxlobs_abstractions::{
        BridgeKind, CaptureInterpretation, CaptureLoss, ObservableSurface, ObservableSurfaceKind,
        ObservationUncertainty, ReplayClass, ScenarioId, SurfaceStatus,
    };
    use oxxlobs_bridge::{BridgeEnvelope, BridgeInvocationMode};
    use oxxlobs_capture::{CaptureValidationError, ObservationCapture, ObservedSurface};
    use oxxlobs_provenance::{ObservationProvenance, ProvenanceValidationError};
    use oxxlobs_scenario::ObservationScenario;

    #[test]
    fn rejects_empty_capture() {
        let seed = ReplayReadyBundleSeed {
            scenario: ObservationScenario {
                scenario_id: ScenarioId("xlobs_bundle_seed_handoff_001".to_owned()),
                replay_class: ReplayClass::BundleSeedBasic,
                retained_root: "docs/test-corpus/bundles/xlobs_bundle_seed_handoff_001".to_owned(),
                workbook_ref: "fixtures/book.xlsx".to_owned(),
                trigger: "recalc".to_owned(),
                observable_surfaces: vec![ObservableSurface {
                    surface_id: "sheet1_a1_value".to_owned(),
                    surface_kind: ObservableSurfaceKind::CellValue,
                    locator: "Sheet1!A1".to_owned(),
                    required: true,
                }],
            },
            provenance: ObservationProvenance {
                scenario_id: ScenarioId("xlobs_bundle_seed_handoff_001".to_owned()),
                run_id: "run_bundle_001".to_owned(),
                workbook_ref: "fixtures/book.xlsx".to_owned(),
                workbook_fingerprint: Some("sha256:fixture".to_owned()),
                excel_version: "16.0".to_owned(),
                excel_build: "17328".to_owned(),
                excel_channel: "current".to_owned(),
                host_os: "Windows".to_owned(),
                host_architecture: "x64".to_owned(),
                macro_mode: "disabled".to_owned(),
                automation_policy: "clean_room_declared".to_owned(),
                captured_at_utc: "2026-03-18T10:15:30Z".to_owned(),
                timezone: "Africa/Johannesburg".to_owned(),
                declared_surface_ids: vec!["sheet1_a1_value".to_owned()],
                capture_loss_summary: Vec::new(),
                uncertainty_summary: Vec::new(),
                bridge: BridgeEnvelope {
                    scenario_id: ScenarioId("xlobs_bundle_seed_handoff_001".to_owned()),
                    bridge_kind: BridgeKind::PureRust,
                    bridge_version: "0.1.0".to_owned(),
                    executable_identity: None,
                    command_channel: "in-memory".to_owned(),
                    invocation_mode: BridgeInvocationMode::InProcess,
                    interpretation_limits: Vec::new(),
                },
            },
            capture: ObservationCapture {
                surfaces: Vec::new(),
                interpretation: CaptureInterpretation::default(),
            },
        };

        let err = validate_bundle_seed(&seed).expect_err("expected invalid bundle seed");
        assert_eq!(
            err,
            BundleSeedError::InvalidCapture(CaptureValidationError::EmptyCapture)
        );
    }

    #[test]
    fn accepts_minimal_valid_bundle_seed() {
        let seed = ReplayReadyBundleSeed {
            scenario: ObservationScenario {
                scenario_id: ScenarioId("xlobs_bundle_seed_handoff_001".to_owned()),
                replay_class: ReplayClass::BundleSeedBasic,
                retained_root: "docs/test-corpus/bundles/xlobs_bundle_seed_handoff_001".to_owned(),
                workbook_ref: "fixtures/book.xlsx".to_owned(),
                trigger: "recalc".to_owned(),
                observable_surfaces: vec![ObservableSurface {
                    surface_id: "sheet1_a1_value".to_owned(),
                    surface_kind: ObservableSurfaceKind::CellValue,
                    locator: "Sheet1!A1".to_owned(),
                    required: true,
                }],
            },
            provenance: ObservationProvenance {
                scenario_id: ScenarioId("xlobs_bundle_seed_handoff_001".to_owned()),
                run_id: "run_bundle_001".to_owned(),
                workbook_ref: "fixtures/book.xlsx".to_owned(),
                workbook_fingerprint: Some("sha256:fixture".to_owned()),
                excel_version: "16.0".to_owned(),
                excel_build: "17328".to_owned(),
                excel_channel: "current".to_owned(),
                host_os: "Windows".to_owned(),
                host_architecture: "x64".to_owned(),
                macro_mode: "disabled".to_owned(),
                automation_policy: "clean_room_declared".to_owned(),
                captured_at_utc: "2026-03-18T10:15:30Z".to_owned(),
                timezone: "Africa/Johannesburg".to_owned(),
                declared_surface_ids: vec!["sheet1_a1_value".to_owned()],
                capture_loss_summary: Vec::new(),
                uncertainty_summary: Vec::new(),
                bridge: BridgeEnvelope {
                    scenario_id: ScenarioId("xlobs_bundle_seed_handoff_001".to_owned()),
                    bridge_kind: BridgeKind::PureRust,
                    bridge_version: "0.1.0".to_owned(),
                    executable_identity: None,
                    command_channel: "in-memory".to_owned(),
                    invocation_mode: BridgeInvocationMode::InProcess,
                    interpretation_limits: Vec::new(),
                },
            },
            capture: ObservationCapture {
                surfaces: vec![ObservedSurface {
                    surface: ObservableSurface {
                        surface_id: "sheet1_a1_value".to_owned(),
                        surface_kind: ObservableSurfaceKind::CellValue,
                        locator: "Sheet1!A1".to_owned(),
                        required: true,
                    },
                    status: SurfaceStatus::Direct,
                    value_repr: Some("42".to_owned()),
                    capture_loss: CaptureLoss::None,
                    uncertainty: ObservationUncertainty::None,
                }],
                interpretation: CaptureInterpretation::default(),
            },
        };

        validate_bundle_seed(&seed).expect("expected valid bundle seed");
    }

    #[test]
    fn rejects_invalid_provenance() {
        let seed = ReplayReadyBundleSeed {
            scenario: ObservationScenario {
                scenario_id: ScenarioId("xlobs_bundle_seed_handoff_001".to_owned()),
                replay_class: ReplayClass::BundleSeedBasic,
                retained_root: "docs/test-corpus/bundles/xlobs_bundle_seed_handoff_001".to_owned(),
                workbook_ref: "fixtures/book.xlsx".to_owned(),
                trigger: "recalc".to_owned(),
                observable_surfaces: vec![ObservableSurface {
                    surface_id: "sheet1_a1_value".to_owned(),
                    surface_kind: ObservableSurfaceKind::CellValue,
                    locator: "Sheet1!A1".to_owned(),
                    required: true,
                }],
            },
            provenance: ObservationProvenance {
                scenario_id: ScenarioId("xlobs_bundle_seed_handoff_001".to_owned()),
                run_id: String::new(),
                workbook_ref: "fixtures/book.xlsx".to_owned(),
                workbook_fingerprint: Some("sha256:fixture".to_owned()),
                excel_version: "16.0".to_owned(),
                excel_build: "17328".to_owned(),
                excel_channel: "current".to_owned(),
                host_os: "Windows".to_owned(),
                host_architecture: "x64".to_owned(),
                macro_mode: "disabled".to_owned(),
                automation_policy: "clean_room_declared".to_owned(),
                captured_at_utc: "2026-03-18T10:15:30Z".to_owned(),
                timezone: "Africa/Johannesburg".to_owned(),
                declared_surface_ids: vec!["sheet1_a1_value".to_owned()],
                capture_loss_summary: Vec::new(),
                uncertainty_summary: Vec::new(),
                bridge: BridgeEnvelope {
                    scenario_id: ScenarioId("xlobs_bundle_seed_handoff_001".to_owned()),
                    bridge_kind: BridgeKind::PureRust,
                    bridge_version: "0.1.0".to_owned(),
                    executable_identity: None,
                    command_channel: "in-memory".to_owned(),
                    invocation_mode: BridgeInvocationMode::InProcess,
                    interpretation_limits: Vec::new(),
                },
            },
            capture: ObservationCapture {
                surfaces: vec![ObservedSurface {
                    surface: ObservableSurface {
                        surface_id: "sheet1_a1_value".to_owned(),
                        surface_kind: ObservableSurfaceKind::CellValue,
                        locator: "Sheet1!A1".to_owned(),
                        required: true,
                    },
                    status: SurfaceStatus::Direct,
                    value_repr: Some("42".to_owned()),
                    capture_loss: CaptureLoss::None,
                    uncertainty: ObservationUncertainty::None,
                }],
                interpretation: CaptureInterpretation::default(),
            },
        };

        let err = validate_bundle_seed(&seed).expect_err("expected invalid provenance");
        assert_eq!(
            err,
            BundleSeedError::InvalidProvenance(ProvenanceValidationError::BlankRunId)
        );
    }
}
