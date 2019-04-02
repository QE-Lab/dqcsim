use super::*;

/// Constructs a DQCsim simulation.
///
/// The provided handle is consumed if it is a simulation configuration,
/// regardless of whether simulation construction succeeds.
#[no_mangle]
pub extern "C" fn dqcs_sim_init(scfg: dqcs_handle_t) -> dqcs_handle_t {
    api_return(0, || {
        take!(scfg as SimulatorConfiguration);
        API_STATE.with(|state| state.borrow().thread_locals_assert_free())?;
        let sim = Simulator::new(scfg)?;
        let sim = insert(sim);
        API_STATE.with(|state| state.borrow_mut().thread_locals_claim(sim).unwrap());
        Ok(sim)
    })
}

/// Yields to the simulator.
///
/// The simulation runs until it blocks again. This is useful if you want an
/// immediate response to an otherwise asynchronous call through the logging
/// system or some communication channel outside of DQCsim's control.
///
/// This function silently returns immediately if no asynchronous data was
/// pending or if the simulator is waiting for something that has not been
/// sent yet.
#[no_mangle]
pub extern "C" fn dqcs_sim_yield(sim: dqcs_handle_t) -> dqcs_return_t {
    api_return_none(|| {
        resolve!(sim as &mut Simulator);
        sim.as_mut().yield_to_frontend()?;
        Ok(())
    })
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
pub extern "C" fn dqcs_sim_arb(
    sim: dqcs_handle_t,
    name: *const c_char,
    cmd: dqcs_handle_t,
) -> dqcs_handle_t {
    api_return(0, || {
        resolve!(sim as &mut Simulator);
        resolve!(cmd as pending ArbCmd);
        let cmd_ob = {
            let x: &ArbCmd = cmd.as_ref().unwrap();
            x.clone()
        };
        let data = sim.as_mut().arb(receive_str(name)?, cmd_ob)?;
        take!(resolved cmd as ArbCmd);
        let _ = cmd;
        Ok(insert(data))
    })
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
pub extern "C" fn dqcs_sim_arb_idx(
    sim: dqcs_handle_t,
    index: ssize_t,
    cmd: dqcs_handle_t,
) -> dqcs_handle_t {
    api_return(0, || {
        resolve!(sim as &mut Simulator);
        resolve!(cmd as pending ArbCmd);
        let cmd_ob = {
            let x: &ArbCmd = cmd.as_ref().unwrap();
            x.clone()
        };
        let data = sim.as_mut().arb_idx(index, cmd_ob)?;
        take!(resolved cmd as ArbCmd);
        let _ = cmd;
        Ok(insert(data))
    })
}

/// Queries the implementation name of a plugin, referenced by instance
/// name.
///
/// On success, this **returns a newly allocated string containing the
/// name. Free it with `free()` when you're done with it to avoid memory
/// leaks.** On failure (i.e., the handle is invalid) this returns `NULL`.
#[no_mangle]
pub extern "C" fn dqcs_sim_get_name(sim: dqcs_handle_t, name: *const c_char) -> *mut c_char {
    api_return_string(|| {
        resolve!(sim as &Simulator);
        Ok(sim
            .as_ref()
            .get_metadata(receive_str(name)?)?
            .get_name()
            .to_string())
    })
}

/// Queries the implementation name of a plugin, referenced by index.
///
/// On success, this **returns a newly allocated string containing the
/// name. Free it with `free()` when you're done with it to avoid memory
/// leaks.** On failure (i.e., the handle is invalid) this returns `NULL`.
#[no_mangle]
pub extern "C" fn dqcs_sim_get_name_idx(sim: dqcs_handle_t, index: ssize_t) -> *mut c_char {
    api_return_string(|| {
        resolve!(sim as &Simulator);
        Ok(sim.as_ref().get_metadata_idx(index)?.get_name().to_string())
    })
}

/// Queries the author of a plugin, referenced by instance name.
///
/// On success, this **returns a newly allocated string containing the
/// author. Free it with `free()` when you're done with it to avoid memory
/// leaks.** On failure (i.e., the handle is invalid) this returns `NULL`.
#[no_mangle]
pub extern "C" fn dqcs_sim_get_author(sim: dqcs_handle_t, name: *const c_char) -> *mut c_char {
    api_return_string(|| {
        resolve!(sim as &Simulator);
        Ok(sim
            .as_ref()
            .get_metadata(receive_str(name)?)?
            .get_author()
            .to_string())
    })
}

/// Queries the author of a plugin, referenced by index.
///
/// On success, this **returns a newly allocated string containing the
/// author. Free it with `free()` when you're done with it to avoid memory
/// leaks.** On failure (i.e., the handle is invalid) this returns `NULL`.
#[no_mangle]
pub extern "C" fn dqcs_sim_get_author_idx(sim: dqcs_handle_t, index: ssize_t) -> *mut c_char {
    api_return_string(|| {
        resolve!(sim as &Simulator);
        Ok(sim
            .as_ref()
            .get_metadata_idx(index)?
            .get_author()
            .to_string())
    })
}

/// Queries the version of a plugin, referenced by instance name.
///
/// On success, this **returns a newly allocated string containing the
/// version. Free it with `free()` when you're done with it to avoid memory
/// leaks.** On failure (i.e., the handle is invalid) this returns `NULL`.
#[no_mangle]
pub extern "C" fn dqcs_sim_get_version(sim: dqcs_handle_t, name: *const c_char) -> *mut c_char {
    api_return_string(|| {
        resolve!(sim as &Simulator);
        Ok(sim
            .as_ref()
            .get_metadata(receive_str(name)?)?
            .get_version()
            .to_string())
    })
}

/// Queries the version of a plugin, referenced by index.
///
/// On success, this **returns a newly allocated string containing the
/// version. Free it with `free()` when you're done with it to avoid memory
/// leaks.** On failure (i.e., the handle is invalid) this returns `NULL`.
#[no_mangle]
pub extern "C" fn dqcs_sim_get_version_idx(sim: dqcs_handle_t, index: ssize_t) -> *mut c_char {
    api_return_string(|| {
        resolve!(sim as &Simulator);
        Ok(sim
            .as_ref()
            .get_metadata_idx(index)?
            .get_version()
            .to_string())
    })
}
