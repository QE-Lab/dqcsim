use crate::common::types::{ArbData, PluginMetadata};
use serde::{Deserialize, Serialize};

/// Plugin to simulator responses.
#[derive(Debug, Serialize, Deserialize)]
pub enum PluginToSimulator {
    /// Success response to requests that don't return data..
    Success,

    /// Failure response to any request, containing an error message.
    Failure(String),

    /// Success response to `SimulatorToPlugin::Initialize`.
    Initialized(PluginInitializeResponse),

    /// Success response to `SimulatorToPlugin::RunRequest`.
    RunResponse(FrontendRunResponse),

    /// Success response to `SimulatorToPlugin::ArbRequest`.
    ArbResponse(ArbData),
}

/// Initialization response.
#[derive(Debug, Serialize, Deserialize)]
pub struct PluginInitializeResponse {
    /// Gatestream endpoint for the upstream plugin to connect to.
    ///
    /// Must be specified for operators and backends, must not be specified for
    /// frontends.
    pub upstream: Option<String>,

    /// Plugin metadata information from the `PluginDefinition` structure.
    pub metadata: PluginMetadata,
}

/// Frontend run response.
#[derive(Debug, Serialize, Deserialize)]
pub struct FrontendRunResponse {
    /// When specified, the frontend's `run()` callback terminated with the
    /// contained return value.
    pub complete: Option<ArbData>,

    /// Messages queued up through the frontend's `send()` function, to be
    /// consumed by the host's `recv()` function.
    pub messages: Vec<ArbData>,
}
