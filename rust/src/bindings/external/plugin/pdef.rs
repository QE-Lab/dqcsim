use super::*;

/// Creates a new `PluginDefinition` object.
///
/// Plugin definitions contain the callback functions/closures that define the
/// functionality of a plugin. They also contain some metadata to identify the
/// implementation, in the form of a name, author, and version string, that
/// must be specified when the definition is constructed. The callback
/// functions/closures are initialized to sane defaults for the requested
/// plugin type, but obviously one or more of these should be overridden to
/// make the plugin do something.
///
/// Once a definition object has been built, it can be used to spawn a plugin
/// thread or run a plugin in the main thread, given a DQCsim server URL for it
/// to connect to.
#[no_mangle]
pub extern "C" fn dqcs_pdef_new(
    typ: dqcs_plugin_type_t,
    name: *const c_char,
    author: *const c_char,
    version: *const c_char,
) -> dqcs_handle_t {
    api_return(0, || {
        let typ: Result<PluginType> = typ.into();
        Ok(insert(PluginDefinition::new(
            typ?,
            PluginMetadata::new(
                receive_optional_str(name)?
                    .filter(|x| !x.is_empty())
                    .ok_or_else(oe_inv_arg("plugin name is required"))?,
                receive_optional_str(author)?
                    .filter(|x| !x.is_empty())
                    .ok_or_else(oe_inv_arg("author name is required"))?,
                receive_optional_str(version)?
                    .filter(|x| !x.is_empty())
                    .ok_or_else(oe_inv_arg("version string is required"))?,
            ),
        )))
    })
}

/// Returns the plugin type for the given plugin definition object.
#[no_mangle]
pub extern "C" fn dqcs_pdef_type(pdef: dqcs_handle_t) -> dqcs_plugin_type_t {
    api_return(dqcs_plugin_type_t::DQCS_PTYPE_INVALID, || {
        resolve!(pdef as &PluginDefinition);
        Ok(pdef.get_type().into())
    })
}

/// Returns the plugin name for the given plugin definition object.
///
/// On success, this **returns a newly allocated string containing the JSON
/// string. Free it with `free()` when you're done with it to avoid memory
/// leaks.** On failure, this returns `NULL`.
#[no_mangle]
pub extern "C" fn dqcs_pdef_name(pdef: dqcs_handle_t) -> *mut c_char {
    api_return_string(|| {
        resolve!(pdef as &PluginDefinition);
        Ok(pdef.get_metadata().get_name().to_string())
    })
}

/// Returns the plugin author for the given plugin definition object.
///
/// On success, this **returns a newly allocated string containing the JSON
/// string. Free it with `free()` when you're done with it to avoid memory
/// leaks.** On failure, this returns `NULL`.
#[no_mangle]
pub extern "C" fn dqcs_pdef_author(pdef: dqcs_handle_t) -> *mut c_char {
    api_return_string(|| {
        resolve!(pdef as &PluginDefinition);
        Ok(pdef.get_metadata().get_author().to_string())
    })
}

/// Returns the plugin version for the given plugin definition object.
///
/// On success, this **returns a newly allocated string containing the JSON
/// string. Free it with `free()` when you're done with it to avoid memory
/// leaks.** On failure, this returns `NULL`.
#[no_mangle]
pub extern "C" fn dqcs_pdef_version(pdef: dqcs_handle_t) -> *mut c_char {
    api_return_string(|| {
        resolve!(pdef as &PluginDefinition);
        Ok(pdef.get_metadata().get_version().to_string())
    })
}

