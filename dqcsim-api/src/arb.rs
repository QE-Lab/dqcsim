use super::*;
use failure::Error;
use std::ptr::null;

/// Convenience function for writing functions that operate on `ArbData`s.
fn with_arb<T>(
    handle: dqcs_handle_t,
    error: impl FnOnce() -> T,
    call: impl FnOnce(&mut ArbData) -> Result<T, Error>,
) -> T {
    STATE.with(|state| {
        let mut state = state.borrow_mut();
        match state.objects.get_mut(&handle) {
            Some(Object::ArbData(x)) => match call(x) {
                Ok(r) => r,
                Err(e) => {
                    state.fail(e.to_string());
                    error()
                }
            },
            Some(_) => {
                state.fail(format!("Handle {} is not of type ArbData", handle));
                error()
            }
            None => {
                state.fail(format!("Handle {} is invalid", handle));
                error()
            }
        }
    })
}

/// Creates a new `ArbData` object.
///
/// Returns the handle of the newly created ArbData. The `ArbData` is
/// initialized with JSON object `{}` and an empty binary argument list.
#[no_mangle]
pub extern "C" fn dqcs_arb_new() -> dqcs_handle_t {
    STATE.with(|state| {
        let mut state = state.borrow_mut();
        state.push(Object::ArbData(ArbData::default()))
    })
}

/// Sets the JSON object of an `ArbData` object by means of a JSON string.
#[no_mangle]
pub extern "C" fn dqcs_arb_json_set_str(
    handle: dqcs_handle_t,
    json: *const c_char,
) -> dqcs_return_t {
    with_arb(
        handle,
        || dqcs_return_t::DQCS_FAILURE,
        |arb| {
            arb.json = serde_json::from_str(receive_str(json)?)?;
            Ok(dqcs_return_t::DQCS_SUCCESS)
        },
    )
}

/// Returns the JSON object of an `ArbData` object in the form of a JSON string.
///
/// On success, this **returns a newly allocated string containing the JSON
/// string. Free it with `free()` when you're done with it to avoid memory
/// leaks.** On failure, this returns `NULL`.
#[no_mangle]
pub extern "C" fn dqcs_arb_json_get_str(handle: dqcs_handle_t) -> *const c_char {
    with_arb(handle, null, |arb| {
        return_string(serde_json::to_string(&arb.json)?)
    })
}

/// Pushes an unstructured string argument to the back of the list.
#[no_mangle]
pub extern "C" fn dqcs_arb_push_str(handle: dqcs_handle_t, s: *const c_char) -> dqcs_return_t {
    with_arb(
        handle,
        || dqcs_return_t::DQCS_FAILURE,
        |arb| {
            arb.args.push(receive_str(s)?.bytes().collect());
            Ok(dqcs_return_t::DQCS_SUCCESS)
        },
    )
}

/// Pops an unstructured string argument from the back of the list.
///
/// On success, this **returns a newly allocated string containing the JSON
/// string. Free it with `free()` when you're done with it to avoid memory
/// leaks.** On failure, this returns `NULL`.
#[no_mangle]
pub extern "C" fn dqcs_arb_pop_str(handle: dqcs_handle_t) -> *const c_char {
    with_arb(handle, null, |arb| {
        return_string(String::from_utf8(
            arb.args.pop().ok_or_else(|| APIError::IndexError(0))?,
        )?)
    })
}

/// Inserts an unstructured string argument into the list at the specified
/// index.
#[no_mangle]
pub extern "C" fn dqcs_arb_insert_str(
    handle: dqcs_handle_t,
    index: size_t,
    s: *const c_char,
) -> dqcs_return_t {
    with_arb(
        handle,
        || dqcs_return_t::DQCS_FAILURE,
        |arb| {
            if index > arb.args.len() {
                Err(APIError::IndexError(index).into())
            } else {
                arb.args.insert(index, receive_str(s)?.bytes().collect());
                Ok(dqcs_return_t::DQCS_SUCCESS)
            }
        },
    )
}

/// Removes the specified unstructured string argument from the list.
#[no_mangle]
pub extern "C" fn dqcs_arb_remove(handle: dqcs_handle_t, index: size_t) -> dqcs_return_t {
    with_arb(
        handle,
        || dqcs_return_t::DQCS_FAILURE,
        |arb| {
            if index >= arb.args.len() {
                Err(APIError::IndexError(index).into())
            } else {
                arb.args.remove(index);
                Ok(dqcs_return_t::DQCS_SUCCESS)
            }
        },
    )
}

/// Replaces the unstructured argument at the specified index with the
/// specified string.
#[no_mangle]
pub extern "C" fn dqcs_arb_set_str(
    handle: dqcs_handle_t,
    index: size_t,
    s: *const c_char,
) -> dqcs_return_t {
    with_arb(
        handle,
        || dqcs_return_t::DQCS_FAILURE,
        |arb| {
            let mut s: Vec<u8> = receive_str(s)?.bytes().collect();
            let arg = arb
                .args
                .get_mut(index)
                .ok_or_else(|| APIError::IndexError(0))?;
            arg.clear();
            arg.append(&mut s);
            Ok(dqcs_return_t::DQCS_SUCCESS)
        },
    )
}

/// Returns the unstructured string argument at the specified index.
///
/// On success, this **returns a newly allocated string containing the JSON
/// string. Free it with `free()` when you're done with it to avoid memory
/// leaks.** On failure, this returns `NULL`.
#[no_mangle]
pub extern "C" fn dqcs_arb_get_str(handle: dqcs_handle_t, index: size_t) -> *const c_char {
    with_arb(handle, null, |arb| {
        return_string(String::from_utf8(
            arb.args
                .get(index)
                .ok_or_else(|| APIError::IndexError(0))?
                .clone(),
        )?)
    })
}

/// Returns the number of unstructured arguments, or -1 to indicate failure.
#[no_mangle]
pub extern "C" fn dqcs_arb_len(handle: dqcs_handle_t) -> ssize_t {
    with_arb(handle, || -1, |arb| Ok(arb.args.len() as ssize_t))
}
