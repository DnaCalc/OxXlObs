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

#[cfg(test)]
mod tests {
    use super::{
        BundleSeedError, HandoffValidationOutput, ReplayReadyBundleSeed, assemble_bundle_seed,
        validate_bundle_seed, validate_handoff,
    };

    const BUNDLE_FIXTURE: &str =
        include_str!("../../../docs/test-corpus/bundles/xlobs_bundle_seed_handoff_001/bundle.json");
    const HANDOFF_FIXTURE: &str = include_str!(
        "../../../docs/test-corpus/bundles/xlobs_bundle_seed_handoff_001/handoff-validation.json"
    );

    fn load_bundle_fixture() -> ReplayReadyBundleSeed {
        serde_json::from_str(BUNDLE_FIXTURE).expect("bundle fixture should deserialize")
    }

    fn load_handoff_fixture() -> HandoffValidationOutput {
        serde_json::from_str(HANDOFF_FIXTURE).expect("handoff fixture should deserialize")
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
}
