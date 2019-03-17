use super::*;
use failure::Error;
use std::ptr::{null, null_mut};

/// Convenience function for writing functions that operate on `ArbData`s.
fn with_arb<T>(
    handle: dqcs_handle_t,
    error: impl FnOnce() -> T,
    call: impl FnOnce(&mut ArbData) -> Result<T, Error>,
) -> T {
    with_state(error, |mut state| match state.objects.get_mut(&handle) {
        Some(Object::ArbData(x)) => call(x),
        Some(Object::ArbCmd(x)) => call(x.data_mut()),
        Some(_) => Err(APIError::UnsupportedHandle(handle).into()),
        None => Err(APIError::InvalidHandle(handle).into()),
    })
}

/// Creates a new `ArbData` object.
///
/// Returns the handle of the newly created `ArbData`. The `ArbData` is
/// initialized with JSON object `{}` and an empty binary argument list.
///
/// `ArbData` objects support the `handle` and `arb`.
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

/// Pushes an unstructured raw argument to the back of the list.
#[no_mangle]
pub extern "C" fn dqcs_arb_push_raw(
    handle: dqcs_handle_t,
    obj: *const c_void,
    obj_size: size_t,
) -> dqcs_return_t {
    with_arb(
        handle,
        || dqcs_return_t::DQCS_FAILURE,
        |arb| {
            arb.args.push(receive_raw(obj, obj_size)?.to_owned());
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

/// Pops an unstructured raw argument from the back of the list.
///
/// If the actual size of the object differs from the specified object size,
/// this function will copy the minimum of the actual and specified sizes
/// number of bytes, and return what the actual size was.
///
/// If the specified object size is zero, `obj` is allowed to be `NULL`. You
/// can use this if you don't need the contents of the argument and just want
/// to delete it.
///
/// Since this function removes the returned element, data will be lost if the
/// specified size is smaller than the actual size. To avoid this, first use
/// `dqcs_arb_get_size(handle, -1)` to query the size.
///
/// This function returns -1 on failure.
#[no_mangle]
pub extern "C" fn dqcs_arb_pop_raw(
    handle: dqcs_handle_t,
    obj: *mut c_void,
    obj_size: size_t,
) -> ssize_t {
    with_arb(
        handle,
        || -1,
        |arb| {
            return_raw(
                &arb.args.pop().ok_or_else(|| APIError::IndexError(0))?,
                obj,
                obj_size,
            )
        },
    )
}

/// Pops an unstructured argument from the back of the list without returning
/// it.
#[no_mangle]
pub extern "C" fn dqcs_arb_pop(handle: dqcs_handle_t) -> dqcs_return_t {
    if dqcs_arb_pop_raw(handle, null_mut(), 0) < 0 {
        dqcs_return_t::DQCS_FAILURE
    } else {
        dqcs_return_t::DQCS_SUCCESS
    }
}

/// Inserts an unstructured string argument into the list at the specified
/// index.
#[no_mangle]
pub extern "C" fn dqcs_arb_insert_str(
    handle: dqcs_handle_t,
    index: ssize_t,
    s: *const c_char,
) -> dqcs_return_t {
    with_arb(
        handle,
        || dqcs_return_t::DQCS_FAILURE,
        |arb| {
            arb.args.insert(
                receive_index(arb.args.len(), index, true)?,
                receive_str(s)?.bytes().collect(),
            );
            Ok(dqcs_return_t::DQCS_SUCCESS)
        },
    )
}

/// Inserts an unstructured raw argument into the list at the specified
/// index.
#[no_mangle]
pub extern "C" fn dqcs_arb_insert_raw(
    handle: dqcs_handle_t,
    index: ssize_t,
    obj: *const c_void,
    obj_size: size_t,
) -> dqcs_return_t {
    with_arb(
        handle,
        || dqcs_return_t::DQCS_FAILURE,
        |arb| {
            arb.args.insert(
                receive_index(arb.args.len(), index, true)?,
                receive_raw(obj, obj_size)?.to_owned(),
            );
            Ok(dqcs_return_t::DQCS_SUCCESS)
        },
    )
}

/// Removes the specified unstructured string argument from the list.
#[no_mangle]
pub extern "C" fn dqcs_arb_remove(handle: dqcs_handle_t, index: ssize_t) -> dqcs_return_t {
    with_arb(
        handle,
        || dqcs_return_t::DQCS_FAILURE,
        |arb| {
            arb.args
                .remove(receive_index(arb.args.len(), index, false)?);
            Ok(dqcs_return_t::DQCS_SUCCESS)
        },
    )
}

/// Replaces the unstructured argument at the specified index with the
/// specified string.
#[no_mangle]
pub extern "C" fn dqcs_arb_set_str(
    handle: dqcs_handle_t,
    index: ssize_t,
    s: *const c_char,
) -> dqcs_return_t {
    with_arb(
        handle,
        || dqcs_return_t::DQCS_FAILURE,
        |arb| {
            let mut s: Vec<u8> = receive_str(s)?.bytes().collect();
            let index = receive_index(arb.args.len(), index, false)?;
            let arg = &mut arb.args[index];
            arg.clear();
            arg.append(&mut s);
            Ok(dqcs_return_t::DQCS_SUCCESS)
        },
    )
}

/// Replaces the unstructured argument at the specified index with the
/// specified raw object.
#[no_mangle]
pub extern "C" fn dqcs_arb_set_raw(
    handle: dqcs_handle_t,
    index: ssize_t,
    obj: *const c_void,
    obj_size: size_t,
) -> dqcs_return_t {
    with_arb(
        handle,
        || dqcs_return_t::DQCS_FAILURE,
        |arb| {
            let mut s: Vec<u8> = receive_raw(obj, obj_size)?.to_owned();
            let index = receive_index(arb.args.len(), index, false)?;
            let arg = &mut arb.args[index];
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
pub extern "C" fn dqcs_arb_get_str(handle: dqcs_handle_t, index: ssize_t) -> *const c_char {
    with_arb(handle, null, |arb| {
        return_string(String::from_utf8(
            arb.args[receive_index(arb.args.len(), index, false)?].clone(),
        )?)
    })
}

/// Returns the unstructured string argument at the specified index.
///
/// If the actual size of the object differs from the specified object size,
/// this function will copy the minimum of the actual and specified sizes
/// number of bytes, and return what the actual size was.
///
/// If the specified object size is zero, `obj` is allowed to be `NULL`. You
/// can use this to determine the size of the argument prior to actually
/// reading it, so you can allocate the right buffer size first.
///
/// This function returns -1 on failure.
#[no_mangle]
pub extern "C" fn dqcs_arb_get_raw(
    handle: dqcs_handle_t,
    index: ssize_t,
    obj: *mut c_void,
    obj_size: size_t,
) -> ssize_t {
    with_arb(
        handle,
        || -1,
        |arb| {
            return_raw(
                &arb.args[receive_index(arb.args.len(), index, false)?],
                obj,
                obj_size,
            )
        },
    )
}

/// Returns the size in bytes of the unstructured string argument at the
/// specified index.
///
/// Returns -1 when the function fails.
#[no_mangle]
pub extern "C" fn dqcs_arb_get_size(handle: dqcs_handle_t, index: ssize_t) -> ssize_t {
    dqcs_arb_get_raw(handle, index, null_mut(), 0)
}

/// Returns the number of unstructured arguments, or -1 to indicate failure.
#[no_mangle]
pub extern "C" fn dqcs_arb_len(handle: dqcs_handle_t) -> ssize_t {
    with_arb(handle, || -1, |arb| Ok(arb.args.len() as ssize_t))
}

/// Clears the unstructured argument list.
#[no_mangle]
pub extern "C" fn dqcs_arb_clear(handle: dqcs_handle_t) -> dqcs_return_t {
    with_arb(
        handle,
        || dqcs_return_t::DQCS_FAILURE,
        |arb| {
            arb.args.clear();
            Ok(dqcs_return_t::DQCS_SUCCESS)
        },
    )
}

/// Copies the data from one object to another.
#[no_mangle]
pub extern "C" fn dqcs_arb_assign(dest: dqcs_handle_t, src: dqcs_handle_t) -> dqcs_return_t {
    let src_clone = with_arb(src, || None, |src_arb| Ok(Some(src_arb.clone())));
    match src_clone {
        Some(src_clone) => with_arb(
            dest,
            || dqcs_return_t::DQCS_FAILURE,
            |dest_arb| {
                dest_arb.json = src_clone.json;
                dest_arb.args = src_clone.args;
                Ok(dqcs_return_t::DQCS_SUCCESS)
            },
        ),
        None => dqcs_return_t::DQCS_FAILURE,
    }
}
