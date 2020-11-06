use super::*;
use crate::common::gates::UnitaryGateType;
use std::convert::{TryFrom, TryInto};
use std::mem::size_of;
use std::ptr::null_mut;

/// Constructs a new gate matrix.
///>
///> `num_qubits` must be set to the number of qubits mutated by this matrix.
///> It must be greater than or equal to zero.
///> `matrix` must point to an appropriately sized array of doubles. The matrix
///> is specified in row-major form, using pairs of doubles for the real vs.
///> imaginary component of each entry. The size must be `4**num_qubits` complex
///> numbers = `2*4**num_qubits` doubles = `16*4**num_qubits` bytes,
///> representing a `2**num_qubits` by `2**num_qubits` matrix.
///> This function returns the constructed matrix handle, or 0 if an error
///> occurs.
///>
///> While not enforced at this level, the matrix is normally unitary, or
///> approximately so within some floating-point error margin.
///>
///> This function returns the handle to the matrix, or 0 to indicate failure.
#[no_mangle]
pub extern "C" fn dqcs_mat_new(num_qubits: size_t, matrix: *const c_double) -> dqcs_handle_t {
    api_return(0, || {
        if num_qubits == 0 {
            inv_arg("cannot construct matrix for 0 qubits")
        } else {
            let num_entries = 4usize.pow(num_qubits as u32);
            let mut vec = Vec::with_capacity(num_entries);
            for i in 0..num_entries {
                let re: f64 = unsafe { *matrix.add(i * 2) };
                let im: f64 = unsafe { *matrix.add(i * 2 + 1) };
                vec.push(Complex64::new(re, im));
            }
            Ok(insert(Matrix::new(vec)?))
        }
    })
}

/// Constructs a new gate matrix for one of DQCsim's predefined gates.
///>
///> `gate_type` specifies which kind of gate should be constructed.
///>
///> `param_data` takes an optional `ArbData` object used to parameterize the
///> matrix if necessary. If not specified, an empty object is used. The
///> `ArbData` representation for each gate can be found in the docs for
///> `dqcs_predefined_gate_t`. If nothing is specified, no `ArbData` is used.
///>
///> This function returns the handle to the matrix, or 0 to indicate failure.
///> The parameterization data (if specified) is consumed/deleted by this
///> function if and only if it succeeds.
#[no_mangle]
pub extern "C" fn dqcs_mat_predef(
    gate_type: dqcs_predefined_gate_t,
    param_data: dqcs_handle_t,
) -> dqcs_handle_t {
    api_return(0, || {
        // Interpret gate type.
        let converter: Box<dyn MatrixConverterArb> = UnitaryGateType::try_from(gate_type)?.into();

        // Interpret data.
        resolve!(optional param_data as pending ArbData);
        let mut data: ArbData = {
            if let Some(data) = param_data.as_ref() {
                let x: &ArbData = data.as_ref().unwrap();
                x.clone()
            } else {
                ArbData::default()
            }
        };

        // Construct the gate.
        let matrix = insert(converter.construct_matrix_arb(&mut data)?);

        // Delete consumed handles.
        if let Some(mut param_data) = param_data {
            delete!(resolved param_data);
        }

        Ok(matrix)
    })
}

/// Constructs a matrix with the eigenvectors of one of the Pauli matrices
/// as column vectors.
///>
///> This can be used for constructing measurement or prep gates with the
///> given basis. Returns a new handle to the constructed matrix or returns
///> 0 if an error occurs.
#[no_mangle]
pub extern "C" fn dqcs_mat_basis(basis: dqcs_basis_t) -> dqcs_handle_t {
    api_return(0, || {
        let matrix: Matrix = Basis::try_from(basis)?.into();
        Ok(insert(matrix))
    })
}

/// Returns the number of complex entries in the given matrix.
///>
///> This function returns -1 when an error occurs.
#[no_mangle]
pub extern "C" fn dqcs_mat_len(mat: dqcs_handle_t) -> ssize_t {
    api_return(-1, || {
        resolve!(mat as &Matrix);
        Ok(mat.len().try_into().unwrap())
    })
}

/// Returns the dimension (number of rows == number of columns) of the given
/// matrix.
///>
///> This function returns -1 when an error occurs.
#[no_mangle]
pub extern "C" fn dqcs_mat_dimension(mat: dqcs_handle_t) -> ssize_t {
    api_return(-1, || {
        resolve!(mat as &Matrix);
        Ok(mat.dimension().try_into().unwrap())
    })
}

/// Returns the number of qubits targeted by the given matrix.
///>
///> This function returns -1 when an error occurs.
#[no_mangle]
pub extern "C" fn dqcs_mat_num_qubits(mat: dqcs_handle_t) -> ssize_t {
    api_return(-1, || {
        resolve!(mat as &Matrix);
        let num_qubits = mat
            .num_qubits()
            .ok_or_else(oe_inv_arg("corrupted internal matrix size"))?;
        Ok(num_qubits.try_into().unwrap())
    })
}

