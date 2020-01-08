use super::*;

/// Executes a plugin in the current thread.
///
/// `pdef` must be an appropriately populated plugin definition object.
/// Its callback functions will be called from the current thread, from within
/// the context of this function.
///
/// `simulator` must be set to the address of our endpoint of the simulator
/// that's using the plugin; DQCsim normally passes this as the first command
/// line argument of the plugin process.
///
/// If the plugin starts, the `pdef` handle is consumed by this function,
/// regardless of whether the plugin eventually closes normally. The handle is
/// only left alive if `pdef` is not a plugin definition object.
#[no_mangle]
pub extern "C" fn dqcs_plugin_run(pdef: dqcs_handle_t, simulator: *const c_char) -> dqcs_return_t {
    api_return_none(|| {
        take!(pdef as PluginDefinition);
        PluginState::run(&pdef, receive_str(simulator)?)
    })
}

/// Executes a plugin in a worker thread.
///
/// This function behaves the same as dqcs_plugin_log(), but is asynchronous;
/// it always returns immediately. Of course, this means that the callbacks in
/// `pdef` will be called from a different thread.
///
/// To wait for the thread to finish executing, call `dqcs_plugin_wait()` on
/// the returned join handle. Alternatively you can delete the join handle
/// object, which will detach the thread.
///
/// Note that `dqcs_log_*()` will only be available in the thread that the
/// plugin actually runs in.
///
/// This function returns 0 to indicate failure to start the plugin. Otherwise,
/// the join handle is returned.
#[no_mangle]
pub extern "C" fn dqcs_plugin_start(
    pdef: dqcs_handle_t,
    simulator: *const c_char,
) -> dqcs_handle_t {
    api_return(0, || {
        take!(pdef as PluginDefinition);
        let simulator = receive_str(simulator)?.to_string();
        Ok(insert(std::thread::spawn(move || {
            // Make sure panics are printed.
            std::panic::set_hook(Box::new(|info| {
                eprintln!("{}", info);
            }));

            PluginState::run(&pdef, simulator)
        })))
    })
}

/// Waits for a plugin worker thread to finish executing.
///
/// Unless the join handle is invalid, this function returns success/failure
/// based on the result of the plugin execution. If the plugin thread is
/// joined, the join handle is deleted.
#[no_mangle]
pub extern "C" fn dqcs_plugin_wait(pjoin: dqcs_handle_t) -> dqcs_return_t {
    api_return_none(|| {
        take!(pjoin as PluginJoinHandle);
        pjoin.join().map_err(|_| oe_err("thread panicked")())?
    })
}

/// Sends a message to the host.
///
/// It is only legal to call this function from within the `run()` callback.
/// Any other source will result in an error.
///
/// The `cmd` handle is consumed by this function if and only if it succeeds.
#[no_mangle]
pub extern "C" fn dqcs_plugin_send(
    plugin: dqcs_plugin_state_t,
    arb: dqcs_handle_t,
) -> dqcs_return_t {
    api_return_none(|| {
        resolve!(arb as pending ArbData);
        clone!(arb_ob: ArbData = resolved arb);
        plugin.resolve()?.send(arb_ob)?;
        delete!(resolved arb);
        Ok(())
    })
}

/// Waits for a message from the host.
///
/// It is only legal to call this function from within the `run()` callback.
/// Any other source will result in an error.
///
/// When successful, this function returns a new handle to the received
/// `ArbData` object. 0 is used to indicate that an error occurred.
#[no_mangle]
pub extern "C" fn dqcs_plugin_recv(plugin: dqcs_plugin_state_t) -> dqcs_handle_t {
    api_return(0, || Ok(insert(plugin.resolve()?.recv()?)))
}

/// Allocate the given number of downstream qubits.
///
/// Backend plugins are not allowed to call this. Doing so will result in an
/// error.
///
/// `num_qubits` specifies the number of qubits that are to be allocated.
///
/// `commands` must be 0 or a valid handle to an `ArbCmd` queue, containing a
/// list of commands that may be used to modify the behavior of the qubit
/// register; 0 is equivalent to zero commands. The queue is consumed by this
/// function, i.e. the handle becomes invalid, if and only if it succeeds.
///
/// If the function is successful, a new handle to the set of qubit references
/// representing the newly allocated register is returned. When the function
/// fails, 0 is returned.
#[no_mangle]
pub extern "C" fn dqcs_plugin_allocate(
    plugin: dqcs_plugin_state_t,
    num_qubits: usize,
    cq: dqcs_handle_t,
) -> dqcs_handle_t {
    api_return(0, || {
        let result = QubitReferenceSet::from(if cq != 0 {
            resolve!(cq as pending ArbCmdQueue);
            clone!(cq_ob: ArbCmdQueue = resolved cq);
            let result = plugin
                .resolve()?
                .allocate(num_qubits, cq_ob.into_iter().collect())?;
            delete!(resolved cq);
            result
        } else {
            plugin.resolve()?.allocate(num_qubits, vec![])?
        });
        Ok(insert(result))
    })
}

/// Free the given downstream qubits.
///
/// Backend plugins are not allowed to call this. Doing so will result in an
/// error.
///
/// `qubits` must be a valid set of qubit references. The set is consumed by
/// this function, i.e. the handle becomes invalid, if and only if it succeeds.
#[no_mangle]
pub extern "C" fn dqcs_plugin_free(
    plugin: dqcs_plugin_state_t,
    qbset: dqcs_handle_t,
) -> dqcs_return_t {
    api_return_none(|| {
        resolve!(qbset as pending QubitReferenceSet);
        clone!(qbset_ob: QubitReferenceSet = resolved qbset);
        plugin.resolve()?.free(qbset_ob.into())?;
        delete!(resolved qbset);
        Ok(())
    })
}

