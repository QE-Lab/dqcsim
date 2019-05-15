use super::*;
use std::ptr::null_mut;

/// Creates a new `ArbData` object.
///
/// Returns the handle of the newly created `ArbData`. The `ArbData` is
/// initialized with JSON object `{}` and an empty binary argument list.
///
/// `ArbData` objects support the `handle` and `arb` APIs.
#[no_mangle]
pub extern "C" fn dqcs_arb_new() -> dqcs_handle_t {
    insert(ArbData::default())
}

/// Sets the JSON/CBOR object of an `ArbData` object by means of a JSON string.
#[no_mangle]
pub extern "C" fn dqcs_arb_json_set(arb: dqcs_handle_t, json: *const c_char) -> dqcs_return_t {
    api_return_none(|| {
        resolve!(arb as &mut ArbData);
        arb.set_json(receive_str(json)?)?;
        Ok(())
    })
}

/// Returns the JSON/CBOR object of an `ArbData` object in the form of a JSON
/// string.
///
/// On success, this **returns a newly allocated string containing the JSON
/// string. Free it with `free()` when you're done with it to avoid memory
/// leaks.** On failure, this returns `NULL`.
#[no_mangle]
pub extern "C" fn dqcs_arb_json_get(arb: dqcs_handle_t) -> *mut c_char {
    api_return_string(|| {
        resolve!(arb as &ArbData);
        arb.get_json()
    })
}

/// Sets the JSON/CBOR object of an `ArbData` object by means of a CBOR object.
#[no_mangle]
pub extern "C" fn dqcs_arb_cbor_set(
    arb: dqcs_handle_t,
    obj: *const c_void,
    obj_size: size_t,
) -> dqcs_return_t {
    api_return_none(|| {
        resolve!(arb as &mut ArbData);
        arb.set_cbor(receive_raw(obj, obj_size)?)
    })
}

/// Returns the JSON/CBOR object of an `ArbData` object in the form of a CBOR
/// object.
///
/// If the actual size of the object differs from the specified object size,
/// this function will copy the minimum of the actual and specified sizes
/// number of bytes, and return what the actual size was.
///
/// If the specified object size is zero, `obj` is allowed to be `NULL`. You
/// can use this to query the size before allocating an object.
///
/// This function returns -1 on failure.
#[no_mangle]
pub extern "C" fn dqcs_arb_cbor_get(
    arb: dqcs_handle_t,
    obj: *mut c_void,
    obj_size: size_t,
) -> ssize_t {
    api_return(-1, || {
        resolve!(arb as &ArbData);
        return_raw(&arb.get_cbor(), obj, obj_size)
    })
}

/// Pushes an unstructured string argument to the back of the list.
#[no_mangle]
pub extern "C" fn dqcs_arb_push_str(arb: dqcs_handle_t, s: *const c_char) -> dqcs_return_t {
    api_return_none(|| {
        resolve!(arb as &mut ArbData);
        arb.get_args_mut().push(receive_str(s)?.bytes().collect());
        Ok(())
    })
}

/// Pushes an unstructured raw argument to the back of the list.
#[no_mangle]
pub extern "C" fn dqcs_arb_push_raw(
    arb: dqcs_handle_t,
    obj: *const c_void,
    obj_size: size_t,
) -> dqcs_return_t {
    api_return_none(|| {
        resolve!(arb as &mut ArbData);
        arb.get_args_mut()
            .push(receive_raw(obj, obj_size)?.to_owned());
        Ok(())
    })
}

