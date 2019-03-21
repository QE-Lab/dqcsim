use super::*;
use dqcsim::simulator::Simulator;

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
    DQCSIM_STATE.with(|dstate| {
        let mut dstate = dstate.borrow_mut();

        // Fail if a sim is already running.
        if dstate.is_some() {
            return result_to_api(
                inv_op("an accelerator or plugin instance has already been created in this thread"),
                || dqcs_return_t::DQCS_FAILURE,
            );
        }

        // Try to acquire the sim config object without keeping a reference to
        // `API_STATE`. While this doesn't happen at the time of writing, the
        // simulator object is allowed to call API callbacks, which are in turn
        // allowed to get a mutable reference to `API_STATE`, so we must make sure
        // to release our reference before that happens.
        let ob = API_STATE.with(|state| state.borrow_mut().objects.remove(&scfg_handle));
        let result = match ob {
            Some(APIObject::SimulatorConfiguration(scfg_ob)) => match Simulator::new(scfg_ob) {
                Ok(sim) => {
                    dstate.replace(DQCsimState::Simulator(sim));
                    Ok(dqcs_return_t::DQCS_SUCCESS)
                }
                Err(e) => Err(e),
            },
            Some(ob) => {
                API_STATE.with(|state| state.borrow_mut().objects.insert(scfg_handle, ob));
                unsup_handle(scfg_handle, "scfg")
            }
            None => inv_handle(scfg_handle),
        };
        result_to_api(result, || dqcs_return_t::DQCS_FAILURE)
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
    DQCSIM_STATE.with(|dstate| {
        let mut dstate = dstate.borrow_mut();
        if let Some(DQCsimState::Simulator(_)) = dstate.as_ref() {
            dstate.take();
            dqcs_return_t::DQCS_SUCCESS
        } else {
            result_to_api(inv_op("no simulation was running"), || {
                dqcs_return_t::DQCS_FAILURE
            })
        }
    })
}

/// Starts a program on the accelerator.
///
/// This is an asynchronous call: nothing happens until `yield()`,
/// `recv()`, or `wait()` is called.
///
/// The `ArbData` handle is optional; if 0 is passed, an empty data object is
/// used. If a handle is passed, it is consumed if and only if the API call
/// succeeds.
#[no_mangle]
pub extern "C" fn dqcs_accel_start(data: dqcs_handle_t) -> dqcs_return_t {
    with_accel(
        || dqcs_return_t::DQCS_FAILURE,
        |accel| {
            take_arb(data, |data| {
                accel
                    .as_mut()
                    .start(data.clone())
                    .map(|_| dqcs_return_t::DQCS_SUCCESS)
                    .map_err(Error::from) // TODO: jeroen
            })
        },
    )
}

/// Waits for the accelerator to finish its current program.
///
/// When this succeeds, the return value of the accelerator's `run()`
/// function is returned in the form of a new handle. When it fails, 0 is
/// returned.
///
/// Deadlocks are detected and prevented by returning an error.
#[no_mangle]
pub extern "C" fn dqcs_accel_wait() -> dqcs_handle_t {
    with_accel(
        || 0,
        |accel| {
            accel
                .as_mut()
                .wait()
                .map(|d| API_STATE.with(|state| state.borrow_mut().push(APIObject::ArbData(d))))
                .map_err(Error::from) // TODO: jeroen
        },
    )
}

/// Sends a message to the accelerator.
///
/// This is an asynchronous call: nothing happens until `yield()`,
/// `recv()`, or `wait()` is called.
///
/// The `ArbData` handle is optional; if 0 is passed, an empty data object is
/// used. If a handle is passed, it is consumed if and only if the API call
/// succeeds.
#[no_mangle]
pub extern "C" fn dqcs_accel_send(data: dqcs_handle_t) -> dqcs_return_t {
    with_accel(
        || dqcs_return_t::DQCS_FAILURE,
        |accel| {
            take_arb(data, |data| {
                accel
                    .as_mut()
                    .send(data.clone())
                    .map(|_| dqcs_return_t::DQCS_SUCCESS)
                    .map_err(Error::from) // TODO: jeroen
            })
        },
    )
}

/// Waits for the accelerator to send a message to us.
///
/// When this succeeds, the received data is returned in the form of a new
/// handle. When it fails, 0 is returned.
///
/// Deadlocks are detected and prevented by returning an error.
#[no_mangle]
pub extern "C" fn dqcs_accel_recv() -> dqcs_handle_t {
    with_accel(
        || 0,
        |accel| {
            accel
                .as_mut()
                .recv()
                .map(|d| API_STATE.with(|state| state.borrow_mut().push(APIObject::ArbData(d))))
                .map_err(Error::from) // TODO: jeroen
        },
    )
}

/// Yields to the accelerator.
///
/// The accelerator simulation runs until it blocks again. This is useful
/// if you want an immediate response to an otherwise asynchronous call
/// through the logging system or some communication channel outside of
/// DQCsim's control.
///
/// This function silently returns immediately if no asynchronous data was
/// pending or if the simulator is waiting for something that has not been
/// sent yet.
#[no_mangle]
pub extern "C" fn dqcs_accel_yield() -> dqcs_return_t {
    with_accel(
        || dqcs_return_t::DQCS_FAILURE,
        |accel| {
            accel
                .as_mut()
                .yield_to_frontend()
                .map(|_| dqcs_return_t::DQCS_SUCCESS)
                .map_err(Error::from) // TODO: jeroen
        },
    )
}

/// Sends an `ArbCmd` message to one of the plugins, referenced by name.
///
/// `ArbCmd`s are executed immediately after yielding to the simulator, so
/// all pending asynchronous calls are flushed and executed *before* the
/// `ArbCmd`.
///
/// When this succeeds, the received data is returned in the form of a new
/// handle. When it fails, 0 is returned.
///
/// The `ArbCmd` handle is consumed if and only if the API call succeeds.
#[no_mangle]
pub extern "C" fn dqcs_accel_arb(name: *const c_char, cmd: dqcs_handle_t) -> dqcs_handle_t {
    with_accel(
        || 0,
        |accel| {
            take_cmd(cmd, |cmd| {
                accel
                    .as_mut()
                    .arb(receive_str(name)?, cmd.clone())
                    .map(|d| API_STATE.with(|state| state.borrow_mut().push(APIObject::ArbData(d))))
                    .map_err(Error::from) // TODO: jeroen
            })
        },
    )
}

/// Sends an `ArbCmd` message to one of the plugins, referenced by index.
///
/// The frontend always has index 0. 1 through N are used for the operators
/// in front to back order (where N is the number of operators). The
/// backend is at index N+1.
///
/// Python-style negative indices are supported. That is, -1 can be used to
/// refer to the backend, -2 to the last operator, and so on.
///
/// `ArbCmd`s are executed immediately after yielding to the simulator, so
/// all pending asynchronous calls are flushed and executed *before* the
/// `ArbCmd`.
///
/// When this succeeds, the received data is returned in the form of a new
/// handle. When it fails, 0 is returned.
///
/// The `ArbCmd` handle is consumed if and only if the API call succeeds.
#[no_mangle]
pub extern "C" fn dqcs_accel_arb_idx(index: ssize_t, cmd: dqcs_handle_t) -> dqcs_handle_t {
    with_accel(
        || 0,
        |accel| {
            take_cmd(cmd, |cmd| {
                accel
                    .as_mut()
                    .arb_idx(index, cmd.clone())
                    .map(|d| API_STATE.with(|state| state.borrow_mut().push(APIObject::ArbData(d))))
                    .map_err(Error::from) // TODO: jeroen
            })
        },
    )
}
