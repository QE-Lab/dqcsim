use super::*;
use std::mem::size_of;
use std::ptr::null_mut;

/// Constructs a new unitary gate.
///
/// `targets` must be a handle to a non-empty qubit set. The qubits in this set
/// correspond with the supplied unitary matrix.
///
/// `controls` optionally specifies a set of control qubits. You may pass 0 or
/// an empty qubit set if you don't need control qubits.
///
/// `matrix` must point to an appropriately sized array of doubles,
/// representing the unitary matrix to be applied to the qubits in the
/// `targets` set. The matrix is specified in row-major form, using pairs of
/// doubles for the real vs. imaginary component of each entry. The size must
/// thus be `4**len(targets)` complex numbers = `2*4**len(targets)` doubles =
/// `16*4**len(targets)` bytes. `matrix_len` must be set to the number of
/// complex numbers.
///
/// The supplied matrix is only applied to the target qubits if all the control
/// qubits are or will be determined to be set. For instance, to encode a
/// CCNOT/Toffoli gate, you can specify one target qubits, two control qubits,
/// and [0, 1; 1, 0] (X) for the matrix. This is equivalent to extending the
/// matrix to the full Toffoli matrix and specifying all three qubits in the
/// targets set, or the midway solution using a CNOT matrix, but these
/// solutions may be less efficient depending on whether the simulator can
/// optimize its calculations for controlled gates.
///
/// Simulators are not required to apply the (hidden) global phase component of
/// the gate matrix in the same way it is specified; that is, if the simulator
/// can optimize its calculations by altering the global phase it is allowed
/// to.
///
/// If is up to the user to ensure that the specified matrix is unitary. This
/// is NOT checked by DQCsim. The simulator backend may or may not check this.
///
/// This function returns the handle to the gate, or 0 to indicate failure.
/// The `targets` qubit set and (if specified) the `controls` qubit set are
/// consumed/deleted by this function if and only if it succeeds.
#[no_mangle]
pub extern "C" fn dqcs_gate_new_unitary(
    targets: dqcs_handle_t,
    controls: dqcs_handle_t,
    matrix: *const c_double,
    matrix_len: size_t,
) -> dqcs_handle_t {
    api_return(0, || {
        // Interpret target set.
        resolve!(targets as pending QubitReferenceSet);
        let target_vec: Vec<QubitRef> = {
            let x: &QubitReferenceSet = targets.as_ref().unwrap();
            x.iter().cloned().collect()
        };

        // Interpret control set.
        resolve!(optional controls as pending QubitReferenceSet);
        let control_vec: Vec<QubitRef> = {
            if let Some(controls) = controls.as_ref() {
                let x: &QubitReferenceSet = controls.as_ref().unwrap();
                x.iter().cloned().collect()
            } else {
                vec![]
            }
        };

        // Interpret matrix.
        let matrix = receive_matrix(matrix, matrix_len, target_vec.len())?
            .ok_or_else(oe_inv_arg("the unitary matrix cannot be null"))?;

        // Construct the gate.
        let gate = insert(Gate::new_unitary(target_vec, control_vec, matrix)?);

        // Everything went OK. Now make sure that the target and control set
        // handles are deleted.
        take!(resolved targets as QubitReferenceSet);
        let _ = targets;
        if let Some(mut controls) = controls {
            take!(resolved controls as QubitReferenceSet);
            let _ = controls;
        }
        Ok(gate)
    })
}

/// Constructs a new measurement gate.
///
/// `measures` must be a handle to a qubit set. The qubits in this set are
/// measured in the Z-basis. To measure in other bases, first apply the
/// respective rotation, or use a custom gate.
///
/// This function returns the handle to the gate, or 0 to indicate failure.
/// The `measures` qubit set is consumed/deleted by this function if and only
/// if it succeeds.
#[no_mangle]
pub extern "C" fn dqcs_gate_new_measurement(measures: dqcs_handle_t) -> dqcs_handle_t {
    api_return(0, || {
        // Interpret measures set.
        resolve!(measures as pending QubitReferenceSet);
        let measure_vec: Vec<QubitRef> = {
            let x: &QubitReferenceSet = measures.as_ref().unwrap();
            x.iter().cloned().collect()
        };

        // Construct the gate.
        let gate = insert(Gate::new_measurement(measure_vec)?);

        // Everything went OK. Now make sure that the measure set handle is
        // deleted.
        take!(resolved measures as QubitReferenceSet);
        let _ = measures;
        Ok(gate)
    })
}

