#![forbid(unsafe_code)]

use oxxlplay_abstractions::{
    CaptureInterpretation, CaptureLoss, ObservableSurface, ObservationUncertainty, SurfaceStatus,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ObservedSurface {
    pub surface: ObservableSurface,
    pub status: SurfaceStatus,
    pub value_repr: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value_json: Option<Value>,
    pub capture_loss: CaptureLoss,
    pub uncertainty: ObservationUncertainty,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ObservationCapture {
    pub surfaces: Vec<ObservedSurface>,
    #[serde(default)]
    pub interpretation: CaptureInterpretation,
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum CaptureValidationError {
    #[error("capture must contain at least one observed surface")]
    EmptyCapture,
    #[error("surface `{0}` is direct or derived but has no observed value")]
    MissingObservedValue(String),
    #[error("surface `{0}` is unavailable but still carries an observed value")]
    UnavailableSurfaceHasValue(String),
    #[error("surface `{0}` is unavailable but has no capture-loss marker")]
    UnavailableSurfaceMissingCaptureLoss(String),
    #[error("capture marks bridge influence but declares no interpretation limits")]
    MissingInterpretationLimit,
    #[error("capture declares interpretation limits without marking bridge influence")]
    InterpretationLimitWithoutBridgeInfluence,
}

pub fn validate_capture(capture: &ObservationCapture) -> Result<(), CaptureValidationError> {
    if capture.surfaces.is_empty() {
        return Err(CaptureValidationError::EmptyCapture);
    }

    if capture.interpretation.bridge_influenced
        && capture.interpretation.interpretation_limits.is_empty()
    {
        return Err(CaptureValidationError::MissingInterpretationLimit);
    }

    if !capture.interpretation.bridge_influenced
        && !capture.interpretation.interpretation_limits.is_empty()
    {
        return Err(CaptureValidationError::InterpretationLimitWithoutBridgeInfluence);
    }

    for surface in &capture.surfaces {
        match surface.status {
            SurfaceStatus::Direct | SurfaceStatus::Derived => {
                let has_value = has_observed_value(surface);
                if !has_value {
                    return Err(CaptureValidationError::MissingObservedValue(
                        surface.surface.surface_id.clone(),
                    ));
                }
            }
            SurfaceStatus::Unavailable => {
                if surface.value_repr.is_some() || surface.value_json.is_some() {
                    return Err(CaptureValidationError::UnavailableSurfaceHasValue(
                        surface.surface.surface_id.clone(),
                    ));
                }

                if surface.capture_loss == CaptureLoss::None {
                    return Err(
                        CaptureValidationError::UnavailableSurfaceMissingCaptureLoss(
                            surface.surface.surface_id.clone(),
                        ),
                    );
                }
            }
        }
    }

    Ok(())
}

fn has_observed_value(surface: &ObservedSurface) -> bool {
    surface
        .value_repr
        .as_ref()
        .map(|value| !value.trim().is_empty())
        .unwrap_or(false)
        || surface.value_json.is_some()
}

#[cfg(test)]
mod tests {
    use super::{CaptureValidationError, ObservationCapture, ObservedSurface, validate_capture};
    use oxxlplay_abstractions::{
        CaptureInterpretation, CaptureLoss, ObservableSurface, ObservableSurfaceKind,
        ObservationUncertainty, SurfaceStatus,
    };

    const BASIC_CAPTURE_FIXTURE: &str = include_str!(
        "../../../docs/test-corpus/excel/xlplay_capture_values_formulae_001/capture.json"
    );
    const LOSS_CAPTURE_FIXTURE: &str = include_str!(
        "../../../docs/test-corpus/excel/xlplay_capture_loss_formula_unavailable_001/capture.json"
    );
    const SPREADSHEETML_CAPTURE_FIXTURE: &str = include_str!(
        "../../../docs/test-corpus/excel/xlplay_capture_spreadsheetml_formatting_001/capture.json"
    );

    fn load_fixture(fixture: &str) -> ObservationCapture {
        serde_json::from_str(fixture).expect("fixture should deserialize")
    }

    #[test]
    fn validates_basic_capture_fixture() {
        let capture = load_fixture(BASIC_CAPTURE_FIXTURE);
        validate_capture(&capture).expect("expected valid capture fixture");
        assert_eq!(capture.surfaces.len(), 2);
        assert!(!capture.interpretation.bridge_influenced);
    }

    #[test]
    fn validates_capture_loss_fixture() {
        let capture = load_fixture(LOSS_CAPTURE_FIXTURE);
        validate_capture(&capture).expect("expected valid capture-loss fixture");
        assert_eq!(capture.surfaces[1].status, SurfaceStatus::Unavailable);
        assert_eq!(
            capture.surfaces[1].capture_loss,
            CaptureLoss::FormulaUnavailable
        );
        assert!(capture.interpretation.bridge_influenced);
    }

    #[test]
    fn validates_spreadsheetml_capture_fixture() {
        let capture = load_fixture(SPREADSHEETML_CAPTURE_FIXTURE);
        validate_capture(&capture).expect("expected SpreadsheetML capture fixture");
        assert_eq!(capture.surfaces.len(), 9);
        assert!(capture.interpretation.bridge_influenced);
        assert!(
            capture
                .surfaces
                .iter()
                .any(|surface| surface.value_json.is_some())
        );
    }

    #[test]
    fn rejects_unavailable_surface_without_capture_loss() {
        let capture = ObservationCapture {
            surfaces: vec![ObservedSurface {
                surface: ObservableSurface {
                    surface_id: "sheet1_a1_formula".to_owned(),
                    surface_kind: ObservableSurfaceKind::FormulaText,
                    locator: "Sheet1!A1".to_owned(),
                    required: false,
                },
                status: SurfaceStatus::Unavailable,
                value_repr: None,
                value_json: None,
                capture_loss: CaptureLoss::None,
                uncertainty: ObservationUncertainty::None,
            }],
            interpretation: CaptureInterpretation::default(),
        };

        let err = validate_capture(&capture)
            .expect_err("expected unavailable surface without capture-loss to fail");
        assert_eq!(
            err,
            CaptureValidationError::UnavailableSurfaceMissingCaptureLoss(
                "sheet1_a1_formula".to_owned(),
            )
        );
    }

    #[test]
    fn rejects_bridge_influence_without_limits() {
        let capture = ObservationCapture {
            surfaces: vec![ObservedSurface {
                surface: ObservableSurface {
                    surface_id: "sheet1_a1_value".to_owned(),
                    surface_kind: ObservableSurfaceKind::CellValue,
                    locator: "Sheet1!A1".to_owned(),
                    required: true,
                },
                status: SurfaceStatus::Direct,
                value_repr: Some("42".to_owned()),
                value_json: None,
                capture_loss: CaptureLoss::None,
                uncertainty: ObservationUncertainty::None,
            }],
            interpretation: CaptureInterpretation {
                bridge_influenced: true,
                interpretation_limits: Vec::new(),
            },
        };

        let err =
            validate_capture(&capture).expect_err("expected missing interpretation limits to fail");
        assert_eq!(err, CaptureValidationError::MissingInterpretationLimit);
    }

    #[test]
    fn accepts_structured_json_payload_for_derived_surface() {
        let capture = ObservationCapture {
            surfaces: vec![ObservedSurface {
                surface: ObservableSurface {
                    surface_id: "input_a1_conditional_rules".to_owned(),
                    surface_kind: ObservableSurfaceKind::ConditionalFormattingRules,
                    locator: "Input!A1".to_owned(),
                    required: true,
                },
                status: SurfaceStatus::Derived,
                value_repr: None,
                value_json: Some(serde_json::json!([
                    {
                        "range": "A1",
                        "rule_kind": "Expression",
                        "formula": "=A1>0"
                    }
                ])),
                capture_loss: CaptureLoss::None,
                uncertainty: ObservationUncertainty::PostProcessed,
            }],
            interpretation: CaptureInterpretation::default(),
        };

        validate_capture(&capture).expect("expected structured payload to validate");
    }
}
