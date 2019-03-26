use crate::common::protocol::{ArbData, Cycles, QubitRef, SequenceNumber};
use serde::{Deserialize, Serialize};

/// Gatestream responses/upstream messages.
#[derive(Debug, Serialize, Deserialize)]
pub enum GatestreamUp {
    /// Acknowledges one or more requests.
    ///
    /// This message indicates that all commands up to the given sequence
    /// number have been executed. It must be sent by the plugin as soon as
    /// possible, but does NOT have to be sent for every request necessarily.
    CompletedUpTo(SequenceNumber),

    /// Indicates that the message with the specified sequence number failed.
    Failure(SequenceNumber, String),

    /// Specifies that the specified qubit was measured.
    ///
    /// This may only be sent in response to a gate that contains the
    /// referenced qubit in its `measured` set. This is to be checked when the
    /// subsequent `CompletedUpTo` message is received, as this message does
    /// not contain the sequence number itself.
    Measured(QubitRef, bool, ArbData),

    /// Indicates that the simulation was advanced by the specified number of
    /// cycles.
    Advanced(Cycles),

    /// Indicates that a `GatestreamDown::ArbRequest` was executed successfully.
    ArbSuccess(ArbData),

    /// Indicates that a `GatestreamDown::ArbRequest` failed.
    ArbFailure(String),
}
