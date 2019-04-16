use crate::common::{error::Result, types::ArbData};

/// Trait containing the primitive operations for an accelerator.
pub trait Accelerator {
    /// Starts a program on the accelerator.
    fn start(&mut self, args: impl Into<ArbData>) -> Result<()>;

    /// Waits for the accelerator to finish its current program.
    fn wait(&mut self) -> Result<ArbData>;

    /// Sends a message to the accelerator.
    fn send(&mut self, args: impl Into<ArbData>) -> Result<()>;

    /// Waits for the accelerator to send a message to us.
    fn recv(&mut self) -> Result<ArbData>;
}