/// Pops an unstructured string argument from the back of the list.
///
/// On success, this **returns a newly allocated string containing the JSON
/// string. Free it with `free()` when you're done with it to avoid memory
/// leaks.** On failure, this returns `NULL`. If the failure is due to the
/// conversion from binary object to C string (i.e., embedded nulls), the
/// data is still popped and is thus lost.
#[no_mangle]
pub extern "C" fn dqcs_arb_pop_str(arb: dqcs_handle_t) -> *mut c_char {
    api_return_string(|| {
        resolve!(arb as &mut ArbData);
        Ok(String::from_utf8(
            arb.get_args_mut()
                .pop()
                .ok_or_else(oe_inv_arg("pop from empty list"))?,
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
/// This function returns -1 on failure. If this is due to a `NULL` buffer
/// being passed, the data that was popped is lost.
#[no_mangle]
pub extern "C" fn dqcs_arb_pop_raw(
    arb: dqcs_handle_t,
    obj: *mut c_void,
    obj_size: size_t,
) -> ssize_t {
    api_return(-1, || {
        resolve!(arb as &mut ArbData);
        return_raw(
            &arb.get_args_mut()
                .pop()
                .ok_or_else(oe_inv_arg("pop from empty list"))?,
            obj,
            obj_size,
        )
    })
}

/// Pops an unstructured argument from the back of the list without returning
/// it.
#[no_mangle]
pub extern "C" fn dqcs_arb_pop(arb: dqcs_handle_t) -> dqcs_return_t {
    if dqcs_arb_pop_raw(arb, null_mut(), 0) < 0 {
        dqcs_return_t::DQCS_FAILURE
    } else {
        dqcs_return_t::DQCS_SUCCESS
    }
}

/// Inserts an unstructured string argument into the list at the specified
/// index.
#[no_mangle]
pub extern "C" fn dqcs_arb_insert_str(
    arb: dqcs_handle_t,
    index: ssize_t,
    s: *const c_char,
) -> dqcs_return_t {
    api_return_none(|| {
        resolve!(arb as &mut ArbData);
        let len = arb.get_args().len();
        arb.get_args_mut().insert(
            receive_index(len, index, true)?,
            receive_str(s)?.bytes().collect(),
        );
        Ok(())
    })
}

/// Inserts an unstructured raw argument into the list at the specified
/// index.
#[no_mangle]
pub extern "C" fn dqcs_arb_insert_raw(
    arb: dqcs_handle_t,
    index: ssize_t,
    obj: *const c_void,
    obj_size: size_t,
) -> dqcs_return_t {
    api_return_none(|| {
        resolve!(arb as &mut ArbData);
        let len = arb.get_args().len();
        arb.get_args_mut().insert(
            receive_index(len, index, true)?,
            receive_raw(obj, obj_size)?.to_owned(),
        );
        Ok(())
    })
}

/// Removes the specified unstructured string argument from the list.
#[no_mangle]
pub extern "C" fn dqcs_arb_remove(arb: dqcs_handle_t, index: ssize_t) -> dqcs_return_t {
    api_return_none(|| {
        resolve!(arb as &mut ArbData);
        let len = arb.get_args().len();
        arb.get_args_mut().remove(receive_index(len, index, false)?);
        Ok(())
    })
}

/// Replaces the unstructured argument at the specified index with the
/// specified string.
#[no_mangle]
pub extern "C" fn dqcs_arb_set_str(
    arb: dqcs_handle_t,
    index: ssize_t,
    s: *const c_char,
) -> dqcs_return_t {
    api_return_none(|| {
        resolve!(arb as &mut ArbData);
        let mut s: Vec<u8> = receive_str(s)?.bytes().collect();
        let index = receive_index(arb.get_args().len(), index, false)?;
        let arg = &mut arb.get_args_mut()[index];
        arg.clear();
        arg.append(&mut s);
        Ok(())
    })
}

/// Replaces the unstructured argument at the specified index with the
/// specified raw object.
#[no_mangle]
pub extern "C" fn dqcs_arb_set_raw(
    arb: dqcs_handle_t,
    index: ssize_t,
    obj: *const c_void,
    obj_size: size_t,
) -> dqcs_return_t {
    api_return_none(|| {
        resolve!(arb as &mut ArbData);
        let mut s: Vec<u8> = receive_raw(obj, obj_size)?.to_owned();
        let index = receive_index(arb.get_args().len(), index, false)?;
        let arg = &mut arb.get_args_mut()[index];
        arg.clear();
        arg.append(&mut s);
        Ok(())
    })
}

/// Returns the unstructured string argument at the specified index.
///
/// On success, this **returns a newly allocated string containing the JSON
/// string. Free it with `free()` when you're done with it to avoid memory
/// leaks.** On failure, this returns `NULL`.
#[no_mangle]
pub extern "C" fn dqcs_arb_get_str(arb: dqcs_handle_t, index: ssize_t) -> *mut c_char {
    api_return_string(|| {
        resolve!(arb as &ArbData);
        Ok(String::from_utf8(
            arb.get_args()[receive_index(arb.get_args().len(), index, false)?].clone(),
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
    arb: dqcs_handle_t,
    index: ssize_t,
    obj: *mut c_void,
    obj_size: size_t,
) -> ssize_t {
    api_return(-1, || {
        resolve!(arb as &ArbData);
        return_raw(
            &arb.get_args()[receive_index(arb.get_args().len(), index, false)?],
            obj,
            obj_size,
        )
    })
}

/// Returns the size in bytes of the unstructured string argument at the
/// specified index.
///
/// Returns -1 when the function fails.
#[no_mangle]
pub extern "C" fn dqcs_arb_get_size(arb: dqcs_handle_t, index: ssize_t) -> ssize_t {
    dqcs_arb_get_raw(arb, index, null_mut(), 0)
}

/// Returns the number of unstructured arguments, or -1 to indicate failure.
#[no_mangle]
pub extern "C" fn dqcs_arb_len(arb: dqcs_handle_t) -> ssize_t {
    api_return(-1, || {
        resolve!(arb as &ArbData);
        Ok(arb.get_args().len() as ssize_t)
    })
}

/// Clears the unstructured argument list.
#[no_mangle]
pub extern "C" fn dqcs_arb_clear(arb: dqcs_handle_t) -> dqcs_return_t {
    api_return_none(|| {
        resolve!(arb as &mut ArbData);
        arb.get_args_mut().clear();
        Ok(())
    })
}

/// Copies the data from one object to another.
#[no_mangle]
pub extern "C" fn dqcs_arb_assign(dest: dqcs_handle_t, src: dqcs_handle_t) -> dqcs_return_t {
    api_return_none(|| {
        resolve!(src as &ArbData);
        resolve!(dest as &mut ArbData);
        dest.set_cbor_unchecked(src.get_cbor());
        dest.set_args(src.get_args());
        Ok(())
    })
}
