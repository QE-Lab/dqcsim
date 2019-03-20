use super::*;
use std::ptr::null;

/// Creates a new `ArbCmd` object.
///
/// Returns the handle of the newly created `ArbCmd`. The `ArbCmd` is
/// initialized with the given interface and operation IDs, JSON object `{}`,
/// and an empty binary argument list. Upon failure, returns 0.
///
/// `ArbCmd` objects support the `handle`, `arb`, and `cmd` interfaces.
#[no_mangle]
pub extern "C" fn dqcs_cmd_new(iface: *const c_char, oper: *const c_char) -> dqcs_handle_t {
    with_state(
        || 0,
        |mut state| {
            Ok(state.push(Object::ArbCmd(ArbCmd::try_from(
                receive_str(iface)?,
                receive_str(oper)?,
                ArbData::default(),
            )?)))
        },
    )
}

/// Returns the interface ID of an `ArbCmd`.
///
/// On success, this **returns a newly allocated string containing the JSON
/// string. Free it with `free()` when you're done with it to avoid memory
/// leaks.** On failure, this returns `NULL`.
#[no_mangle]
pub extern "C" fn dqcs_cmd_iface_get(handle: dqcs_handle_t) -> *const c_char {
    with_cmd(handle, null, |cmd| {
        return_string(cmd.interface_identifier())
    })
}

/// Compares the interface ID of an `ArbCmd` with the given string.
///
/// Returns -1 for failure, 0 for no match, or 1 for a match.
#[no_mangle]
pub extern "C" fn dqcs_cmd_iface_cmp(
    handle: dqcs_handle_t,
    iface: *const c_char,
) -> dqcs_bool_return_t {
    with_cmd(
        handle,
        || dqcs_bool_return_t::DQCS_BOOL_FAILURE,
        |cmd| {
            Ok(dqcs_bool_return_t::from(
                cmd.interface_identifier() == receive_str(iface)?,
            ))
        },
    )
}

/// Returns the operation ID of an `ArbCmd`.
///
/// On success, this **returns a newly allocated string containing the JSON
/// string. Free it with `free()` when you're done with it to avoid memory
/// leaks.** On failure, this returns `NULL`.
#[no_mangle]
pub extern "C" fn dqcs_cmd_oper_get(handle: dqcs_handle_t) -> *const c_char {
    with_cmd(handle, null, |cmd| {
        return_string(cmd.operation_identifier())
    })
}

/// Compares the operation ID of an `ArbCmd` with the given string.
///
/// Returns -1 for failure, 0 for no match, or 1 for a match.
#[no_mangle]
pub extern "C" fn dqcs_cmd_oper_cmp(
    handle: dqcs_handle_t,
    oper: *const c_char,
) -> dqcs_bool_return_t {
    with_cmd(
        handle,
        || dqcs_bool_return_t::DQCS_BOOL_FAILURE,
        |cmd| {
            Ok(dqcs_bool_return_t::from(
                cmd.operation_identifier() == receive_str(oper)?,
            ))
        },
    )
}
