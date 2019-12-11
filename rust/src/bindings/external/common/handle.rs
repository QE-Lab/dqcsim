use super::*;

/// Returns the type of object associated with the given handle.
#[no_mangle]
pub extern "C" fn dqcs_handle_type(handle: dqcs_handle_t) -> dqcs_handle_type_t {
    api_return(dqcs_handle_type_t::DQCS_HTYPE_INVALID, || {
        API_STATE.with(|state| {
            let state = state.borrow();
            match &state.objects.get(&handle) {
                None => inv_arg(format!("handle {} is invalid", handle)),
                Some(APIObject::ArbData(_)) => Ok(dqcs_handle_type_t::DQCS_HTYPE_ARB_DATA),
                Some(APIObject::ArbCmd(_)) => Ok(dqcs_handle_type_t::DQCS_HTYPE_ARB_CMD),
                Some(APIObject::ArbCmdQueue(_)) => Ok(dqcs_handle_type_t::DQCS_HTYPE_ARB_CMD_QUEUE),
                Some(APIObject::QubitReferenceSet(_)) => {
                    Ok(dqcs_handle_type_t::DQCS_HTYPE_QUBIT_SET)
                }
                Some(APIObject::Gate(_)) => Ok(dqcs_handle_type_t::DQCS_HTYPE_GATE),
                Some(APIObject::QubitMeasurementResult(_)) => {
                    Ok(dqcs_handle_type_t::DQCS_HTYPE_MEAS)
                }
                Some(APIObject::QubitMeasurementResultSet(_)) => {
                    Ok(dqcs_handle_type_t::DQCS_HTYPE_MEAS_SET)
                }
                Some(APIObject::MatrixMapC(_)) => Ok(dqcs_handle_type_t::DQCS_HTYPE_MATRIX_MAP),
                Some(APIObject::MatrixMapBuilderC(_)) => {
                    Ok(dqcs_handle_type_t::DQCS_HTYPE_MATRIX_MAP_BUILDER)
                }
                Some(APIObject::PluginProcessConfiguration(x)) => match x.get_type() {
                    PluginType::Frontend => Ok(dqcs_handle_type_t::DQCS_HTYPE_FRONT_PROCESS_CONFIG),
                    PluginType::Operator => Ok(dqcs_handle_type_t::DQCS_HTYPE_OPER_PROCESS_CONFIG),
                    PluginType::Backend => Ok(dqcs_handle_type_t::DQCS_HTYPE_BACK_PROCESS_CONFIG),
                },
                Some(APIObject::PluginThreadConfiguration(x)) => match x.get_type() {
                    PluginType::Frontend => Ok(dqcs_handle_type_t::DQCS_HTYPE_FRONT_THREAD_CONFIG),
                    PluginType::Operator => Ok(dqcs_handle_type_t::DQCS_HTYPE_OPER_THREAD_CONFIG),
                    PluginType::Backend => Ok(dqcs_handle_type_t::DQCS_HTYPE_BACK_THREAD_CONFIG),
                },
                Some(APIObject::SimulatorConfiguration(_)) => {
                    Ok(dqcs_handle_type_t::DQCS_HTYPE_SIM_CONFIG)
                }
                Some(APIObject::Simulator(_)) => Ok(dqcs_handle_type_t::DQCS_HTYPE_SIM),
                Some(APIObject::PluginDefinition(x)) => match x.get_type() {
                    PluginType::Frontend => Ok(dqcs_handle_type_t::DQCS_HTYPE_FRONT_DEF),
                    PluginType::Operator => Ok(dqcs_handle_type_t::DQCS_HTYPE_OPER_DEF),
                    PluginType::Backend => Ok(dqcs_handle_type_t::DQCS_HTYPE_BACK_DEF),
                },
                Some(APIObject::PluginJoinHandle(_)) => {
                    Ok(dqcs_handle_type_t::DQCS_HTYPE_PLUGIN_JOIN)
                }
            }
        })
    })
}

/// Returns a debug dump of the object associated with the given handle.
///
/// On success, this **returns a newly allocated string containing the
/// description. Free it with `free()` when you're done with it to avoid memory
/// leaks.** On failure (i.e., the handle is invalid) this returns `NULL`.
#[no_mangle]
pub extern "C" fn dqcs_handle_dump(handle: dqcs_handle_t) -> *mut c_char {
    api_return_string(|| {
        resolve!(handle as &APIObject);
        Ok(format!("{:#?}", handle))
    })
}

/// Destroys the object associated with a handle.
///
/// Returns 0 when successful, -1 otherwise.
#[no_mangle]
pub extern "C" fn dqcs_handle_delete(handle: dqcs_handle_t) -> dqcs_return_t {
    api_return_none(|| {
        // Use take! vs. delete! so an error is returned if the handle was
        // invalid or already freed.
        take!(handle as APIObject);
        let _ = handle;
        Ok(())
    })
}

/// Deletes all handles for the current thread.
///
/// This can be used to clean stuff up at the end of `main()` or before an
/// `abort()` of some kind. If you don't clean up properly, you might get
/// undefined behavior or errors when DQCsim tries to do it for you.
#[no_mangle]
pub extern "C" fn dqcs_handle_delete_all() -> dqcs_return_t {
    API_STATE.with(|state| state.borrow_mut().objects.clear());
    dqcs_return_t::DQCS_SUCCESS
}

/// Succeeds only if there are no live handles in the current thread.
///
/// This is intended for testing and for finding handle leaks. The error
/// message returned when handles remain contains dumps of the first 10
/// remaining handles.
#[no_mangle]
pub extern "C" fn dqcs_handle_leak_check() -> dqcs_return_t {
    api_return_none(|| {
        API_STATE.with(|state| {
            let remain = state.borrow().objects.len();
            if remain == 0 {
                Ok(())
            } else {
                let mut msg = format!("Leak check: {} handles remain", remain);
                let state = state.borrow();
                let sorted: std::collections::BTreeMap<_, _> = state.objects.iter().collect();
                for (idx, (handle, object)) in sorted.iter().enumerate() {
                    if idx == 10 {
                        msg = format!("{}, and {} more", msg, remain - 10);
                        break;
                    }
                    msg = format!("{}, {} = {:?}", msg, handle, object);
                }
                err(msg)
            }
        })
    })
}