/// Constructs a new custom gate.
///
/// The functionality of custom gates is not specified by DQCsim. Instead, this
/// is left up to the plugins. Of course, for this to work, plugins that are
/// connected to each other must agree on the format used.
///
/// `name` specifies the name of the gate. The name is used to indicate which
/// custom operation is to be applied.
///
/// `targets` optionally specifies the set of target qubits. You may pass 0 or
/// an empty qubit set if you don't need target qubits.
///
/// `controls` optionally specifies the set of control qubits. You may pass 0
/// or an empty qubit set if you don't need control qubits.
///
/// `measures` optionally specifies the set of measured qubits. You may pass 0
/// or an empty qubit set if no qubits are measured. Note that the upstream
/// plugin expects exactly one measurement result for each qubit specified in
/// this set; anything else results in a warning and the measurement result
/// being set to undefined.
///
/// `matrix` can point to an appropriately sized array of doubles, or be `NULL`
/// if no matrix is required. If a matrix is specified, at least one target
/// qubit is required, and the matrix must be appropriately sized for the
/// number of target qubits. The matrix is specified in row-major form, using
/// pairs of doubles for the real vs. imaginary component of each entry. The
/// size must thus be `4**len(targets)` complex numbers = `2*4**len(targets)`
/// doubles = `16*4**len(targets)` bytes. `matrix_len` must be set to the
/// number of complex numbers.
///
/// In addition to the above data, gate objects implement the `arb` interface
/// to allow user-specified classical information to be attached.
///
/// This function returns the handle to the gate, or 0 to indicate failure.
/// The specified qubit sets are consumed/deleted by this function if and only
/// if it succeeds.
#[no_mangle]
pub extern "C" fn dqcs_gate_new_custom(
    name: *const c_char,
    targets: dqcs_handle_t,
    controls: dqcs_handle_t,
    measures: dqcs_handle_t,
    matrix: *const c_double,
    matrix_len: size_t,
) -> dqcs_handle_t {
    api_return(0, || {
        // Interpret name.
        let name = receive_str(name)?;

        // Interpret target set.
        resolve!(optional targets as pending QubitReferenceSet);
        let target_vec: Vec<QubitRef> = {
            if let Some(targets) = targets.as_ref() {
                let x: &QubitReferenceSet = targets.as_ref().unwrap();
                x.iter().cloned().collect()
            } else {
                vec![]
            }
        };

        // Interpret control set.
        resolve!(optional controls as pending QubitReferenceSet);
        let control_vec: Vec<QubitRef> = {
            if let Some(controls) = controls.as_ref() {
                let x: &QubitReferenceSet = controls.as_ref().unwrap();
                x.iter().cloned().collect()
            } else {
                vec![]
            }
        };

        // Interpret measurement set.
        resolve!(optional measures as pending QubitReferenceSet);
        let measure_vec: Vec<QubitRef> = {
            if let Some(measures) = measures.as_ref() {
                let x: &QubitReferenceSet = measures.as_ref().unwrap();
                x.iter().cloned().collect()
            } else {
                vec![]
            }
        };

        // Interpret matrix.
        let matrix = receive_matrix(matrix, matrix_len, target_vec.len())?;

        // Construct the gate.
        let gate = insert(Gate::new_custom(
            name,
            target_vec,
            control_vec,
            measure_vec,
            matrix,
            ArbData::default(),
        )?);

        // Everything went OK. Now make sure that the specified qubit set
        // handles are deleted.
        if let Some(mut targets) = targets {
            take!(resolved targets as QubitReferenceSet);
            let _ = targets;
        }
        if let Some(mut controls) = controls {
            take!(resolved controls as QubitReferenceSet);
            let _ = controls;
        }
        if let Some(mut measures) = measures {
            take!(resolved measures as QubitReferenceSet);
            let _ = measures;
        }
        Ok(gate)
    })
}

/// Returns whether the specified gate is a custom gate.
///
/// If this returns true, the type of gate is to be determined by matching its
/// name against a set of known gate types. If this returns false, the gate is
/// expected to be executed as follows, in this order:
///
///  - if there are target qubits, extend the supplied unitary matrix to
///    include the control qubits (if any), then apply it to the control +
///    target qubits;
///  - measure each measured qubit (if any) in the Z basis.
#[no_mangle]
pub extern "C" fn dqcs_gate_is_custom(gate: dqcs_handle_t) -> dqcs_bool_return_t {
    api_return_bool(|| {
        resolve!(gate as &Gate);
        Ok(gate.get_name().is_some())
    })
}

/// Returns the name of a custom gate.
///
/// This function fails if the gate is not a custom gate. Query
/// `dqcs_gate_is_custom()` to disambiguate between a non-custom gate and a
/// different error.
///
/// On success, this **returns a newly allocated string containing the gate
/// name. Free it with `free()` when you're done with it to avoid memory
/// leaks.** On failure, this returns `NULL`.
#[no_mangle]
pub extern "C" fn dqcs_gate_name(gate: dqcs_handle_t) -> *mut c_char {
    api_return_string(|| {
        resolve!(gate as &Gate);
        Ok(gate
            .get_name()
            .ok_or_else(oe_inv_arg(
                "gate is not custom and thus does not have a name",
            ))?
            .to_string())
    })
}

/// Returns whether the specified gate has target qubits.
#[no_mangle]
pub extern "C" fn dqcs_gate_has_targets(gate: dqcs_handle_t) -> dqcs_bool_return_t {
    api_return_bool(|| {
        resolve!(gate as &Gate);
        Ok(!gate.get_targets().is_empty())
    })
}