/// Tells the downstream plugin to execute a gate.
///
/// Backend plugins are not allowed to call this. Doing so will result in an
/// error.
///
/// `gate` must be a valid gate object. The object is consumed by this
/// function, i.e. the handle becomes invalid, if and only if it succeeds.
#[no_mangle]
pub extern "C" fn dqcs_plugin_gate(
    plugin: dqcs_plugin_state_t,
    gate: dqcs_handle_t,
) -> dqcs_return_t {
    api_return_none(|| {
        resolve!(gate as pending Gate);
        clone!(gate_ob: Gate = resolved gate);
        plugin.resolve()?.gate(gate_ob)?;
        delete!(resolved gate);
        Ok(())
    })
}

/// Returns the latest measurement of the given downstream qubit.
///
/// Backend plugins are not allowed to call this. Doing so will result in an
/// error.
///
/// If the function succeeds, it returns a new handle to a qubit measurement
/// result object. Otherwise it returns 0.
#[no_mangle]
pub extern "C" fn dqcs_plugin_get_measurement(
    plugin: dqcs_plugin_state_t,
    qubit: dqcs_qubit_t,
) -> dqcs_handle_t {
    api_return(0, || {
        let qubit =
            QubitRef::from_foreign(qubit).ok_or_else(oe_inv_arg("0 is not a valid qubit"))?;
        Ok(insert(plugin.resolve()?.get_measurement(qubit)?))
    })
}

/// Returns the number of downstream cycles since the latest measurement of the
/// given downstream qubit.
///
/// Backend plugins are not allowed to call this. Doing so will result in an
/// error.
///
/// This function uses -1 to signal an error.
#[no_mangle]
pub extern "C" fn dqcs_plugin_get_cycles_since_measure(
    plugin: dqcs_plugin_state_t,
    qubit: dqcs_qubit_t,
) -> dqcs_cycle_t {
    api_return(-1, || {
        let qubit =
            QubitRef::from_foreign(qubit).ok_or_else(oe_inv_arg("0 is not a valid qubit"))?;
        Ok(plugin.resolve()?.get_cycles_since_measure(qubit)? as dqcs_cycle_t)
    })
}

/// Returns the number of downstream cycles between the last two measurements
/// of the given downstream qubit.
///
/// Backend plugins are not allowed to call this. Doing so will result in an
/// error.
///
/// This function uses -1 to signal an error.
#[no_mangle]
pub extern "C" fn dqcs_plugin_get_cycles_between_measures(
    plugin: dqcs_plugin_state_t,
    qubit: dqcs_qubit_t,
) -> dqcs_cycle_t {
    api_return(-1, || {
        let qubit =
            QubitRef::from_foreign(qubit).ok_or_else(oe_inv_arg("0 is not a valid qubit"))?;
        Ok(plugin.resolve()?.get_cycles_between_measures(qubit)? as dqcs_cycle_t)
    })
}

/// Tells the downstream plugin to run for the specified number of cycles.
///
/// Backend plugins are not allowed to call this. Doing so will result in an
/// error.
///
/// The return value is the new cycle counter. This function uses -1 to signal
/// an error.
#[no_mangle]
pub extern "C" fn dqcs_plugin_advance(
    plugin: dqcs_plugin_state_t,
    cycles: dqcs_cycle_t,
) -> dqcs_cycle_t {
    api_return(-1, || {
        if cycles < 0 {
            inv_arg("cannot advance by a negative number of cycles")
        } else {
            Ok(plugin.resolve()?.advance(cycles as u64)?.into())
        }
    })
}

/// Returns the current value of the downstream cycle counter.
///
/// Backend plugins are not allowed to call this. Doing so will result in an
/// error.
///
/// This function uses -1 to signal an error.
#[no_mangle]
pub extern "C" fn dqcs_plugin_get_cycle(plugin: dqcs_plugin_state_t) -> dqcs_cycle_t {
    api_return(-1, || Ok(plugin.resolve()?.get_cycle()?.into()))
}

/// Sends an arbitrary command downstream.
///
/// Backend plugins are not allowed to call this. Doing so will result in an
/// error.
///
/// This function returns a new handle to an `ArbData` object representing the
/// return value of the `ArbCmd` when successful. Otherwise, it returns 0.
#[no_mangle]
pub extern "C" fn dqcs_plugin_arb(
    plugin: dqcs_plugin_state_t,
    cmd: dqcs_handle_t,
) -> dqcs_handle_t {
    api_return(0, || {
        resolve!(cmd as pending ArbCmd);
        clone!(cmd_ob: ArbCmd = resolved cmd);
        let result = plugin.resolve()?.arb(cmd_ob)?;
        delete!(resolved cmd);
        Ok(insert(result))
    })
}

/// Generates a random unsigned 64-bit number using the simulator random seed.
///
/// This function only fails if the `plugin` handle is invalid, in which case
/// it returns 0. Of course, 0 is also a valid (if rare) random return value.
#[no_mangle]
pub extern "C" fn dqcs_plugin_random_u64(plugin: dqcs_plugin_state_t) -> dqcs_handle_t {
    api_return(0, || Ok(plugin.resolve()?.random_u64()))
}

/// Generates a random floating point number using the simulator random seed.
///
/// The generated numbers are uniformly distributed in the range `[0,1>`.
///
/// This function only fails if the `plugin` handle is invalid, in which case
/// it returns 0. Of course, 0 is also a valid (if rare) random return value.
#[no_mangle]
pub extern "C" fn dqcs_plugin_random_f64(plugin: dqcs_plugin_state_t) -> c_double {
    api_return(0.0, || Ok(plugin.resolve()?.random_f64()))
}