/// Sets the user logic initialization callback.
///
/// This is always called before any of the other callbacks are run. The
/// downstream plugin has already been initialized at this stage, so it is
/// legal to send it commands.
///
/// The default behavior is no-op.
///
/// Besides the common arguments, the callback receives a handle to an
/// `ArbCmd` queue (`dqcs_cq_*`, `dqcs_cmd_*`, and `dqcs_arb_*` interfaces)
/// containing user-defined initialization commands. This is a borrowed
/// handle; the caller will delete it.
///
/// The callback can return an error by setting an error message using
/// `dqcs_error_set()` and returning `DQCS_FAILURE`. Otherwise, it should
/// return `DQCS_SUCCESS`.
#[no_mangle]
pub extern "C" fn dqcs_pdef_set_initialize_cb(
    pdef: dqcs_handle_t,
    callback: Option<
        extern "C" fn(
            user_data: *mut c_void,
            state: dqcs_plugin_state_t,
            init_cmds: dqcs_handle_t,
        ) -> dqcs_return_t,
    >,
    user_free: Option<extern "C" fn(user_data: *mut c_void)>,
    user_data: *mut c_void,
) -> dqcs_return_t {
    api_return_none(|| {
        let data = CallbackUserData::new(user_free, user_data);
        let callback = callback.ok_or_else(oe_inv_arg("callback cannot be null"))?;
        resolve!(pdef as &mut PluginDefinition);
        pdef.initialize = Box::new(
            move |state: &mut PluginState, init_cmds: Vec<ArbCmd>| -> Result<()> {
                let init_cmds: ArbCmdQueue = init_cmds.into_iter().collect();
                let init_cmds = insert(init_cmds);
                let result = cb_return_none(callback(data.data(), state.into(), init_cmds));
                delete!(init_cmds);
                result
            },
        );
        Ok(())
    })
}

/// Sets the user logic drop/cleanup callback.
///
/// This is called when a plugin is gracefully terminated. It is not
/// recommended to execute any downstream instructions at this time, but it
/// is supported in case this is really necessary.
///
/// The default behavior is no-op.
///
/// The callback can return an error by setting an error message using
/// `dqcs_error_set()` and returning `DQCS_FAILURE`. Otherwise, it should
/// return `DQCS_SUCCESS`.
#[no_mangle]
pub extern "C" fn dqcs_pdef_set_drop_cb(
    pdef: dqcs_handle_t,
    callback: Option<
        extern "C" fn(user_data: *mut c_void, state: dqcs_plugin_state_t) -> dqcs_return_t,
    >,
    user_free: Option<extern "C" fn(user_data: *mut c_void)>,
    user_data: *mut c_void,
) -> dqcs_return_t {
    api_return_none(|| {
        let data = CallbackUserData::new(user_free, user_data);
        let callback = callback.ok_or_else(oe_inv_arg("callback cannot be null"))?;
        resolve!(pdef as &mut PluginDefinition);
        pdef.drop = Box::new(move |state: &mut PluginState| -> Result<()> {
            cb_return_none(callback(data.data(), state.into()))
        });
        Ok(())
    })
}

/// Sets the run callback for frontends.
///
/// This is called in response to a `start()` host API call. The return
/// value is returned through the `wait()` host API call.
///
/// The default behavior is to fail with a "not implemented" error;
/// frontends backends should always override this. This callback is never
/// called for operator or backend plugins.
///
/// Besides the common arguments, the callback receives a handle to an
/// `ArbData` object containing the data that the host passed to `start()`.
/// This is a borrowed handle; the caller will delete it.
///
/// When the run callback is successful, it should return a valid `ArbData`
/// handle. This can be the same as the argument, but it can also be a new
/// object. This `ArbData` is returned to the host through `wait()`. This
/// `ArbData` object is deleted after the callback completes.
///
/// The callback can return an error by setting an error message using
/// `dqcs_error_set()` and returning 0. Otherwise, it should return a
/// valid `ArbData` handle.
#[no_mangle]
pub extern "C" fn dqcs_pdef_set_run_cb(
    pdef: dqcs_handle_t,
    callback: Option<
        extern "C" fn(
            user_data: *mut c_void,
            state: dqcs_plugin_state_t,
            args: dqcs_handle_t,
        ) -> dqcs_handle_t,
    >,
    user_free: Option<extern "C" fn(user_data: *mut c_void)>,
    user_data: *mut c_void,
) -> dqcs_return_t {
    api_return_none(|| {
        let data = CallbackUserData::new(user_free, user_data);
        let callback = callback.ok_or_else(oe_inv_arg("callback cannot be null"))?;
        resolve!(pdef as &mut PluginDefinition);
        if pdef.get_type() != PluginType::Frontend {
            return inv_op("the run() callback is only supported for frontends");
        }
        pdef.run = Box::new(
            move |state: &mut PluginState, args: ArbData| -> Result<ArbData> {
                let args = insert(args);
                let result =
                    cb_return(0, callback(data.data(), state.into(), args)).and_then(|arb| {
                        take!(arb as ArbData);
                        Ok(arb)
                    });
                delete!(args);
                result
            },
        );
        Ok(())
    })
}

