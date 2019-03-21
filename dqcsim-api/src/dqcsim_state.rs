use super::*;

/// DQCsim state type, containing either a simulator or a plugin instance.
#[allow(dead_code)] // <-- TODO: remove me
pub enum DQCsimState {
    Simulator(dqcsim::host::simulator::Simulator),
    Plugin, // TODO
}

thread_local! {
    /// DQCsim state storage. This contains the objects owned by DQCsim itself
    /// The difference with API_STATE is that DQCSIM_STATE may own closures
    /// that, when called, can take a mutable reference to API_STATE.
    pub static DQCSIM_STATE: RefCell<Option<DQCsimState>> = RefCell::new(None);
}

/// Convenience function for writing functions that operate on the accelerator
/// (a.k.a. simulator) instance.
pub fn with_accel<T>(
    error: impl FnOnce() -> T,
    call: impl FnOnce(&mut dqcsim::host::simulator::Simulator) -> Result<T>,
) -> T {
    DQCSIM_STATE.with(|dstate| {
        let result = match dstate.borrow_mut().as_mut() {
            Some(DQCsimState::Simulator(sim)) => call(sim),
            Some(_) | None => inv_op("simulation is not running"),
        };
        result_to_api(result, error)
    })
}
