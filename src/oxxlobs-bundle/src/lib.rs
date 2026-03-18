#![forbid(unsafe_code)]

use oxxlobs_capture::{CaptureValidationError, ObservationCapture, validate_capture};
use oxxlobs_provenance::{ObservationProvenance, ProvenanceValidationError, validate_provenance};
use oxxlobs_scenario::{ObservationScenario, ScenarioValidationError, validate_scenario};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BundleSidecarRef {
    pub kind: String,
    pub path: String,
    pub media_type: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BundleHandoff {
    pub intended_replay_consumers: Vec<String>,
    pub intended_diff_consumers: Vec<String>,
    pub capability_hints: Vec<String>,
    pub pack_hints: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReplayReadyBundleSeed {
    pub bundle_schema: String,
    pub scenario: ObservationScenario,
    pub provenance: ObservationProvenance,
    pub capture: ObservationCapture,
    pub sidecars: Vec<BundleSidecarRef>,
    pub handoff: BundleHandoff,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HandoffValidationOutput {
    pub bundle_schema: String,
    pub checked_consumers: Vec<String>,
    pub valid: bool,
    pub notes: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AdapterCapabilityLevel {
    C0IngestValid,
    C1ReplayValid,
    C2DiffValid,
    C3ExplainValid,
    C4DistillValid,
    C5PackValid,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ComparisonArtifactRef {
    pub lane_id: String,
    pub producer_id: String,
    pub artifact_ref: String,
    pub adapter_id: String,
    pub adapter_version: String,
    pub capability_level: AdapterCapabilityLevel,
    pub engine_config_ref: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MismatchKind {
    ValueMismatch,
    ErrorMismatch,
    MissingSurface,
    CaptureLossDifference,
    StatusMismatch,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WitnessSeverity {
    Info,
    Warning,
    Error,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DivergenceRecord {
    pub surface_id: String,
    pub mismatch_kind: MismatchKind,
    pub severity: WitnessSeverity,
    pub excel_value_repr: Option<String>,
    pub comparison_value_repr: Option<String>,
    pub comparison_capture_loss_note: Option<String>,
    pub explanatory_note: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WitnessLifecycleState {
    ExplanatoryOnly,
    RetainedLocal,
    Quarantined,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DifferentialWitnessSeed {
    pub witness_schema: String,
    pub source_bundle: ReplayReadyBundleSeed,
    pub comparison_refs: Vec<ComparisonArtifactRef>,
    pub divergences: Vec<DivergenceRecord>,
    pub lifecycle_state: WitnessLifecycleState,
    pub quarantine_reason: Option<String>,
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum BundleSeedError {
    #[error(transparent)]
    InvalidScenario(#[from] ScenarioValidationError),
    #[error(transparent)]
    InvalidCapture(#[from] CaptureValidationError),
    #[error(transparent)]
    InvalidProvenance(#[from] ProvenanceValidationError),
    #[error("bundle schema must not be blank")]
    BlankBundleSchema,
    #[error("bundle handoff must declare at least one replay consumer")]
    MissingReplayConsumer,
    #[error("sidecar `{0}` is missing a kind")]
    BlankSidecarKind(usize),
    #[error("sidecar `{0}` is missing a path")]
    BlankSidecarPath(usize),
    #[error("sidecar `{0}` uses an absolute path")]
    AbsoluteSidecarPath(usize),
    #[error("sidecar `{0}` is missing a media type")]
    BlankSidecarMediaType(usize),
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum WitnessSeedError {
    #[error(transparent)]
    InvalidSourceBundle(#[from] BundleSeedError),
    #[error("witness schema must not be blank")]
    BlankWitnessSchema,
    #[error("witness seed must declare at least one comparison ref")]
    MissingComparisonRef,
    #[error("comparison ref `{0}` is missing a lane id")]
    BlankComparisonLaneId(usize),
    #[error("comparison ref `{0}` is missing an artifact ref")]
    BlankComparisonArtifactRef(usize),
    #[error("comparison ref `{0}` is missing an adapter id")]
    BlankComparisonAdapterId(usize),
    #[error("comparison ref `{0}` is missing an adapter version")]
    BlankComparisonAdapterVersion(usize),
    #[error("comparison ref `{0}` is missing an engine config ref")]
    BlankComparisonEngineConfigRef(usize),
    #[error("witness seed must retain at least one divergence")]
    MissingDivergence,
    #[error("divergence `{0}` is missing a surface id")]
    BlankDivergenceSurfaceId(usize),
    #[error("divergence `{0}` is missing an explanatory note")]
    BlankDivergenceNote(usize),
    #[error("quarantined witnesses must retain a quarantine reason")]
    MissingQuarantineReason,
    #[error("non-quarantined witnesses must not retain a quarantine reason")]
    UnexpectedQuarantineReason,
}

pub fn assemble_bundle_seed(
    bundle_schema: String,
    scenario: ObservationScenario,
    provenance: ObservationProvenance,
    capture: ObservationCapture,
    sidecars: Vec<BundleSidecarRef>,
    handoff: BundleHandoff,
) -> Result<ReplayReadyBundleSeed, BundleSeedError> {
    let seed = ReplayReadyBundleSeed {
        bundle_schema,
        scenario,
        provenance,
        capture,
        sidecars,
        handoff,
    };
    validate_bundle_seed(&seed)?;
    Ok(seed)
}

pub fn validate_bundle_seed(seed: &ReplayReadyBundleSeed) -> Result<(), BundleSeedError> {
    validate_scenario(&seed.scenario)?;
    validate_capture(&seed.capture)?;
    validate_provenance(&seed.provenance)?;

    if seed.bundle_schema.trim().is_empty() {
        return Err(BundleSeedError::BlankBundleSchema);
    }

    if seed.handoff.intended_replay_consumers.is_empty() {
        return Err(BundleSeedError::MissingReplayConsumer);
    }

    for (index, sidecar) in seed.sidecars.iter().enumerate() {
        if sidecar.kind.trim().is_empty() {
            return Err(BundleSeedError::BlankSidecarKind(index));
        }

        if sidecar.path.trim().is_empty() {
            return Err(BundleSeedError::BlankSidecarPath(index));
        }

        if is_absolute_repo_path(&sidecar.path) {
            return Err(BundleSeedError::AbsoluteSidecarPath(index));
        }

        if sidecar.media_type.trim().is_empty() {
            return Err(BundleSeedError::BlankSidecarMediaType(index));
        }
    }

    Ok(())
}

pub fn validate_handoff(seed: &ReplayReadyBundleSeed) -> HandoffValidationOutput {
    let mut checked_consumers = seed.handoff.intended_replay_consumers.clone();
    for consumer in &seed.handoff.intended_diff_consumers {
        if !checked_consumers.contains(consumer) {
            checked_consumers.push(consumer.clone());
        }
    }

    let notes = vec![
        if seed.bundle_schema.trim().is_empty() {
            "bundle schema missing".to_owned()
        } else {
            "bundle schema present".to_owned()
        },
        if seed.handoff.intended_replay_consumers.is_empty() {
            "replay consumers missing".to_owned()
        } else {
            "replay consumers declared".to_owned()
        },
        if seed
            .sidecars
            .iter()
            .any(|sidecar| is_absolute_repo_path(&sidecar.path))
        {
            "absolute sidecar refs are not allowed".to_owned()
        } else {
            "all sidecar refs are repo-relative".to_owned()
        },
    ];

    HandoffValidationOutput {
        bundle_schema: seed.bundle_schema.clone(),
        checked_consumers,
        valid: validate_bundle_seed(seed).is_ok(),
        notes,
    }
}

fn is_absolute_repo_path(path: &str) -> bool {
    let normalized = path.replace('\\', "/");
    normalized.starts_with('/')
        || normalized.starts_with("//")
        || (normalized.len() > 1 && normalized.as_bytes()[1] == b':')
}

pub fn validate_witness_seed(seed: &DifferentialWitnessSeed) -> Result<(), WitnessSeedError> {
    validate_bundle_seed(&seed.source_bundle)?;

    if seed.witness_schema.trim().is_empty() {
        return Err(WitnessSeedError::BlankWitnessSchema);
    }

    if seed.comparison_refs.is_empty() {
        return Err(WitnessSeedError::MissingComparisonRef);
    }

    for (index, comparison_ref) in seed.comparison_refs.iter().enumerate() {
        if comparison_ref.lane_id.trim().is_empty() {
            return Err(WitnessSeedError::BlankComparisonLaneId(index));
        }
        if comparison_ref.artifact_ref.trim().is_empty() {
            return Err(WitnessSeedError::BlankComparisonArtifactRef(index));
        }
        if comparison_ref.adapter_id.trim().is_empty() {
            return Err(WitnessSeedError::BlankComparisonAdapterId(index));
        }
        if comparison_ref.adapter_version.trim().is_empty() {
            return Err(WitnessSeedError::BlankComparisonAdapterVersion(index));
        }
        if comparison_ref.engine_config_ref.trim().is_empty() {
            return Err(WitnessSeedError::BlankComparisonEngineConfigRef(index));
        }
    }

    if seed.divergences.is_empty() {
        return Err(WitnessSeedError::MissingDivergence);
    }

    for (index, divergence) in seed.divergences.iter().enumerate() {
        if divergence.surface_id.trim().is_empty() {
            return Err(WitnessSeedError::BlankDivergenceSurfaceId(index));
        }
        if divergence.explanatory_note.trim().is_empty() {
            return Err(WitnessSeedError::BlankDivergenceNote(index));
        }
    }

    match seed.lifecycle_state {
        WitnessLifecycleState::Quarantined => {
            if seed
                .quarantine_reason
                .as_ref()
                .map(|reason| reason.trim().is_empty())
                .unwrap_or(true)
            {
                return Err(WitnessSeedError::MissingQuarantineReason);
            }
        }
        _ => {
            if seed.quarantine_reason.is_some() {
                return Err(WitnessSeedError::UnexpectedQuarantineReason);
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{
        BundleSeedError, DifferentialWitnessSeed, HandoffValidationOutput, ReplayReadyBundleSeed,
        WitnessLifecycleState, WitnessSeedError, assemble_bundle_seed, validate_bundle_seed,
        validate_handoff, validate_witness_seed,
    };
    use oxxlobs_bridge::BridgeInvocationMode;

    const BUNDLE_FIXTURE: &str =
        include_str!("../../../docs/test-corpus/bundles/xlobs_bundle_seed_handoff_001/bundle.json");
    const HANDOFF_FIXTURE: &str = include_str!(
        "../../../docs/test-corpus/bundles/xlobs_bundle_seed_handoff_001/handoff-validation.json"
    );
    const WITNESS_FIXTURE: &str = include_str!(
        "../../../docs/test-corpus/bundles/xlobs_witness_seed_divergence_001/witness-seed.json"
    );
    const LIVE_DRIVER_BUNDLE_FIXTURE: &str =
        include_str!("../../../states/excel/xlobs_capture_values_formulae_001/bundle.json");

    fn load_bundle_fixture() -> ReplayReadyBundleSeed {
        serde_json::from_str(BUNDLE_FIXTURE).expect("bundle fixture should deserialize")
    }

    fn load_handoff_fixture() -> HandoffValidationOutput {
        serde_json::from_str(HANDOFF_FIXTURE).expect("handoff fixture should deserialize")
    }

    fn load_witness_fixture() -> DifferentialWitnessSeed {
        serde_json::from_str(WITNESS_FIXTURE).expect("witness fixture should deserialize")
    }

    fn load_live_driver_bundle_fixture() -> ReplayReadyBundleSeed {
        serde_json::from_str(LIVE_DRIVER_BUNDLE_FIXTURE)
            .expect("live driver bundle fixture should deserialize")
    }

    #[test]
    fn validates_bundle_fixture() {
        let seed = load_bundle_fixture();
        validate_bundle_seed(&seed).expect("expected valid bundle fixture");
    }

    #[test]
    fn assembles_bundle_seed_from_valid_parts() {
        let seed = load_bundle_fixture();
        let assembled = assemble_bundle_seed(
            seed.bundle_schema.clone(),
            seed.scenario.clone(),
            seed.provenance.clone(),
            seed.capture.clone(),
            seed.sidecars.clone(),
            seed.handoff.clone(),
        )
        .expect("expected assembly to succeed");

        assert_eq!(assembled.bundle_schema, "oxxlobs.replay_bundle_seed.v1");
        assert_eq!(
            assembled.handoff.intended_replay_consumers,
            vec!["OxReplay"]
        );
    }

    #[test]
    fn validate_handoff_matches_fixture() {
        let output = validate_handoff(&load_bundle_fixture());
        let expected = load_handoff_fixture();
        assert_eq!(output, expected);
    }

    #[test]
    fn rejects_absolute_sidecar_path() {
        let mut seed = load_bundle_fixture();
        seed.sidecars[0].path = "C:/absolute/path.md".to_owned();
        let err = validate_bundle_seed(&seed).expect_err("expected absolute sidecar path to fail");
        assert_eq!(err, BundleSeedError::AbsoluteSidecarPath(0));
    }

    #[test]
    fn validates_witness_fixture() {
        let witness = load_witness_fixture();
        validate_witness_seed(&witness).expect("expected valid witness fixture");
        assert_eq!(witness.comparison_refs[0].lane_id, "OxCalc");
    }

    #[test]
    fn validates_live_driver_bundle_fixture() {
        let seed = load_live_driver_bundle_fixture();
        validate_bundle_seed(&seed).expect("expected live driver bundle fixture to validate");
        assert_eq!(
            seed.provenance.bridge.invocation_mode,
            BridgeInvocationMode::ComAutomation
        );
    }

    #[test]
    fn rejects_quarantined_witness_without_reason() {
        let mut witness = load_witness_fixture();
        witness.lifecycle_state = WitnessLifecycleState::Quarantined;
        witness.quarantine_reason = None;
        let err = validate_witness_seed(&witness)
            .expect_err("expected quarantined witness without reason to fail");
        assert_eq!(err, WitnessSeedError::MissingQuarantineReason);
    }
}