/// Sets the qubit allocation callback for operators and backends.
///
/// The default for operators is to pass through to
/// `dqcs_plugin_allocate()`. The default for backends is no-op. This
/// callback is never called for frontend plugins.
///
/// Besides the common arguments, the callback receives a handle to a qubit
/// set containing the references that are to be used for the
/// to-be-allocated qubits and an `ArbCmd` queue containing user-defined
/// commands to optionally augment the behavior of the qubits. These are
/// borrowed handles; the caller will delete them.
///
/// The callback can return an error by setting an error message using
/// `dqcs_error_set()` and returning `DQCS_FAILURE`. Otherwise, it should
/// return `DQCS_SUCCESS`.
#[no_mangle]
pub extern "C" fn dqcs_pdef_set_allocate_cb(
    pdef: dqcs_handle_t,
    callback: Option<
        extern "C" fn(
            user_data: *mut c_void,
            state: dqcs_plugin_state_t,
            qubits: dqcs_handle_t,
            alloc_cmds: dqcs_handle_t,
        ) -> dqcs_return_t,
    >,
    user_free: Option<extern "C" fn(user_data: *mut c_void)>,
    user_data: *mut c_void,
) -> dqcs_return_t {
    api_return_none(|| {
        let data = CallbackUserData::new(user_free, user_data);
        let callback = callback.ok_or_else(oe_inv_arg("callback cannot be null"))?;
        resolve!(pdef as &mut PluginDefinition);
        if pdef.get_type() == PluginType::Frontend {
            return inv_op("the allocate() callback is not supported for frontends");
        }
        pdef.allocate = Box::new(
            move |state: &mut PluginState,
                  qubits: Vec<QubitRef>,
                  alloc_cmds: Vec<ArbCmd>|
                  -> Result<()> {
                let qubits: QubitReferenceSet = qubits.into_iter().collect();
                let qubits = insert(qubits);
                let alloc_cmds: ArbCmdQueue = alloc_cmds.into_iter().collect();
                let alloc_cmds = insert(alloc_cmds);
                let result =
                    cb_return_none(callback(data.data(), state.into(), qubits, alloc_cmds));
                delete!(qubits);
                delete!(alloc_cmds);
                result
            },
        );
        Ok(())
    })
}

/// Sets the qubit deallocation callback for operators and backends.
///
/// The default for operators is to pass through to `dqcs_plugin_free()`.
/// The default for backends is no-op. This callback is never called for
/// frontend plugins.
///
/// Besides the common arguments, the callback receives a handle to a qubit
/// set containing the qubits that are to be freed. This is a borrowed
/// handle; the caller will delete it.
///
/// The callback can return an error by setting an error message using
/// `dqcs_error_set()` and returning `DQCS_FAILURE`. Otherwise, it should
/// return `DQCS_SUCCESS`.
#[no_mangle]
pub extern "C" fn dqcs_pdef_set_free_cb(
    pdef: dqcs_handle_t,
    callback: Option<
        extern "C" fn(
            user_data: *mut c_void,
            state: dqcs_plugin_state_t,
            qubits: dqcs_handle_t,
        ) -> dqcs_return_t,
    >,
    user_free: Option<extern "C" fn(user_data: *mut c_void)>,
    user_data: *mut c_void,
) -> dqcs_return_t {
    api_return_none(|| {
        let data = CallbackUserData::new(user_free, user_data);
        let callback = callback.ok_or_else(oe_inv_arg("callback cannot be null"))?;
        resolve!(pdef as &mut PluginDefinition);
        if pdef.get_type() == PluginType::Frontend {
            return inv_op("the free() callback is not supported for frontends");
        }
        pdef.free = Box::new(
            move |state: &mut PluginState, qubits: Vec<QubitRef>| -> Result<()> {
                let qubits: QubitReferenceSet = qubits.into_iter().collect();
                let qubits = insert(qubits);
                let result = cb_return_none(callback(data.data(), state.into(), qubits));
                delete!(qubits);
                result
            },
        );
        Ok(())
    })
}

