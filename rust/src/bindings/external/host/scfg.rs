use super::*;
use crate::common::{
    log,
    log::{callback::LogCallback, tee_file::TeeFileConfiguration},
};
use std::time::*;

/// Constructs an empty simulation configuration.
///
/// Before the configuration can be used, at least a frontend and a backend
/// plugin configuration must be pushed into it. This can be done with
/// `dqcs_scfg_push_plugin()`. Failing to do this will result in an error when
/// you try to start the simulation.
///
/// The default settings correspond to the defaults of the `dqcsim` command
/// line interface. Refer to its help for more information.
#[no_mangle]
pub extern "C" fn dqcs_scfg_new() -> dqcs_handle_t {
    insert(SimulatorConfiguration::default())
}

/// Appends a plugin to a simulation configuration.
///
/// Both plugin process and plugin thread configuration objects may be used.
/// The handle is consumed by this function, and is thus invalidated, if and
/// only if it is successful.
///
/// Frontend and backend plugins will automatically be inserted at the front
/// and back of the pipeline when the simulation is created. Operators are
/// inserted in front to back order. This function does not provide safeguards
/// against multiple frontends/backends; such errors will only be reported when
/// the simulation is started.
///
/// Note that it is not possible to observe or mutate a plugin configuration
/// once it has been added to a simulator configuration handle. If you want to
/// do this for some reason, you should maintain your own data structures, and
/// only build the DQCsim structures from them when you're done.
#[no_mangle]
pub extern "C" fn dqcs_scfg_push_plugin(scfg: dqcs_handle_t, xcfg: dqcs_handle_t) -> dqcs_return_t {
    api_return_none(|| {
        resolve!(scfg as &mut SimulatorConfiguration);
        take!(xcfg as BoxedPluginConfiguration);
        scfg.plugins.push(xcfg);
        Ok(())
    })
}

/// Configures the random seed that the simulation should use.
///
/// Note that the seed is randomized by default.
#[no_mangle]
pub extern "C" fn dqcs_scfg_seed_set(scfg: dqcs_handle_t, seed: u64) -> dqcs_return_t {
    api_return_none(|| {
        resolve!(scfg as &mut SimulatorConfiguration);
        scfg.seed.value = seed;
        Ok(())
    })
}

/// Returns the configured random seed.
///
/// This function will return 0 when it fails, but this can unfortunately not
/// be reliably distinguished from a seed that was set to 0.
#[no_mangle]
pub extern "C" fn dqcs_scfg_seed_get(scfg: dqcs_handle_t) -> u64 {
    api_return(0, || {
        resolve!(scfg as &SimulatorConfiguration);
        Ok(scfg.seed.value)
    })
}

/// Configures the stderr sink verbosity for a simulation.
///
/// That is, the minimum loglevel that a messages needs to have for it to be
/// printed to stderr.
#[no_mangle]
pub extern "C" fn dqcs_scfg_stderr_verbosity_set(
    scfg: dqcs_handle_t,
    level: dqcs_loglevel_t,
) -> dqcs_return_t {
    api_return_none(|| {
        resolve!(scfg as &mut SimulatorConfiguration);
        scfg.stderr_level = level.into_loglevel_filter()?;
        Ok(())
    })
}

/// Returns the configured stderr sink verbosity for a simulation.
///
/// That is, the minimum loglevel that a messages needs to have for it to be
/// printed to stderr.
#[no_mangle]
pub extern "C" fn dqcs_scfg_stderr_verbosity_get(scfg: dqcs_handle_t) -> dqcs_loglevel_t {
    api_return(dqcs_loglevel_t::DQCS_LOG_INVALID, || {
        resolve!(scfg as &SimulatorConfiguration);
        Ok(scfg.stderr_level.into())
    })
}

/// Configures DQCsim to also output its log messages to a file.
///
/// `verbosity` configures the verbosity level for the file only.
#[no_mangle]
pub extern "C" fn dqcs_scfg_tee(
    scfg: dqcs_handle_t,
    verbosity: dqcs_loglevel_t,
    filename: *const c_char,
) -> dqcs_return_t {
    api_return_none(|| {
        resolve!(scfg as &mut SimulatorConfiguration);
        scfg.tee_files.push(TeeFileConfiguration::new(
            verbosity.into_loglevel_filter()?,
            receive_str(filename)?,
        ));
        Ok(())
    })
}