/// Returns a copy of the contained matrix as a C array.
///>
///> If this function succeeds, the matrix is returned in row-major form, using
///> pairs of doubles for the real vs. imaginary component of each entry. The
///> size will be `4**num_qubits` complex numbers = `2*4**num_qubits` doubles =
///> `16*4**num_qubits` bytes. A newly allocated matrix is returned; **free it
///> with `free()` when you're done with it to avoid memory leaks.** On failure,
///> this function returns `NULL`.
#[no_mangle]
pub extern "C" fn dqcs_mat_get(mat: dqcs_handle_t) -> *mut c_double {
    api_return(null_mut(), || {
        resolve!(mat as &Matrix);
        let ffi_matrix = unsafe { calloc(2 * mat.len(), size_of::<c_double>()) as *mut c_double };
        if ffi_matrix.is_null() {
            err("failed to allocate return value")
        } else {
            unsafe {
                memcpy(
                    ffi_matrix as *mut c_void,
                    mat.as_ptr() as *const c_void,
                    2 * mat.len() * size_of::<c_double>(),
                )
            };
            Ok(ffi_matrix)
        }
    })
}

/// Approximately compares two matrices.
///>
///> `a` and `b` are borrowed matrix handles.
///> `epsilon` specifies the maximum element-wise root-mean-square error
///> between the matrices that results in a positive match. `ignore_gphase`
///> specifies whether the check should ignore global phase.
///>
///> If ignore_gphase is set, this checks that the following holds for some x:
///>
///> \f[
///> A \cdot e^{ix} \approx B
///> \f]
///>
///> This function returns `DQCS_TRUE` if the matrices match according to the
///> aforementioned criteria, or `DQCS_FALSE` if not. `DQCS_BOOL_ERROR` is used
///> when either handle is invalid or not a matrix. If the matrices differ in
///> dimensionality, `DQCS_FALSE` is used.
#[no_mangle]
pub extern "C" fn dqcs_mat_approx_eq(
    a: dqcs_handle_t,
    b: dqcs_handle_t,
    epsilon: c_double,
    ignore_gphase: bool,
) -> dqcs_bool_return_t {
    api_return_bool(|| {
        resolve!(a as &Matrix);
        resolve!(b as &Matrix);
        Ok(a.approx_eq(b, epsilon, ignore_gphase))
    })
}

/// Approximately compares two basis matrices.
///>
///> `a` and `b` are borrowed matrix handles.
///> `epsilon` specifies the maximum element-wise root-mean-square error
///> between the matrices that results in a positive match.
///>
///> This checks that the following holds for some x and y:
///>
///> \f[
///> A \cdot \begin{bmatrix}
///> e^{ix} & 0 \\
///> 0 & e^{iy}
///> \end{bmatrix} \approx B
///> \f]
///>
///> This function returns `DQCS_TRUE` if the matrices match according to the
///> aforementioned criteria, or `DQCS_FALSE` if not. `DQCS_BOOL_ERROR` is used
///> when either handle is invalid or not a matrix. If either matrix is not
///> 2x2, `DQCS_FALSE` is used.
#[no_mangle]
pub extern "C" fn dqcs_mat_basis_approx_eq(
    a: dqcs_handle_t,
    b: dqcs_handle_t,
    epsilon: c_double,
) -> dqcs_bool_return_t {
    api_return_bool(|| {
        resolve!(a as &Matrix);
        resolve!(b as &Matrix);
        Ok(a.basis_approx_eq(b, epsilon))
    })
}

/// Returns whether the matrix is approximately unitary.
///>
///> `matrix` is a borrowed handle to the matrix to check.
///> `epsilon` specifies the maximum element-wise root-mean-square error
///> between the product of the matrix and its hermetian compared to the
///> identity matrix.
///>
///> This function returns `DQCS_TRUE` if the matrix is approximately unitary,
///> or `DQCS_FALSE` if not. `DQCS_BOOL_ERROR` is used when either handle is
///> invalid or not a matrix.
#[no_mangle]
pub extern "C" fn dqcs_mat_approx_unitary(
    matrix: dqcs_handle_t,
    epsilon: c_double,
) -> dqcs_bool_return_t {
    api_return_bool(|| {
        resolve!(matrix as &Matrix);
        Ok(matrix.approx_unitary(epsilon))
    })
}

