use super::*;
use dqcsim::simulator::Simulator;
//use failure::Error;
//use std::ptr::{null, null_mut};

/// Simulator/accelerator object storage. There can be only one
/// simulator/accelerator per thread.
thread_local! {
    static ACCEL: RefCell<Option<Simulator>> = RefCell::new(None);
}

/// Convenience function for writing functions that operate on the accelerator
/// instance.
/*fn with_accel<T>(
    error: impl FnOnce() -> T,
    call: impl FnOnce(&mut Simulator) -> Result<T, Error>,
) -> T {
    ACCEL.with(|accel| match accel.borrow_mut().as_mut() {
        Some(accel) => match call(accel) {
            Ok(r) => r,
            Err(e) => {
                STATE.with(|state| state.borrow_mut().fail(e.to_string()));
                error()
            }
        },
        None => {
            STATE.with(|state| {
                state
                    .borrow_mut()
                    .fail("Simulation is not running".to_string())
            });
            error()
        }
    })
}*/

/// Constructs a DQCsim simulation.
///
/// The simulation behaves like a quantum accelerator, hence the `dqcs_accel_`
/// prefix for this interface.
///
/// The provided handle is consumed if it is a simulation configuration,
/// regardless of whether simulation construction succeeds. (This has to do
/// with the fact that the log callback closure is not copyable in Rust, and
/// returning ownership to the object store would be inconvenient to say the
/// least.)
#[no_mangle]
pub extern "C" fn dqcs_accel_init(scfg_handle: dqcs_handle_t) -> dqcs_return_t {
    ACCEL.with(|accel| {
        let mut accel = accel.borrow_mut();

        // Fail if a sim is already running.
        if accel.is_some() {
            STATE.with(|state| {
                state
                    .borrow_mut()
                    .fail("A simulation is already running".to_string())
            });
            return dqcs_return_t::DQCS_FAILURE;
        }

        // Try to acquire the sim config object without keeping a reference to
        // `STATE`. While this doesn't happen at the time of writing, the
        // simulator object is allowed to call API callbacks, which are in turn
        // allowed to get a mutable reference to `STATE`, so we must make sure
        // to release our reference before that happens.
        match STATE.with(|state| state.borrow_mut().objects.remove(&scfg_handle)) {
            Some(Object::SimulatorConfiguration(scfg_ob)) => match Simulator::try_from(scfg_ob) {
                Ok(sim) => {
                    accel.replace(sim);
                    dqcs_return_t::DQCS_SUCCESS
                }
                Err(e) => {
                    STATE.with(|state| state.borrow_mut().fail(e.to_string()));
                    dqcs_return_t::DQCS_FAILURE
                }
            },
            Some(ob) => {
                STATE.with(|state| {
                    let mut state = state.borrow_mut();
                    state.objects.insert(scfg_handle, ob);
                    state.fail(format!(
                        "Handle {} is not a simulator configuration",
                        scfg_handle
                    ));
                });
                dqcs_return_t::DQCS_FAILURE
            }
            None => {
                STATE.with(|state| {
                    state
                        .borrow_mut()
                        .fail(format!("Invalid handle {}", scfg_handle))
                });
                dqcs_return_t::DQCS_FAILURE
            }
        }
    })
}

/// Destroys a DQCsim simulation.
///
/// This is a graceful shutdown if possible. Note that a shutdown is normally
/// performed automatically when `libdqcsim.so` is unloaded, so you only need
/// to do this if you want to shut the simulation down before that point.
///
/// This returns failure if no simulation was running.
#[no_mangle]
pub extern "C" fn dqcs_accel_drop() -> dqcs_return_t {
    ACCEL.with(|accel| {
        let mut accel = accel.borrow_mut();
        if accel.is_none() {
            STATE.with(|state| {
                state
                    .borrow_mut()
                    .fail("No simulation was running".to_string())
            });
            dqcs_return_t::DQCS_FAILURE
        } else {
            accel.take();
            dqcs_return_t::DQCS_SUCCESS
        }
    })
}