/// Sets the gate execution callback for operators and backends.
///
/// Besides the common arguments, the callback receives a handle to the
/// to-be-executed gate. This is a borrowed handle; the caller will delete
/// it.
///
/// The callback must return one of the following things:
///
///  - a valid handle to a measurement set, created using
///    `dqcs_mset_new()` (this object is automatically deleted after the
///    callback returns);
///  - a valid handle to a single qubit measurement, created using
///    `dqcs_meas_new()` (this object is automatically deleted after the
///    callback returns);
///  - the handle to the supplied gate, a shortcut for not returning any
///    measurements (this is less clear than returning an empty measurement
///    set, but slightly faster); or
///  - 0 to report an error, after calling the error string using
///    `dqcs_set_error()`.
///
/// Backend plugins must return a measurement result set containing exactly
/// those qubits specified in the measurement set. For operators, however,
/// the story is more complicated. Let's say we want to make a silly
/// operator that inverts all measurements. The trivial way to do
/// this would be to forward the gate, query all the measurement results
/// using `dqcs_plugin_get_measurement()`, invert them, stick them in a
/// measurement result set, and return that result set. However, this
/// approach is not very efficient, because `dqcs_plugin_get_measurement()`
/// has to wait for all downstream plugins to finish executing the gate,
/// forcing the OS to switch threads, etc. Instead, operators are allowed
/// to return only a subset (or none) of the measured qubits, as long as
/// they return the measurements as they arrive through the
/// `modify_measurement()` callback.
///
/// The default implementation for this callback for operators is to pass
/// the gate through to the downstream plugin and return an empty set of
/// measurements. Combined with the default implementation of
/// `modify_measurement()`, this behavior is sane. Backends must override
/// this callback; the default is to return a not-implemented error.
///
/// Note that for our silly example operator, the default behavior for this
/// function is sufficient; you'd only have to override
/// `modify_measurement()` to, well, modify the measurements.
#[no_mangle]
pub extern "C" fn dqcs_pdef_set_gate_cb(
    pdef: dqcs_handle_t,
    callback: Option<
        extern "C" fn(
            user_data: *mut c_void,
            state: dqcs_plugin_state_t,
            gate: dqcs_handle_t,
        ) -> dqcs_handle_t,
    >,
    user_free: Option<extern "C" fn(user_data: *mut c_void)>,
    user_data: *mut c_void,
) -> dqcs_return_t {
    api_return_none(|| {
        let data = CallbackUserData::new(user_free, user_data);
        let callback = callback.ok_or_else(oe_inv_arg("callback cannot be null"))?;
        resolve!(pdef as &mut PluginDefinition);
        if pdef.get_type() == PluginType::Frontend {
            return inv_op("the gate() callback is not supported for frontends");
        }
        pdef.gate = Box::new(
            move |state: &mut PluginState, gate: Gate| -> Result<Vec<QubitMeasurementResult>> {
                let gate_handle = insert(gate);
                let result = cb_return(0, callback(data.data(), state.into(), gate_handle))
                    .and_then(|result| {
                        if result == gate_handle {
                            Ok(vec![])
                        } else {
                            take!(result as QubitMeasurementResultSet);
                            Ok(result.into_iter().map(|(_, m)| m).collect())
                        }
                    });
                delete!(gate_handle);
                result
            },
        );
        Ok(())
    })
}

