#![forbid(unsafe_code)]

use oxxlobs_abstractions::{CaptureLoss, SurfaceStatus};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ObservedSurface {
    pub surface_name: String,
    pub status: SurfaceStatus,
    pub value_repr: String,
    pub capture_loss: CaptureLoss,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ObservationCapture {
    pub surfaces: Vec<ObservedSurface>,
}
