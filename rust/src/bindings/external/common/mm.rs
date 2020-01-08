use super::*;
use crate::common::gates::{GateType, UnboundGate};
use std::rc::Rc;

/// Constructs a new matrix map builder.
///>
///> Returns a handle to a matrix map builder with no detectors attached to it
///> yet. Use `dqcs_mmb_add_*()` to do that. The detectors are queried in the
///> order in which they are added, so be sure to add more specific gates first.
///> Then construct the matrix map object itself using `dqcs_mm_new()`.
#[no_mangle]
pub extern "C" fn dqcs_mmb_new() -> dqcs_handle_t {
    insert(MatrixMapBuilderC::new())
}

/// Adds a gate matrix detector for the given DQCsim-defined gate to the given
/// matrix map builder.
///>
///> `mmb` must be a handle to a matrix map builder object (`dqcs_mmb_new()`).
///> `key_data` is a user-specified value that is returned as part of the match
///> function result when this detector is the first to match. `key_free` is an
///> optional callback function used to free `key_data` when the matrix map
///> (builder) is destroyed, or when this function fails. `gate` defines
///> which gate to detect. Some of the detectable gates are parameterized. Note
///> that there are no controlled gates, because they are normally encoded
///> separately in the DQCsim protocol and therefore not part of the matrix; to
///> detect gate matrices with explicit control qubits, preprocess the gates
///> with `dqcs_gate_reduce_control()` first. `epsilon` specifies the maximum
///> element-wise root-mean-square error between the incoming matrix and the to
///> be detected matrix that results in a positive match. `ignore_phase`
///> specifies whether the aforementioned check should ignore global phase or
///> not. Most gate detectors return an empty ArbData object as they are not
///> parameterized. The following exceptions exist:
///>
///>  - `DQCS_GATE_RX`, `DQCS_GATE_RY`, and `DQCS_GATE_RZ` push a 64-bit double
///>    floating point with the angle.
///>  - `DQCS_GATE_RK` pushes a 32-bit integer with the k parameter.
///>  - `DQCS_GATE_R` pushes a the three double floating point angles (ordered
///>    theta, phi, lambda).
///>  - `DQCS_GATE_U` pushes the entire matrix as a single argument consisting
///>    of 2**N * 2**N * 2 doubles, in real-first row-major format (same as the
///>    other matrix definitions in DQCsim).
#[no_mangle]
pub extern "C" fn dqcs_mmb_add_internal(
    mmb: dqcs_handle_t,
    key_free: Option<extern "C" fn(user_data: *mut c_void)>,
    key_data: *mut c_void,
    gate: dqcs_internal_gate_t,
    epsilon: c_double,
    ignore_gphase: bool,
) -> dqcs_return_t {
    api_return_none(|| {
        resolve!(mmb as &mut MatrixMapBuilderC);
        let key = Rc::new(UserData::new(key_free, key_data));
        if let Some(gate) = Into::<Option<GateType>>::into(gate) {
            let detector = gate.into_detector(epsilon, ignore_gphase);
            mmb.add_detector(
                key,
                Box::new(move |input: &Matrix| -> Result<Option<ArbData>> {
                    detector(input).map(|gate| {
                        gate.map(|gate| match gate {
                            UnboundGate::RX(theta) => {
                                ArbData::from_args(vec![theta.to_ne_bytes().to_vec()])
                            }
                            UnboundGate::RY(theta) => {
                                ArbData::from_args(vec![theta.to_ne_bytes().to_vec()])
                            }
                            UnboundGate::RK(k) => {
                                ArbData::from_args(vec![(k as u32).to_ne_bytes().to_vec()])
                            }
                            UnboundGate::RZ(theta) => {
                                ArbData::from_args(vec![theta.to_ne_bytes().to_vec()])
                            }
                            UnboundGate::R(theta, phi, lambda) => ArbData::from_args(
                                vec![
                                    theta.to_ne_bytes().to_vec(),
                                    phi.to_ne_bytes().to_vec(),
                                    lambda.to_ne_bytes().to_vec(),
                                ]
                                .to_vec(),
                            ),
                            UnboundGate::U(matrix) => ArbData::from_args(
                                matrix
                                    .into_iter()
                                    .map(|c| {
                                        let mut re = c.re.to_ne_bytes().to_vec();
                                        re.append(&mut c.im.to_ne_bytes().to_vec());
                                        re
                                    })
                                    .collect::<Vec<Vec<u8>>>(),
                            ),
                            _ => ArbData::default(),
                        })
                    })
                }),
            );
            Ok(())
        } else {
            inv_arg("invalid gate")
        }
    })
}