/// Sets the measurement modification callback for operators.
///
/// This callback is called for every measurement result received from the
/// downstream plugin, and returns the measurements that should be reported
/// to the upstream plugin. Note that the results from our plugin's
/// `dqcs_plugin_get_measurement()` and friends are consistent with the
/// results received from downstream; they are not affected by this
/// function.
///
/// The callback takes a handle to a single qubit measurement object as an
/// argument, and must return one of the following things:
///
///  - a valid handle to a measurement set, created using
///    `dqcs_mset_new()` (this object is automatically deleted after the
///    callback returns);
///  - a valid handle to a single qubit measurement object, which may or
///    may not be the supplied one (this object is automatically deleted
///    after the callback returns); or
///  - 0 to report an error, after calling the error string using
///    `dqcs_set_error()`.
///
/// This callback is somewhat special in that it is not allowed to call
/// any plugin command other than logging and the pseudorandom number
/// generator functions. This is because this function is called
/// asynchronously with respect to the downstream functions, making the
/// timing of these calls non-deterministic based on operating system
/// scheduling.
///
/// Note that while this function is called for only a single measurement
/// at a time, it is allowed to produce a vector of measurements. This
/// allows you to cancel propagation of the measurement by returning an
/// empty vector, to just modify the measurement data itself, or to
/// generate additional measurements from a single measurement. However,
/// if you need to modify the qubit references for operators that remap
/// qubits, take care to only send measurement data upstream when these
/// were explicitly requested through the associated upstream gate
/// function's `measured` list.
///
/// The default behavior for this callback is to return the measurement
/// without modification.
#[no_mangle]
pub extern "C" fn dqcs_pdef_set_modify_measurement_cb(
    pdef: dqcs_handle_t,
    callback: Option<
        extern "C" fn(
            user_data: *mut c_void,
            state: dqcs_plugin_state_t,
            meas: dqcs_handle_t,
        ) -> dqcs_handle_t,
    >,
    user_free: Option<extern "C" fn(user_data: *mut c_void)>,
    user_data: *mut c_void,
) -> dqcs_return_t {
    api_return_none(|| {
        let data = CallbackUserData::new(user_free, user_data);
        let callback = callback.ok_or_else(oe_inv_arg("callback cannot be null"))?;
        resolve!(pdef as &mut PluginDefinition);
        if pdef.get_type() != PluginType::Operator {
            return inv_op("the modify_measurement() callback is only supported for operators");
        }
        pdef.modify_measurement = Box::new(
            move |state: &mut PluginState,
                  meas: QubitMeasurementResult|
                  -> Result<Vec<QubitMeasurementResult>> {
                let meas_handle = insert(meas);
                let result = cb_return(0, callback(data.data(), state.into(), meas_handle))
                    .and_then(|result| {
                        take!(result as QubitMeasurementResultSet);
                        Ok(result.into_iter().map(|(_, m)| m).collect())
                    });
                delete!(meas_handle);
                result
            },
        );
        Ok(())
    })
}

/// Sets the callback for advancing time for operators and backends.
///
/// The default behavior for operators is to pass through to
/// `dqcs_plugin_advance()`. The default for backends is no-op. This
/// callback is never called for frontend plugins.
///
/// Besides the common arguments, the callback receives an unsigned integer
/// specifying the number of cycles to advance by.
///
/// The callback can return an error by setting an error message using
/// `dqcs_error_set()` and returning `DQCS_FAILURE`. Otherwise, it should
/// return `DQCS_SUCCESS`.
#[no_mangle]
pub extern "C" fn dqcs_pdef_set_advance_cb(
    pdef: dqcs_handle_t,
    callback: Option<
        extern "C" fn(
            user_data: *mut c_void,
            state: dqcs_plugin_state_t,
            cycles: dqcs_cycle_t,
        ) -> dqcs_return_t,
    >,
    user_free: Option<extern "C" fn(user_data: *mut c_void)>,
    user_data: *mut c_void,
) -> dqcs_return_t {
    api_return_none(|| {
        let data = CallbackUserData::new(user_free, user_data);
        let callback = callback.ok_or_else(oe_inv_arg("callback cannot be null"))?;
        resolve!(pdef as &mut PluginDefinition);
        if pdef.get_type() == PluginType::Frontend {
            return inv_op("the advance() callback is not supported for frontends");
        }
        pdef.advance = Box::new(move |state: &mut PluginState, cycles: u64| -> Result<()> {
            cb_return_none(callback(data.data(), state.into(), cycles as dqcs_cycle_t))
        });
        Ok(())
    })
}

