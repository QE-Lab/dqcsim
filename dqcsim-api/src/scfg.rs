use super::*;
use dqcsim::log::tee_file::TeeFile;
use failure::Error;

/// Convenience function for writing functions that operate on
/// `SimulatorConfiguration`s.
fn with_scfg<T>(
    handle: dqcs_handle_t,
    error: impl FnOnce() -> T,
    call: impl FnOnce(&mut SimulatorConfiguration) -> Result<T, Error>,
) -> T {
    with_state(error, |mut state| match state.objects.get_mut(&handle) {
        Some(Object::SimulatorConfiguration(x)) => call(x),
        Some(_) => Err(APIError::UnsupportedHandle(handle).into()),
        None => Err(APIError::InvalidHandle(handle).into()),
    })
}

/// Appends a plugin to a simulation configuration.
///
/// Frontend and backend plugins will automatically be inserted at the front
/// and back of the pipeline when the simulation is created. Operators are
/// inserted in front to back order.
///
/// The `PluginConfiguration` handle is consumed by this function, and is thus
/// invalidated, if and only if it is successful.
///
/// Note that it is not possible to observe or mutate a plugin configuration
/// once it has been added to a simulator configuration handle. If you want to
/// do this for some reason, you should maintain your own data structures, and
/// only build the DQCsim structures from them when you're done.
#[no_mangle]
pub extern "C" fn dqcs_scfg_push_plugin(
    handle: dqcs_handle_t,
    pcfg_handle: dqcs_handle_t,
) -> dqcs_return_t {
    with_state(
        || dqcs_return_t::DQCS_FAILURE,
        |mut state| match state.objects.remove(&pcfg_handle) {
            Some(Object::PluginConfiguration(pcfg_ob)) => match state.objects.get_mut(&handle) {
                Some(Object::SimulatorConfiguration(scfg)) => {
                    scfg.plugins.push(pcfg_ob);
                    Ok(dqcs_return_t::DQCS_SUCCESS)
                }
                Some(_) => {
                    state
                        .objects
                        .insert(pcfg_handle, Object::PluginConfiguration(pcfg_ob));
                    Err(APIError::UnsupportedHandle(handle).into())
                }
                None => {
                    state
                        .objects
                        .insert(pcfg_handle, Object::PluginConfiguration(pcfg_ob));
                    Err(APIError::InvalidHandle(handle).into())
                }
            },
            Some(ob) => {
                state.objects.insert(pcfg_handle, ob);
                Err(APIError::UnsupportedHandle(pcfg_handle).into())
            }
            None => Err(APIError::InvalidHandle(pcfg_handle).into()),
        },
    )
}

/// Configures the random seed that the simulation should use.
///
/// Note that the seed is randomized by default.
#[no_mangle]
pub extern "C" fn dqcs_scfg_seed_set(handle: dqcs_handle_t, seed: u64) -> dqcs_return_t {
    with_scfg(
        handle,
        || dqcs_return_t::DQCS_FAILURE,
        |sim| {
            sim.seed.value = seed;
            Ok(dqcs_return_t::DQCS_SUCCESS)
        },
    )
}

/// Returns the configured random seed.
///
/// This function will return 0 when it fails, but this can unfortunately not
/// be reliably distinguished from a seed that was set to 0.
#[no_mangle]
pub extern "C" fn dqcs_scfg_seed_get(handle: dqcs_handle_t) -> u64 {
    with_scfg(handle, || 0, |sim| Ok(sim.seed.value))
}

/// Configures the logging verbosity for DQCsim's own messages.
#[no_mangle]
pub extern "C" fn dqcs_scfg_dqcsim_verbosity_set(
    handle: dqcs_handle_t,
    level: dqcs_loglevel_t,
) -> dqcs_return_t {
    with_scfg(
        handle,
        || dqcs_return_t::DQCS_FAILURE,
        |sim| {
            sim.dqcsim_level = level.into_loglevel_filter()?;
            Ok(dqcs_return_t::DQCS_SUCCESS)
        },
    )
}

/// Returns the configured verbosity for DQCsim's own messages.
#[no_mangle]
pub extern "C" fn dqcs_scfg_dqcsim_verbosity_get(handle: dqcs_handle_t) -> dqcs_loglevel_t {
    with_scfg(
        handle,
        || dqcs_loglevel_t::DQCS_LOG_INVALID,
        |sim| Ok(sim.dqcsim_level.into()),
    )
}

/// Configures DQCsim to also output its log messages to a file.
///
/// `verbosity` configures the verbosity level for the file only.
#[no_mangle]
pub extern "C" fn dqcs_scfg_tee(
    handle: dqcs_handle_t,
    verbosity: dqcs_loglevel_t,
    filename: *const c_char,
) -> dqcs_return_t {
    with_scfg(
        handle,
        || dqcs_return_t::DQCS_FAILURE,
        |sim| {
            sim.tee_files.push(TeeFile::new(
                verbosity.into_loglevel_filter()?,
                receive_str(filename)?,
            ));
            Ok(dqcs_return_t::DQCS_SUCCESS)
        },
    )
}

/// Configures the stderr sink verbosity for a simulation.
///
/// That is, the minimum loglevel that a messages needs to have for it to be
/// printed to stderr.
#[no_mangle]
pub extern "C" fn dqcs_scfg_stderr_verbosity_set(
    handle: dqcs_handle_t,
    level: dqcs_loglevel_t,
) -> dqcs_return_t {
    with_scfg(
        handle,
        || dqcs_return_t::DQCS_FAILURE,
        |sim| {
            sim.stderr_level = level.into_loglevel_filter()?;
            Ok(dqcs_return_t::DQCS_SUCCESS)
        },
    )
}

/// Returns the configured verbosity for DQCsim's own messages.
///
/// That is, the minimum loglevel that a messages needs to have for it to be
/// printed to stderr.
#[no_mangle]
pub extern "C" fn dqcs_scfg_stderr_verbosity_get(handle: dqcs_handle_t) -> dqcs_loglevel_t {
    with_scfg(
        handle,
        || dqcs_loglevel_t::DQCS_LOG_INVALID,
        |sim| Ok(sim.stderr_level.into()),
    )
}