/// Returns whether this matrix is of the given predefined form and, if it is,
/// any parameters needed to describe it.
///>
///> `mat` is a borrowed handle to the matrix to check.
///> `gate_type` specifies which kind of gate should be detected.
///> `param_data`, if non-null, receives a new `ArbData` handle with
///> parameterization data, or an empty `ArbData` if the gate is not
///> parameterized; the caller must delete this object when it is done with
///> it. This function always writes the 0 handle to this return parameter if
///> it fails. The `ArbData` representation can be found in the documentation
///> for `dqcs_predefined_gate_t`.
///>
///> `epsilon` specifies the maximum element-wise root-mean-square error
///> between the matrices that results in a positive match. `ignore_gphase`
///> specifies whether the check should ignore global phase.
///>
///> This function returns `DQCS_TRUE` if the matrices match according to the
///> aforementioned criteria, or `DQCS_FALSE` if not. `DQCS_BOOL_ERROR` is used
///> when either handle is invalid or not a matrix. If the matrices differ in
///> dimensionality, `DQCS_FALSE` is used.
#[no_mangle]
pub extern "C" fn dqcs_mat_is_predef(
    mat: dqcs_handle_t,
    gate_type: dqcs_predefined_gate_t,
    param_data: *mut dqcs_handle_t,
    epsilon: c_double,
    ignore_gphase: bool,
) -> dqcs_bool_return_t {
    api_return_bool(|| {
        if !param_data.is_null() {
            unsafe { *param_data = 0 };
        }
        resolve!(mat as &Matrix);
        let converter: Box<dyn MatrixConverterArb> = UnitaryGateType::try_from(gate_type)?.into();
        let mut data = ArbData::default();
        let result = converter.detect_matrix_arb(mat, epsilon, ignore_gphase, &mut data)?;
        if !param_data.is_null() {
            let handle = insert(data);
            unsafe { *param_data = handle };
        }
        Ok(result)
    })
}

/// Constructs a controlled matrix from the given matrix.
///>
///> `mat` specifies the matrix to use as the non-controlled submatrix. This
///> is a borrowed handle. `number_of_controls` specifies the number of control
///> qubits to add. This function returns a new matrix handle with the
///> constructed matrix, or 0 if it fails.
#[no_mangle]
pub extern "C" fn dqcs_mat_add_controls(
    mat: dqcs_handle_t,
    number_of_controls: size_t,
) -> dqcs_handle_t {
    api_return(0, || {
        resolve!(mat as &Matrix);
        Ok(insert(mat.add_controls(number_of_controls)))
    })
}

/// Splits a controlled matrix into its non-controlled submatrix and the
/// indices of the control qubits.
///>
///> `mat` specifies the matrix to modify. This is a borrowed handle.
///> `epsilon` specifies the maximum magitude of the difference between the
///> column vectors of the input matrix and the identity matrix (after
///> dephasing if `ignore_gphase` is set) for the column vector to be
///> considered to not affect the respective entry in the quantum state
///> vector. Note that if this is greater than zero, the resulting gate may
///> not be exactly equivalent. If `ignore_global_phase` is set, any global
///> phase in the matrix is ignored, but note that if control qubits are
///> stripped the "global" phase of the resulting submatrix is always
///> significant.
///> `control_indices` is a return argument through which DQCsim will pass
///> the indices of the qubits that were removed in the process of constructing
///> the submatrix. This is represented as an array of indices terminated by
///> a -1 entry. The returned matrix **must be freed using `free()` when you
///> are done with it to avoid memory leaks.** This function returns a new
///> matrix handle with the submatrix, or 0 if it fails. In this case,
///> `control_indices` is not mutated.
///>
///> This function assumes that the incoming matrix is unitary (within
///> `epsilon`) without verifying that this is the case. The results may
///> thus be invalid if it was not.
#[no_mangle]
pub extern "C" fn dqcs_mat_strip_control(
    mat: dqcs_handle_t,
    epsilon: c_double,
    ignore_global_phase: bool,
    control_indices: *mut *mut ssize_t,
) -> dqcs_handle_t {
    api_return(0, || {
        resolve!(mat as &Matrix);
        if control_indices.is_null() {
            err("control_indices cannot be null")
        } else {
            let (control_index_hashset, submatrix) =
                mat.strip_control(epsilon, ignore_global_phase);
            let mut control_index_vec: Vec<usize> = control_index_hashset.into_iter().collect();
            control_index_vec.sort_unstable();
            let control_index_ffi = unsafe {
                calloc(control_index_vec.len() + 1, size_of::<ssize_t>()) as *mut ssize_t
            };
            if control_index_ffi.is_null() {
                err("failed to allocate control indices")
            } else {
                unsafe {
                    memcpy(
                        control_index_ffi as *mut c_void,
                        control_index_vec.as_ptr() as *const c_void,
                        control_index_vec.len() * size_of::<ssize_t>(),
                    );
                    *control_index_ffi.add(control_index_vec.len()) = -1;
                    *control_indices = control_index_ffi;
                }
                Ok(insert(submatrix))
            }
        }
    })
}