/// Sets the callback function for handling an arb from upstream for
/// operators and backends.
///
/// The default behavior for operators is to pass through to
/// `dqcs_plugin_arb()`; operators that do not support the requested
/// interface should always do this. The default for backends is no-op.
/// This callback is never called for frontend plugins.
///
/// Besides the common arguments, the callback receives a handle to the
/// `ArbCmd` object representing the request. It must return a valid
/// `ArbData` handle containing the response. Both objects are deleted
/// automatically after invocation.
///
/// The callback can return an error by setting an error message using
/// `dqcs_error_set()` and returning 0. Otherwise, it should return a valid
/// `ArbData` handle.
#[no_mangle]
pub extern "C" fn dqcs_pdef_set_upstream_arb_cb(
    pdef: dqcs_handle_t,
    callback: Option<
        extern "C" fn(
            user_data: *mut c_void,
            state: dqcs_plugin_state_t,
            cmd: dqcs_handle_t,
        ) -> dqcs_handle_t,
    >,
    user_free: Option<extern "C" fn(user_data: *mut c_void)>,
    user_data: *mut c_void,
) -> dqcs_return_t {
    api_return_none(|| {
        let data = CallbackUserData::new(user_free, user_data);
        let callback = callback.ok_or_else(oe_inv_arg("callback cannot be null"))?;
        resolve!(pdef as &mut PluginDefinition);
        if pdef.get_type() == PluginType::Frontend {
            return inv_op("the upstream_arb() callback is not supported for frontends");
        }
        pdef.upstream_arb = Box::new(
            move |state: &mut PluginState, cmd: ArbCmd| -> Result<ArbData> {
                let cmd_handle = insert(cmd);
                let result = cb_return(0, callback(data.data(), state.into(), cmd_handle))
                    .and_then(|result| {
                        take!(result as ArbData);
                        Ok(result)
                    });
                delete!(cmd_handle);
                result
            },
        );
        Ok(())
    })
}

/// Sets the callback function function for handling an arb from the host.
///
/// The default behavior for this is no-op.
///
/// Besides the common arguments, the callback receives a handle to the
/// `ArbCmd` object representing the request. It must return a valid
/// `ArbData` handle containing the response. Both objects are deleted
/// automatically after invocation.
///
/// The callback can return an error by setting an error message using
/// `dqcs_error_set()` and returning 0. Otherwise, it should return a valid
/// `ArbData` handle.
#[no_mangle]
pub extern "C" fn dqcs_pdef_set_host_arb_cb(
    pdef: dqcs_handle_t,
    callback: Option<
        extern "C" fn(
            user_data: *mut c_void,
            state: dqcs_plugin_state_t,
            cmd: dqcs_handle_t,
        ) -> dqcs_handle_t,
    >,
    user_free: Option<extern "C" fn(user_data: *mut c_void)>,
    user_data: *mut c_void,
) -> dqcs_return_t {
    api_return_none(|| {
        let data = CallbackUserData::new(user_free, user_data);
        let callback = callback.ok_or_else(oe_inv_arg("callback cannot be null"))?;
        resolve!(pdef as &mut PluginDefinition);
        pdef.host_arb = Box::new(
            move |state: &mut PluginState, cmd: ArbCmd| -> Result<ArbData> {
                let cmd_handle = insert(cmd);
                let result = cb_return(0, callback(data.data(), state.into(), cmd_handle))
                    .and_then(|result| {
                        take!(result as ArbData);
                        Ok(result)
                    });
                delete!(cmd_handle);
                result
            },
        );
        Ok(())
    })
}