/// Adds a gate matrix detector for the given gate matrix to the given
/// matrix map builder.
///>
///> `mmb` must be a handle to a matrix map builder object (`dqcs_mmb_new()`).
///> `key_data` is a user-specified value that is returned as part of the match
///> function result when this detector is the first to match. `key_free` is an
///> optional callback function used to free `key_data` when the matrix map
///> (builder) is destroyed, or when this function fails. `matrix` must
///> point to an appropriately sized array of doubles, representing the unitary
///> matrix that is to be detected. The matrix is specified in row-major form,
///> using pairs of doubles for the real vs. imaginary component of each entry.
///> `matrix_len` must be set to the number of complex numbers in the matrix
///> (4 for a one-qubit gate, 16 for a two-qubit gate, 256 for a three-qubit
///> gate and so on). `epsilon` specifies the maximum element-wise
///> root-mean-square error between the incoming matrix and the to be detected
///> matrix that results in a positive match. `ignore_phase` specifies whether
///> the aforementioned check should ignore global phase or not. The generated
///> gate detector always returns an empty ArbData object for the parameter
///> data as it is not parameterized.
#[no_mangle]
pub extern "C" fn dqcs_mmb_add_fixed(
    mmb: dqcs_handle_t,
    key_free: Option<extern "C" fn(key_data: *mut c_void)>,
    key_data: *mut c_void,
    matrix: *const c_double,
    matrix_len: size_t,
    epsilon: c_double,
    ignore_gphase: bool,
) -> dqcs_return_t {
    api_return_none(|| {
        resolve!(mmb as &mut MatrixMapBuilderC);
        let key = Rc::new(UserData::new(key_free, key_data));
        let matrix: Matrix = receive_matrix_raw(matrix, matrix_len)?
            .ok_or_else(oe_inv_arg("empty matrix"))?
            .into();
        mmb.add_detector(
            key,
            matrix.into_detector(epsilon, ignore_gphase, ArbData::default()),
        );
        Ok(())
    })
}

/// Adds a custom gate matrix detector to the given matrix map builder.
///>
///> `mmb` must be a handle to a matrix map builder object (`dqcs_mmb_new()`).
///> `key_data` is a user-specified value that is returned as part of the match
///> function result when this detector is the first to match. `key_free` is an
///> optional callback function used to free `key_data` when the matrix map
///> (builder) is destroyed, or when this function fails.
///>
///> The callback receives a matrix for the user to match. If the gate matches,
///> the function must return `DQCS_TRUE`. If it doesn't match, it must return
///> `DQCS_FALSE`. If an error occurs, it must call `dqcs_error_set()` with the
///> error message and return `DQCS_BOOL_FAILURE`.
///>
///> When the gate matches, the function can return a gate parameter data object
///> by way of assigning `param_data` to a new `ArbData` handle. Ownership of
///> this handle is passed to the callee regardless of the return value (that is,
///> even if no match is reported, the callee will ensure that the `ArbData`
///> handle passed here, if any, is freed). `param_data` will always point to 0
///> when the function is called.
///>
///> It is up to the user how to do this matching, but the function is assumed
///> to always return the same value for the same input. Otherwise, the caching
///> behavior of the `GateMap` will make the results inconsistent.
#[no_mangle]
pub extern "C" fn dqcs_mmb_add_user(
    mmb: dqcs_handle_t,
    key_free: Option<extern "C" fn(key_data: *mut c_void)>,
    key_data: *mut c_void,
    callback: Option<
        extern "C" fn(
            user_data: *const c_void,
            matrix: *const c_double,
            matrix_len: size_t,
            param_data: *mut dqcs_handle_t,
        ) -> dqcs_bool_return_t,
    >,
    user_free: Option<extern "C" fn(user_data: *mut c_void)>,
    user_data: *mut c_void,
) -> dqcs_return_t {
    api_return_none(|| {
        resolve!(mmb as &mut MatrixMapBuilderC);
        let key = Rc::new(UserData::new(key_free, key_data));
        let user = UserData::new(user_free, user_data);
        let callback = callback.ok_or_else(oe_inv_arg("callback cannot be null"))?;
        mmb.add_detector(
            key,
            Box::new(move |input: &Matrix| -> Result<Option<ArbData>> {
                let mut handle = 0;
                let res: Result<bool> = cb_return_bool(callback(
                    user.data(),
                    input.as_ptr(),
                    input.len(),
                    &mut handle as *mut dqcs_handle_t,
                ));
                match res {
                    Ok(true) => Ok({
                        if handle != 0 {
                            take!(handle as ArbData);
                            Some(handle)
                        } else {
                            Some(ArbData::default())
                        }
                    }),
                    Err(x) => {
                        if handle != 0 {
                            delete!(handle);
                        }
                        Err(x)
                    }
                    Ok(false) => {
                        if handle != 0 {
                            delete!(handle);
                        }
                        Ok(None)
                    }
                }
            }),
        );
        Ok(())
    })
}

