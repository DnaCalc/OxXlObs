#![forbid(unsafe_code)]

use oxxlobs_abstractions::ScenarioId;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ObservationScenario {
    pub scenario_id: ScenarioId,
    pub workbook_ref: String,
    pub trigger: String,
    pub observable_surfaces: Vec<String>,
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum ScenarioValidationError {
    #[error("scenario must declare at least one observable surface")]
    MissingObservableSurface,
}

pub fn validate_scenario(scenario: &ObservationScenario) -> Result<(), ScenarioValidationError> {
    if scenario.observable_surfaces.is_empty() {
        return Err(ScenarioValidationError::MissingObservableSurface);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{ObservationScenario, ScenarioValidationError, validate_scenario};
    use oxxlobs_abstractions::ScenarioId;

    #[test]
    fn rejects_missing_observable_surfaces() {
        let scenario = ObservationScenario {
            scenario_id: ScenarioId("xlobs_manifest_invalid_missing_surface_001".to_owned()),
            workbook_ref: "fixtures/book.xlsx".to_owned(),
            trigger: "open".to_owned(),
            observable_surfaces: Vec::new(),
        };

        let err = validate_scenario(&scenario).expect_err("expected invalid scenario");
        assert_eq!(err, ScenarioValidationError::MissingObservableSurface);
    }
}
