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

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CaptureRunBatchManifest {
    pub batch_id: String,
    pub output_root: String,
    #[serde(default)]
    pub shared_worker_options: SharedWorkerOptions,
    pub cases: Vec<CaptureRunBatchCase>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct SharedWorkerOptions {
    #[serde(default = "default_emit_bundle")]
    pub emit_bundle: bool,
    #[serde(default)]
    pub continue_after_case_failure: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_cases_per_worker: Option<u32>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CaptureRunBatchCase {
    pub case_id: String,
    pub workbook_ref: String,
    pub workbook_kind: BatchWorkbookKind,
    pub trigger: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub entered_cell_text: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub requested_observation_scope: Option<RequestedObservationScope>,
    pub case_output_dir: String,
    pub observable_surfaces: Vec<ObservableSurface>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum BatchWorkbookKind {
    #[serde(rename = "file-backed")]
    FileBacked,
    #[serde(rename = "spreadsheetml-2003-import")]
    SpreadsheetMl2003Import,
    #[serde(rename = "programmatic-formula")]
    ProgrammaticFormula,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct RequestedObservationScope {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub oxxlplay_required_surfaces: Vec<String>,
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

#[derive(Debug, Error, PartialEq, Eq)]
pub enum BatchManifestValidationError {
    #[error("batch id must not be blank")]
    BlankBatchId,
    #[error("batch output root must not be blank")]
    BlankOutputRoot,
    #[error("batch must declare at least one case")]
    MissingCase,
    #[error("shared worker option `max_cases_per_worker` must be greater than zero")]
    InvalidMaxCasesPerWorker,
    #[error("case id must not be blank")]
    BlankCaseId,
    #[error("case id `{0}` is duplicated")]
    DuplicateCaseId(String),
    #[error("case `{0}` workbook ref must not be blank")]
    BlankWorkbookRef(String),
    #[error("case `{0}` trigger must not be blank")]
    BlankTrigger(String),
    #[error("case `{0}` output dir must not be blank")]
    BlankCaseOutputDir(String),
    #[error("case `{0}` workbook_kind `programmatic-formula` requires entered_cell_text")]
    MissingEnteredCellText(String),
    #[error("case `{0}` must declare at least one observable surface")]
    MissingObservableSurface(String),
    #[error("case `{0}` observable surface `{1}` is missing a surface id")]
    BlankSurfaceId(String, String),
    #[error("case `{0}` observable surface `{1}` is missing a locator")]
    BlankSurfaceLocator(String, String),
    #[error("case `{0}` observable surface id `{1}` is duplicated")]
    DuplicateSurfaceId(String, String),
}

fn default_emit_bundle() -> bool {
    true
}

fn validate_observable_surfaces(
    surfaces: &[ObservableSurface],
) -> Result<(), ScenarioValidationError> {
    if surfaces.is_empty() {
        return Err(ScenarioValidationError::MissingObservableSurface);
    }

    let mut seen_surface_ids = HashSet::new();
    for surface in surfaces {
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

    validate_observable_surfaces(&scenario.observable_surfaces)
}

pub fn validate_batch_manifest(
    manifest: &CaptureRunBatchManifest,
) -> Result<(), BatchManifestValidationError> {
    if manifest.batch_id.trim().is_empty() {
        return Err(BatchManifestValidationError::BlankBatchId);
    }

    if manifest.output_root.trim().is_empty() {
        return Err(BatchManifestValidationError::BlankOutputRoot);
    }

    if manifest.cases.is_empty() {
        return Err(BatchManifestValidationError::MissingCase);
    }

    if manifest.shared_worker_options.max_cases_per_worker == Some(0) {
        return Err(BatchManifestValidationError::InvalidMaxCasesPerWorker);
    }

    let mut seen_case_ids = HashSet::new();
    for case in &manifest.cases {
        if case.case_id.trim().is_empty() {
            return Err(BatchManifestValidationError::BlankCaseId);
        }

        if !seen_case_ids.insert(case.case_id.clone()) {
            return Err(BatchManifestValidationError::DuplicateCaseId(
                case.case_id.clone(),
            ));
        }

        if case.workbook_ref.trim().is_empty() {
            return Err(BatchManifestValidationError::BlankWorkbookRef(
                case.case_id.clone(),
            ));
        }

        if case.trigger.trim().is_empty() {
            return Err(BatchManifestValidationError::BlankTrigger(
                case.case_id.clone(),
            ));
        }

        if case.case_output_dir.trim().is_empty() {
            return Err(BatchManifestValidationError::BlankCaseOutputDir(
                case.case_id.clone(),
            ));
        }

        if matches!(case.workbook_kind, BatchWorkbookKind::ProgrammaticFormula)
            && case.entered_cell_text.is_none()
        {
            return Err(BatchManifestValidationError::MissingEnteredCellText(
                case.case_id.clone(),
            ));
        }

        match validate_observable_surfaces(&case.observable_surfaces) {
            Ok(()) => {
            }
            Err(ScenarioValidationError::MissingObservableSurface) => {
                return Err(BatchManifestValidationError::MissingObservableSurface(
                    case.case_id.clone(),
                ));
            }
            Err(ScenarioValidationError::BlankSurfaceId(locator)) => {
                return Err(BatchManifestValidationError::BlankSurfaceId(
                    case.case_id.clone(),
                    locator,
                ));
            }
            Err(ScenarioValidationError::BlankSurfaceLocator(surface_id)) => {
                return Err(BatchManifestValidationError::BlankSurfaceLocator(
                    case.case_id.clone(),
                    surface_id,
                ));
            }
            Err(ScenarioValidationError::DuplicateSurfaceId(surface_id)) => {
                return Err(BatchManifestValidationError::DuplicateSurfaceId(
                    case.case_id.clone(),
                    surface_id,
                ));
            }
            Err(
                ScenarioValidationError::BlankScenarioId
                | ScenarioValidationError::BlankRetainedRoot
                | ScenarioValidationError::RetainedRootMismatch,
            ) => unreachable!("batch cases do not validate single-scenario root fields"),
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{
        BatchManifestValidationError, BatchWorkbookKind, CaptureRunBatchCase,
        CaptureRunBatchManifest, ObservationScenario, RequestedObservationScope,
        ScenarioValidationError, SharedWorkerOptions, validate_batch_manifest, validate_scenario,
    };
    use oxxlplay_abstractions::{
        ObservableSurface, ObservableSurfaceKind, ReplayClass, ScenarioId,
    };

    const VALID_SCENARIO_FIXTURE: &str = include_str!(
        "../../../docs/test-corpus/excel/xlplay_manifest_minimal_valid_001/scenario.json"
    );
    const INVALID_SCENARIO_FIXTURE: &str = include_str!(
        "../../../docs/test-corpus/excel/xlplay_manifest_invalid_missing_surface_001/scenario.json"
    );
    const VALID_BATCH_FIXTURE: &str = r#"{
        "batch_id": "batch_capture_values_formulae_001",
        "output_root": "states/excel/batches/batch_capture_values_formulae_001",
        "shared_worker_options": {
            "emit_bundle": true,
            "continue_after_case_failure": false,
            "max_cases_per_worker": 10
        },
        "cases": [
            {
                "case_id": "case_sum",
                "workbook_ref": "docs/test-corpus/excel/xlplay_capture_values_formulae_001/workbook.xlsx",
                "workbook_kind": "file-backed",
                "trigger": "open_then_recalc",
                "requested_observation_scope": {
                    "oxxlplay_required_surfaces": ["cell_value", "formula_text"]
                },
                "case_output_dir": "states/excel/batches/batch_capture_values_formulae_001/cases/case_sum",
                "observable_surfaces": [
                    {
                        "surface_id": "sheet1_a1_value",
                        "surface_kind": "cell_value",
                        "locator": "Sheet1!A1",
                        "required": true
                    }
                ]
            },
            {
                "case_id": "case_programmatic",
                "workbook_ref": "docs/test-corpus/excel/xlplay_capture_values_formulae_001/workbook.xlsx",
                "workbook_kind": "programmatic-formula",
                "trigger": "open_then_recalc",
                "entered_cell_text": "=1+2",
                "case_output_dir": "states/excel/batches/batch_capture_values_formulae_001/cases/case_programmatic",
                "observable_surfaces": [
                    {
                        "surface_id": "sheet1_a1_value",
                        "surface_kind": "cell_value",
                        "locator": "Sheet1!A1",
                        "required": true
                    }
                ]
            }
        ]
    }"#;

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

    fn load_batch_fixture(fixture: &str) -> CaptureRunBatchManifest {
        serde_json::from_str(fixture).expect("batch fixture should deserialize")
    }

    fn minimal_surface() -> ObservableSurface {
        ObservableSurface {
            surface_id: "sheet1_a1_value".to_owned(),
            surface_kind: ObservableSurfaceKind::CellValue,
            locator: "Sheet1!A1".to_owned(),
            required: true,
        }
    }

    #[test]
    fn validates_batch_manifest_fixture() {
        let manifest = load_batch_fixture(VALID_BATCH_FIXTURE);
        validate_batch_manifest(&manifest).expect("expected valid batch manifest");
        assert_eq!(manifest.batch_id, "batch_capture_values_formulae_001");
        assert_eq!(
            manifest.shared_worker_options,
            SharedWorkerOptions {
                emit_bundle: true,
                continue_after_case_failure: false,
                max_cases_per_worker: Some(10),
            }
        );
        assert_eq!(
            manifest.cases[0].requested_observation_scope,
            Some(RequestedObservationScope {
                oxxlplay_required_surfaces: vec![
                    "cell_value".to_owned(),
                    "formula_text".to_owned()
                ],
            })
        );
    }

    #[test]
    fn rejects_blank_batch_output_root() {
        let manifest = CaptureRunBatchManifest {
            batch_id: "batch_capture_values_formulae_001".to_owned(),
            output_root: " ".to_owned(),
            shared_worker_options: SharedWorkerOptions::default(),
            cases: vec![CaptureRunBatchCase {
                case_id: "case_sum".to_owned(),
                workbook_ref:
                    "docs/test-corpus/excel/xlplay_capture_values_formulae_001/workbook.xlsx"
                        .to_owned(),
                workbook_kind: BatchWorkbookKind::FileBacked,
                trigger: "open_then_recalc".to_owned(),
                entered_cell_text: None,
                requested_observation_scope: None,
                case_output_dir:
                    "states/excel/batches/batch_capture_values_formulae_001/cases/case_sum"
                        .to_owned(),
                observable_surfaces: vec![minimal_surface()],
            }],
        };

        let err = validate_batch_manifest(&manifest).expect_err("expected blank output root");
        assert_eq!(err, BatchManifestValidationError::BlankOutputRoot);
    }

    #[test]
    fn rejects_duplicate_case_ids() {
        let manifest = CaptureRunBatchManifest {
            batch_id: "batch_capture_values_formulae_001".to_owned(),
            output_root: "states/excel/batches/batch_capture_values_formulae_001".to_owned(),
            shared_worker_options: SharedWorkerOptions::default(),
            cases: vec![
                CaptureRunBatchCase {
                    case_id: "case_sum".to_owned(),
                    workbook_ref:
                        "docs/test-corpus/excel/xlplay_capture_values_formulae_001/workbook.xlsx"
                            .to_owned(),
                    workbook_kind: BatchWorkbookKind::FileBacked,
                    trigger: "open_then_recalc".to_owned(),
                    entered_cell_text: None,
                    requested_observation_scope: None,
                    case_output_dir:
                        "states/excel/batches/batch_capture_values_formulae_001/cases/case_sum_a"
                            .to_owned(),
                    observable_surfaces: vec![minimal_surface()],
                },
                CaptureRunBatchCase {
                    case_id: "case_sum".to_owned(),
                    workbook_ref:
                        "docs/test-corpus/excel/xlplay_capture_values_formulae_001/workbook.xlsx"
                            .to_owned(),
                    workbook_kind: BatchWorkbookKind::SpreadsheetMl2003Import,
                    trigger: "open_then_recalc".to_owned(),
                    entered_cell_text: None,
                    requested_observation_scope: None,
                    case_output_dir:
                        "states/excel/batches/batch_capture_values_formulae_001/cases/case_sum_b"
                            .to_owned(),
                    observable_surfaces: vec![minimal_surface()],
                },
            ],
        };

        let err = validate_batch_manifest(&manifest).expect_err("expected duplicate case ids");
        assert_eq!(
            err,
            BatchManifestValidationError::DuplicateCaseId("case_sum".to_owned())
        );
    }

    #[test]
    fn rejects_programmatic_formula_case_without_entered_text() {
        let manifest = CaptureRunBatchManifest {
            batch_id: "batch_capture_values_formulae_001".to_owned(),
            output_root: "states/excel/batches/batch_capture_values_formulae_001".to_owned(),
            shared_worker_options: SharedWorkerOptions::default(),
            cases: vec![CaptureRunBatchCase {
                case_id: "case_programmatic".to_owned(),
                workbook_ref:
                    "docs/test-corpus/excel/xlplay_capture_values_formulae_001/workbook.xlsx"
                        .to_owned(),
                workbook_kind: BatchWorkbookKind::ProgrammaticFormula,
                trigger: "open_then_recalc".to_owned(),
                entered_cell_text: None,
                requested_observation_scope: None,
                case_output_dir:
                    "states/excel/batches/batch_capture_values_formulae_001/cases/case_programmatic"
                        .to_owned(),
                observable_surfaces: vec![minimal_surface()],
            }],
        };

        let err = validate_batch_manifest(&manifest)
            .expect_err("expected missing entered cell text to fail");
        assert_eq!(
            err,
            BatchManifestValidationError::MissingEnteredCellText(
                "case_programmatic".to_owned()
            )
        );
    }
}