/// Finalizes a matrix map builder object into a matrix map object.
///>
///> `mmb` must be a handle to a matrix map builder object (`dqcs_mmb_new()`).
///> This handle is consumed if the function succeeds.
///>
///> This function returns a new handle to a matrix map object, which can then
///> be used to map matrices to user-specified keys.
#[no_mangle]
pub extern "C" fn dqcs_mm_new(mmb: dqcs_handle_t) -> dqcs_handle_t {
    api_return(0, || {
        take!(mmb as MatrixMapBuilderC);
        Ok(insert(mmb.finish()))
    })
}

/// Uses a matrix map object to map the given matrix to some key representing
/// the gate type.
///>
///> `mm` must be a handle to a matrix map object (`dqcs_mm_new()`). `matrix`
///> must point to an appropriately sized array of doubles, representing the
///> unitary matrix that is to be matched. The matrix is specified in row-major
///> form, using pairs of doubles for the real vs. imaginary component of each
///> entry. `matrix_len` must be set to the number of complex numbers in the
///> matrix (4 for a one-qubit gate, 16 for a two-qubit gate, 256 for a
///> three-qubit gate and so on). `key_data` serves as an optional return
///> value; if non-NULL and a match is found, the `key_data` specified
///> when the respective matcher was added is returned here as a `const void *`.
///> If no match is found, `key_data` is not written to. `param_data` also
///> serves as an optional return value; if non-NULL and a match is found,
///> it is set to a handle to a new `ArbData` object representing the gate's
///> parameters. Ownership of this object is passed to the user, so it is up
///> to the user to eventually delete this object. If no match is found,
///> `param_data` is set to 0. This function returns `DQCS_TRUE` if a match was
///> found, `DQCS_FALSE` if no match was found, or `DQCS_BOOL_FAILURE` if an
///> error occurs.
#[no_mangle]
pub extern "C" fn dqcs_mm_map_matrix(
    mm: dqcs_handle_t,
    matrix: *const c_double,
    matrix_len: size_t,
    key_data: *mut *const c_void,
    param_data: *mut dqcs_handle_t,
) -> dqcs_bool_return_t {
    api_return_bool(|| {
        if !param_data.is_null() {
            unsafe { *param_data = 0 };
        }
        resolve!(mm as &MatrixMapC);
        let matrix: Matrix = receive_matrix_raw(matrix, matrix_len)?
            .ok_or_else(oe_inv_arg("empty matrix"))?
            .into();
        let detect = mm.detect(&matrix)?;
        match detect {
            Some((key, param)) => {
                if !key_data.is_null() {
                    unsafe { *key_data = key.data() };
                }
                if !param_data.is_null() {
                    unsafe { *param_data = insert(param) };
                }
                Ok(true)
            }
            None => Ok(false),
        }
    })
}