/// Configures DQCsim to also output its log messages to callback function.
///
/// `verbosity` specifies the minimum importance of a message required for the
/// callback to be called.
///
/// `callback` is the callback function to install. It is always called with
/// the `user_data` pointer to make calling stuff like class member functions
/// or closures possible. The `user_free` function, if non-null, will be called
/// when the callback is uninstalled in any way. If `callback` is null, any
/// current callback is uninstalled instead. For consistency, if `user_free` is
/// non-null while `callback` is null, `user_free` is called immediately, under
/// the assumption that the caller has allocated resources unbeknownst that the
/// callback it's trying to install is null.
///
/// **NOTE: both `callback` and `user_free` may be called from a thread spawned
/// by the simulator. Calling any API calls from the callback is therefore
/// undefined behavior!**
///
/// The callback takes the following arguments:
///  - `void*`: user defined data.
///  - `const char*`: log message string, excluding metadata.
///  - `const char*`: name assigned to the logger that was used to produce the
///    message (= "dqcsim" or a plugin name).
///  - `dqcs_loglevel_t`: the verbosity level that the message was logged with.
///  - `const char*`: string representing the source of the log message, or
///    `NULL` when no source is known.
///  - `const char*`: string containing the filename of the source that
///    generated the message, or `NULL` when no source is known.
///  - `uint32_t`: line number within the aforementioned file, or 0 if not
///    known.
///  - `uint64_t`: Time in seconds since the Unix epoch.
///  - `uint32_t`: Additional time in nanoseconds since the aforementioned.
///  - `uint32_t`: PID of the generating process.
///  - `uint64_t`: TID of the generating thread.
///
/// If an internal log record is particularly malformed and cannot be coerced
/// into the above (nul bytes in the strings, invalid timestamp, whatever) the
/// message is silently ignored.
///
/// The primary use of this callback is to pipe DQCsim's messages to an
/// external logging framework. When you do this, you probably also want to
/// call `dqcs_scfg_stderr_verbosity_set(handle, DQCS_LOG_OFF)` to prevent
/// DQCsim from writing the messages to stderr itself.
#[no_mangle]
#[allow(unused_must_use)]
pub extern "C" fn dqcs_scfg_log_callback(
    scfg: dqcs_handle_t,
    verbosity: dqcs_loglevel_t,
    callback: Option<
        extern "C" fn(
            user_data: *mut c_void,
            message: *const c_char,
            logger: *const c_char,
            level: dqcs_loglevel_t,
            module: *const c_char,
            file: *const c_char,
            line: u32,
            time_s: u64,
            time_ns: u32,
            pid: u32,
            tid: u64,
        ),
    >,
    user_free: Option<extern "C" fn(user_data: *mut c_void)>,
    user_data: *mut c_void,
) -> dqcs_return_t {
    api_return_none(|| {
        let data = CallbackUserData::new(user_free, user_data);
        resolve!(scfg as &mut SimulatorConfiguration);

        if let Some(callback) = callback {
            scfg.log_callback = Some(LogCallback {
                callback: Box::new(move |record: &log::LogRecord| {
                    || -> Result<()> {
                        let ts_sec;
                        let ts_nano;
                        if let Ok(ts) = record.timestamp().duration_since(SystemTime::UNIX_EPOCH) {
                            ts_sec = ts.as_secs();
                            ts_nano = ts.subsec_nanos();
                        } else {
                            ts_sec = 0;
                            ts_nano = 0;
                        }
                        let payload = CString::new(record.payload())?;
                        let logger = CString::new(record.logger())?;
                        let module_path = record
                            .module_path()
                            .map(CString::new)
                            .map_or(Ok(None), |v| v.map(Some))?;
                        let file = record
                            .file()
                            .map(CString::new)
                            .map_or(Ok(None), |v| v.map(Some))?;
                        callback(
                            data.data(),
                            payload.as_ptr(),
                            logger.as_ptr(),
                            record.level().into(),
                            module_path.as_ref().map(|x| x.as_ptr()).unwrap_or(null()),
                            file.as_ref().map(|x| x.as_ptr()).unwrap_or(null()),
                            record.line().unwrap_or(0),
                            ts_sec,
                            ts_nano,
                            record.process(),
                            record.thread(),
                        );
                        Ok(())
                    }();
                }),
                filter: verbosity.into_loglevel_filter()?,
            });
        } else {
            scfg.log_callback = None;
        }
        Ok(())
    })
}

/// Configures the logging verbosity for DQCsim's own messages.
#[no_mangle]
pub extern "C" fn dqcs_scfg_dqcsim_verbosity_set(
    scfg: dqcs_handle_t,
    level: dqcs_loglevel_t,
) -> dqcs_return_t {
    api_return_none(|| {
        resolve!(scfg as &mut SimulatorConfiguration);
        scfg.dqcsim_level = level.into_loglevel_filter()?;
        Ok(())
    })
}

/// Returns the configured verbosity for DQCsim's own messages.
#[no_mangle]
pub extern "C" fn dqcs_scfg_dqcsim_verbosity_get(scfg: dqcs_handle_t) -> dqcs_loglevel_t {
    api_return(dqcs_loglevel_t::DQCS_LOG_INVALID, || {
        resolve!(scfg as &SimulatorConfiguration);
        Ok(scfg.dqcsim_level.into())
    })
}

/// Sets the path style used when writing reproduction files.
#[no_mangle]
pub extern "C" fn dqcs_scfg_repro_path_style_set(
    scfg: dqcs_handle_t,
    path_style: dqcs_path_style_t,
) -> dqcs_return_t {
    api_return_none(|| {
        resolve!(scfg as &mut SimulatorConfiguration);
        let path_style: Option<ReproductionPathStyle> = path_style.into();
        scfg.reproduction_path_style =
            Some(path_style.ok_or_else(oe_inv_arg("invalid path style"))?);
        Ok(())
    })
}

/// Returns the path style used when writing reproduction files.
#[no_mangle]
pub extern "C" fn dqcs_scfg_repro_path_style_get(scfg: dqcs_handle_t) -> dqcs_path_style_t {
    api_return(dqcs_path_style_t::DQCS_PATH_STYLE_INVALID, || {
        resolve!(scfg as &SimulatorConfiguration);
        Ok(scfg
            .reproduction_path_style
            .ok_or_else(oe_inv_arg(
                "the reproduction system is disabled for this configuration",
            ))?
            .into())
    })
}

/// Disables the reproduction logging system.
///
/// Calling this will disable the warnings printed when a simulation that
/// cannot be reproduced is constructed.
#[no_mangle]
pub extern "C" fn dqcs_scfg_repro_disable(scfg: dqcs_handle_t) -> dqcs_return_t {
    api_return_none(|| {
        resolve!(scfg as &mut SimulatorConfiguration);
        scfg.reproduction_path_style = None;
        Ok(())
    })
}