/// Returns a handle to a new qubit reference set containing the qubits
/// targetted by this gate.
///
/// If this gate has no targets or something else goes wrong, 0 is returned.
#[no_mangle]
pub extern "C" fn dqcs_gate_targets(gate: dqcs_handle_t) -> dqcs_handle_t {
    api_return(0, || {
        resolve!(gate as &Gate);
        let targets = gate.get_targets();
        if targets.is_empty() {
            inv_arg("gate does not have target qubits")
        } else {
            let targets: QubitReferenceSet = targets.iter().cloned().collect();
            Ok(insert(targets))
        }
    })
}

/// Returns whether the specified gate has control qubits.
#[no_mangle]
pub extern "C" fn dqcs_gate_has_controls(gate: dqcs_handle_t) -> dqcs_bool_return_t {
    api_return_bool(|| {
        resolve!(gate as &Gate);
        Ok(!gate.get_controls().is_empty())
    })
}

/// Returns a handle to a new qubit reference set containing the qubits
/// that control this gate.
///
/// If this gate has no control qubits or something else goes wrong, 0 is
/// returned.
#[no_mangle]
pub extern "C" fn dqcs_gate_controls(gate: dqcs_handle_t) -> dqcs_handle_t {
    api_return(0, || {
        resolve!(gate as &Gate);
        let controls = gate.get_controls();
        if controls.is_empty() {
            inv_arg("gate does not have control qubits")
        } else {
            let controls: QubitReferenceSet = controls.iter().cloned().collect();
            Ok(insert(controls))
        }
    })
}

/// Returns whether the specified gate measures any qubits.
#[no_mangle]
pub extern "C" fn dqcs_gate_has_measures(gate: dqcs_handle_t) -> dqcs_bool_return_t {
    api_return_bool(|| {
        resolve!(gate as &Gate);
        Ok(!gate.get_measures().is_empty())
    })
}

/// Returns a handle to a new qubit reference set containing the qubits
/// measured by this gate.
///
/// If this gate does not measure any qubits or something else goes wrong, 0 is
/// returned.
#[no_mangle]
pub extern "C" fn dqcs_gate_measures(gate: dqcs_handle_t) -> dqcs_handle_t {
    api_return(0, || {
        resolve!(gate as &Gate);
        let measures = gate.get_measures();
        if measures.is_empty() {
            inv_arg("gate does not measure any qubits")
        } else {
            let measures: QubitReferenceSet = measures.iter().cloned().collect();
            Ok(insert(measures))
        }
    })
}

/// Returns whether a unitary matrix is associated with this gate.
#[no_mangle]
pub extern "C" fn dqcs_gate_has_matrix(gate: dqcs_handle_t) -> dqcs_bool_return_t {
    api_return_bool(|| {
        resolve!(gate as &Gate);
        Ok(gate.get_matrix().is_some())
    })
}

/// Returns a copy of the unitary matrix associated with this gate, if one
/// exists.
///
/// If this function succeeds, the matrix is returned in row-major form, using
/// pairs of doubles for the real vs. imaginary component of each entry. The
/// size will be `4**len(targets)` complex numbers = `2*4**len(targets)`
/// doubles = `16*4**len(targets)` bytes.
///
/// On success, this **returns a newly allocated array containing the matrix.
/// Free it with `free()` when you're done with it to avoid memory leaks.** On
/// failure, or if no matrix is associated with this gate, this returns `NULL`.
/// Use `dqcs_gate_has_matrix()` to disambiguate.
#[no_mangle]
pub extern "C" fn dqcs_gate_matrix(gate: dqcs_handle_t) -> *mut c_double {
    api_return(null_mut(), || {
        resolve!(gate as &Gate);
        let matrix = gate.get_matrix();
        if let Some(matrix) = matrix {
            let ffi_matrix =
                unsafe { calloc(2 * matrix.len(), size_of::<c_double>()) as *mut c_double };
            if ffi_matrix.is_null() {
                err("failed to allocate return value")
            } else {
                for (i, x) in matrix.into_iter().enumerate() {
                    unsafe {
                        *ffi_matrix.add(i * 2) = x.re;
                        *ffi_matrix.add(i * 2 + 1) = x.im;
                    }
                }
                Ok(ffi_matrix)
            }
        } else {
            inv_arg("no matrix associated with gate")
        }
    })
}

/// Returns the size of the gate matrix associated with this gate.
///
/// The size is returned in the form of the number of complex entries. That is,
/// the number of doubles is two times the return value, and the size in bytes
/// is 8 times the return value. 0 is returned when there is no matrix or when
/// an error occurs.
#[no_mangle]
pub extern "C" fn dqcs_gate_matrix_len(gate: dqcs_handle_t) -> size_t {
    api_return(0, || {
        resolve!(gate as &Gate);
        let matrix = gate.get_matrix();
        if let Some(matrix) = matrix {
            Ok(matrix.len())
        } else {
            inv_arg("no matrix associated with gate")
        }
    })
}