/// Uses a matrix map object to map the matrix of the given gate to some key
/// representing the gate type.
///>
///> `mm` must be a handle to a matrix map object (`dqcs_mm_new()`). `gate` must
///> be a handle to a gate that has a matrix (that is, either a unitary gate, or
///> a custom gate with a matrix). `key_data` serves as an optional return
///> value; if non-NULL and a match is found, the `key_data` specified
///> when the respective matcher was added is returned here as a `const void *`.
///> If no match is found, `key_data` is not written to. `param_data` also
///> serves as an optional return value; if non-NULL and a match is found,
///> it is set to a handle to a new `ArbData` object representing the gate's
///> parameters. Ownership of this object is passed to the user, so it is up
///> to the user to eventually delete this object. If no match is found,
///> `param_data` is set to 0. This function returns `DQCS_TRUE` if a match was
///> found, `DQCS_FALSE` if no match was found, or `DQCS_BOOL_FAILURE` if an
///> error occurs.
#[no_mangle]
pub extern "C" fn dqcs_mm_map_gate(
    mm: dqcs_handle_t,
    gate: dqcs_handle_t,
    key_data: *mut *const c_void,
    param_data: *mut dqcs_handle_t,
) -> dqcs_bool_return_t {
    api_return_bool(|| {
        if !param_data.is_null() {
            unsafe { *param_data = 0 };
        }
        resolve!(mm as &MatrixMapC);
        resolve!(gate as &Gate);
        match mm.detect(
            &gate
                .get_matrix()
                .ok_or_else(oe_inv_arg("gate has no matrix"))?,
        )? {
            Some((key, param)) => {
                if !key_data.is_null() {
                    unsafe { *key_data = key.data() };
                }
                if !param_data.is_null() {
                    unsafe { *param_data = insert(param) };
                }
                Ok(true)
            }
            None => Ok(false),
        }
    })
}

/// Clears the cache of a matrix map object.
///>
///> Matrix map objects internally cache detected matrices to increase detection
///> speed when a matrix is received a second time. This function clears the
///> cache.
#[no_mangle]
pub extern "C" fn dqcs_mm_clear_cache(mm: dqcs_handle_t) -> dqcs_return_t {
    api_return_none(|| {
        resolve!(mm as &MatrixMapC);
        mm.clear_cache();
        Ok(())
    })
}

/// Returns whether the two supplied matrices are approximately equal, or
/// approximately equivalent when ignoring global phase.
///>
///> The global phase reduction works by gathering the weighted average of the
///> element-wise phase differences between the two matrices (weighted by the
///> product of the element-wise magnitudes) and applying the inverse to one
///> of the matrices before the comparison algorithm. The method used to do this
///> is only guaranteed to be numerically stable when the matrices are unitary.
///> The comparison itself is based on computing the element-wise
///> root-mean-square difference between the matrices, and comparing this
///> difference against `epsilon`.
///>
///> `matrix_a` and `matrix_b` must point to an appropriately sized array of
///> doubles, representing the unitary matrices that are to be compared. The
///> matrices are specified in row-major form, using pairs of doubles for the
///> real vs. imaginary component of each entry. `matrix_len` must be set to the
///> number of complex numbers in each matrix (4 for a one-qubit gate, 16 for a
///> two-qubit gate, 256 for a three-qubit gate and so on). `epsilon` specifies
///> the maximum element-wise root-mean-square error between the two matrices
///> for them to be considered equal. `ignore_phase` specifies whether
///> the aforementioned check should ignore global phase or not.
#[no_mangle]
pub extern "C" fn dqcs_mat_compare(
    matrix_a: *const c_double,
    matrix_b: *const c_double,
    matrix_len: size_t,
    epsilon: c_double,
    ignore_gphase: bool,
) -> dqcs_bool_return_t {
    api_return_bool(|| {
        let a: Matrix = receive_matrix_raw(matrix_a, matrix_len)?
            .ok_or_else(oe_inv_arg("empty matrix"))?
            .into();
        let b: Matrix = receive_matrix_raw(matrix_b, matrix_len)?
            .ok_or_else(oe_inv_arg("empty matrix"))?
            .into();
        Ok(a.approx_eq(&b, epsilon, ignore_gphase))
    })
}
