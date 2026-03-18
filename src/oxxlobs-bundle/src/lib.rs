#![forbid(unsafe_code)]

use oxxlobs_capture::ObservationCapture;
use oxxlobs_provenance::ObservationProvenance;
use oxxlobs_scenario::ObservationScenario;
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
    #[error("bundle seed must contain at least one observed surface")]
    EmptyCapture,
}

pub fn validate_bundle_seed(seed: &ReplayReadyBundleSeed) -> Result<(), BundleSeedError> {
    if seed.capture.surfaces.is_empty() {
        return Err(BundleSeedError::EmptyCapture);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{BundleSeedError, ReplayReadyBundleSeed, validate_bundle_seed};
    use oxxlobs_abstractions::{BridgeKind, ScenarioId};
    use oxxlobs_capture::ObservationCapture;
    use oxxlobs_provenance::ObservationProvenance;
    use oxxlobs_scenario::ObservationScenario;

    #[test]
    fn rejects_empty_capture() {
        let seed = ReplayReadyBundleSeed {
            scenario: ObservationScenario {
                scenario_id: ScenarioId("xlobs_bundle_seed_handoff_001".to_owned()),
                workbook_ref: "fixtures/book.xlsx".to_owned(),
                trigger: "recalc".to_owned(),
                observable_surfaces: vec!["Sheet1!A1".to_owned()],
            },
            provenance: ObservationProvenance {
                scenario_id: ScenarioId("xlobs_bundle_seed_handoff_001".to_owned()),
                excel_version: "16.0".to_owned(),
                excel_build: "17328".to_owned(),
                host_os: "Windows".to_owned(),
                bridge_kind: BridgeKind::PureRust,
                bridge_version: "0.1.0".to_owned(),
            },
            capture: ObservationCapture {
                surfaces: Vec::new(),
            },
        };

        let err = validate_bundle_seed(&seed).expect_err("expected invalid bundle seed");
        assert_eq!(err, BundleSeedError::EmptyCapture);
    }
}
