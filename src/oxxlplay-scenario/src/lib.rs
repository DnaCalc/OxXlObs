#![forbid(unsafe_code)]

use std::collections::HashSet;

use oxxlplay_abstractions::{ObservableSurface, ReplayClass, ScenarioId};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ObservationScenario {
    pub scenario_id: ScenarioId,
    pub replay_class: ReplayClass,
    pub retained_root: String,
    pub workbook_ref: String,
    pub trigger: String,
    pub observable_surfaces: Vec<ObservableSurface>,
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum ScenarioValidationError {
    #[error("scenario id must not be blank")]
    BlankScenarioId,
    #[error("scenario retained root must not be blank")]
    BlankRetainedRoot,
    #[error("scenario must declare at least one observable surface")]
    MissingObservableSurface,
    #[error("scenario retained root must end with the scenario id")]
    RetainedRootMismatch,
    #[error("observable surface `{0}` is missing a surface id")]
    BlankSurfaceId(String),
    #[error("observable surface `{0}` is missing a locator")]
    BlankSurfaceLocator(String),
    #[error("observable surface id `{0}` is duplicated")]
    DuplicateSurfaceId(String),
}

pub fn validate_scenario(scenario: &ObservationScenario) -> Result<(), ScenarioValidationError> {
    if scenario.scenario_id.is_blank() {
        return Err(ScenarioValidationError::BlankScenarioId);
    }

    if scenario.retained_root.trim().is_empty() {
        return Err(ScenarioValidationError::BlankRetainedRoot);
    }

    let retained_root = scenario.retained_root.replace('\\', "/");
    if !retained_root.ends_with(&scenario.scenario_id.0) {
        return Err(ScenarioValidationError::RetainedRootMismatch);
    }

    if scenario.observable_surfaces.is_empty() {
        return Err(ScenarioValidationError::MissingObservableSurface);
    }

    let mut seen_surface_ids = HashSet::new();
    for surface in &scenario.observable_surfaces {
        if surface.surface_id.trim().is_empty() {
            return Err(ScenarioValidationError::BlankSurfaceId(
                surface.locator.clone(),
            ));
        }

        if surface.locator.trim().is_empty() {
            return Err(ScenarioValidationError::BlankSurfaceLocator(
                surface.surface_id.clone(),
            ));
        }

        if !seen_surface_ids.insert(surface.surface_id.clone()) {
            return Err(ScenarioValidationError::DuplicateSurfaceId(
                surface.surface_id.clone(),
            ));
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{ObservationScenario, ScenarioValidationError, validate_scenario};
    use oxxlplay_abstractions::{
        ObservableSurface, ObservableSurfaceKind, ReplayClass, ScenarioId,
    };

    const VALID_SCENARIO_FIXTURE: &str = include_str!(
        "../../../docs/test-corpus/excel/xlplay_manifest_minimal_valid_001/scenario.json"
    );
    const INVALID_SCENARIO_FIXTURE: &str = include_str!(
        "../../../docs/test-corpus/excel/xlplay_manifest_invalid_missing_surface_001/scenario.json"
    );

    fn load_fixture(fixture: &str) -> ObservationScenario {
        serde_json::from_str(fixture).expect("fixture should deserialize")
    }

    #[test]
    fn validates_minimal_valid_fixture() {
        let scenario = load_fixture(VALID_SCENARIO_FIXTURE);
        validate_scenario(&scenario).expect("expected valid scenario fixture");
        assert_eq!(scenario.replay_class, ReplayClass::ScenarioManifestValid);
    }

    #[test]
    fn rejects_missing_observable_surfaces_fixture() {
        let scenario = load_fixture(INVALID_SCENARIO_FIXTURE);
        let err = validate_scenario(&scenario).expect_err("expected invalid scenario fixture");
        assert_eq!(err, ScenarioValidationError::MissingObservableSurface);
    }

    #[test]
    fn rejects_duplicate_surface_ids() {
        let scenario = ObservationScenario {
            scenario_id: ScenarioId("xlplay_manifest_minimal_valid_001".to_owned()),
            replay_class: ReplayClass::ScenarioManifestValid,
            retained_root: "docs/test-corpus/excel/xlplay_manifest_minimal_valid_001".to_owned(),
            workbook_ref: "fixtures/minimal.xlsx".to_owned(),
            trigger: "open_then_recalc".to_owned(),
            observable_surfaces: vec![
                ObservableSurface {
                    surface_id: "sheet1_a1_value".to_owned(),
                    surface_kind: ObservableSurfaceKind::CellValue,
                    locator: "Sheet1!A1".to_owned(),
                    required: true,
                },
                ObservableSurface {
                    surface_id: "sheet1_a1_value".to_owned(),
                    surface_kind: ObservableSurfaceKind::FormulaText,
                    locator: "Sheet1!A1".to_owned(),
                    required: false,
                },
            ],
        };

        let err = validate_scenario(&scenario).expect_err("expected duplicate surface ids to fail");
        assert_eq!(
            err,
            ScenarioValidationError::DuplicateSurfaceId("sheet1_a1_value".to_owned())
        );
    }
}
