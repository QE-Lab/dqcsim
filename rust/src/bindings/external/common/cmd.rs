use super::*;

/// Creates a new `ArbCmd` object.
///>
///> Returns the handle of the newly created `ArbCmd`. The `ArbCmd` is
///> initialized with the given interface and operation IDs, JSON object `{}`,
///> and an empty binary argument list. Upon failure, returns 0.
///>
///> `ArbCmd` objects support the `handle`, `arb`, and `cmd` interfaces.
#[no_mangle]
pub extern "C" fn dqcs_cmd_new(iface: *const c_char, oper: *const c_char) -> dqcs_handle_t {
    api_return(0, || {
        Ok(insert(APIObject::ArbCmd(ArbCmd::try_from(
            receive_str(iface)?,
            receive_str(oper)?,
            ArbData::default(),
        )?)))
    })
}

/// Returns the interface ID of an `ArbCmd`.
///>
///> On success, this **returns a newly allocated string containing the JSON
///> string. Free it with `free()` when you're done with it to avoid memory
///> leaks.** On failure, this returns `NULL`.
#[no_mangle]
pub extern "C" fn dqcs_cmd_iface_get(cmd: dqcs_handle_t) -> *mut c_char {
    api_return_string(|| {
        resolve!(cmd as &ArbCmd);
        Ok(cmd.interface_identifier().to_string())
    })
}

/// Compares the interface ID of an `ArbCmd` with the given string.
///>
///> Returns -1 for failure, 0 for no match, or 1 for a match.
#[no_mangle]
pub extern "C" fn dqcs_cmd_iface_cmp(
    cmd: dqcs_handle_t,
    iface: *const c_char,
) -> dqcs_bool_return_t {
    api_return_bool(|| {
        resolve!(cmd as &ArbCmd);
        Ok(cmd.interface_identifier() == receive_str(iface)?)
    })
}

/// Returns the operation ID of an `ArbCmd`.
///>
///> On success, this **returns a newly allocated string containing the JSON
///> string. Free it with `free()` when you're done with it to avoid memory
///> leaks.** On failure, this returns `NULL`.
#[no_mangle]
pub extern "C" fn dqcs_cmd_oper_get(cmd: dqcs_handle_t) -> *mut c_char {
    api_return_string(|| {
        resolve!(cmd as &ArbCmd);
        Ok(cmd.operation_identifier().to_string())
    })
}

/// Compares the operation ID of an `ArbCmd` with the given string.
///>
///> Returns -1 for failure, 0 for no match, or 1 for a match.
#[no_mangle]
pub extern "C" fn dqcs_cmd_oper_cmp(cmd: dqcs_handle_t, oper: *const c_char) -> dqcs_bool_return_t {
    api_return_bool(|| {
        resolve!(cmd as &ArbCmd);
        Ok(cmd.operation_identifier() == receive_str(oper)?)
    })
}
