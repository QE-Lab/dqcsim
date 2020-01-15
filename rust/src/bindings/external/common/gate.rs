use super::*;
use crate::common::gates::GateType;
use std::convert::TryFrom;

/// Helper function for the `dqcs_gate_new_predef*()` functions.
fn new_predef_helper(
    gate_type: dqcs_internal_gate_t,
    qubits: Vec<QubitRef>,
    param_data: dqcs_handle_t,
) -> Result<dqcs_handle_t> {
    // Interpret gate type.
    let gate_type = GateType::try_from(gate_type)?;

    // Interpret data.
    resolve!(optional param_data as pending ArbData);
    let data: ArbData = {
        if let Some(data) = param_data.as_ref() {
            let x: &ArbData = data.as_ref().unwrap();
            x.clone()
        } else {
            ArbData::default()
        }
    };

    // Construct the gate.
    let gate = insert(
        gate_type
            .into_converter(None, 0., false)
            .construct(&(qubits, data))?,
    );

    // Delete consumed handles.
    if let Some(mut param_data) = param_data {
        delete!(resolved param_data);
    }

    Ok(gate)
}

/// Constructs a new predefined unitary gate.
///
/// `gate_type` specifies which kind of gate should be constructed.
///
/// `targets` must be a handle to a non-empty qubit set, containing at least
/// as many qubits as needed for the specified gate type. If more qubits are
/// specified, the rightmost qubits become the targets, and the remaining
/// qubits become control qubits to make a controlled gate.
///
/// `param_data` takes an optional `ArbData` object used to parameterize the
/// gate if necessary. If not specified, an empty object is used. Some of the
/// gate types are parameterized, and use values from this `ArbData` as
/// follows; anything remaining in the `ArbData` afterwards is placed in the
/// gate object.
///
///  - `DQCS_GATE_RX`, `DQCS_GATE_RY`, `DQCS_GATE_RZ`, and `DQCS_GATE_PHASE`
///    pop a 64-bit double floating point with the angle at binary string index
///    0.
///  - `DQCS_GATE_PHASE_K` pops a 64-bit unsigned integer with the k value at
///    binary string index 0.
///  - `DQCS_GATE_R` pops theta at binary string index 0, phi at index 1, and
///    lambda at index 2. They represent 64-bit double floating points.
///  - `DQCS_GATE_U` pops the entire matrix as a single argument at index 0,
///    consisting of 2**N * 2**N * 2 doubles, in real-first row-major format
///    (same as the other matrix definitions in DQCsim).
///
/// This function returns the handle to the gate, or 0 to indicate failure.
/// The qubit set and parameterization data (if specified) are consumed/deleted
/// by this function if and only if it succeeds.
#[no_mangle]
pub extern "C" fn dqcs_gate_new_predef(
    gate_type: dqcs_internal_gate_t,
    qubits: dqcs_handle_t,
    param_data: dqcs_handle_t,
) -> dqcs_handle_t {
    api_return(0, || {
        // Interpret qubits.
        resolve!(qubits as pending QubitReferenceSet);
        let qubit_vec: Vec<QubitRef> = {
            let x: &QubitReferenceSet = qubits.as_ref().unwrap();
            x.iter().cloned().collect()
        };

        // Call the helper for the rest of the logic.
        let gate = new_predef_helper(gate_type, qubit_vec, param_data)?;

        // Delete consumed handles.
        delete!(resolved qubits);
        Ok(gate)
    })
}

/// Constructs a new predefined unitary one-qubit gate.
///
/// This function is simply a shorthand for `dqcs_gate_new_predef()` with
/// one qubit in the `qubits` set, to make constructing one-qubit gates more
/// ergonomic. Refer to its documentation for more information.
#[no_mangle]
pub extern "C" fn dqcs_gate_new_predef_one(
    gate_type: dqcs_internal_gate_t,
    qa: dqcs_qubit_t,
    param_data: dqcs_handle_t,
) -> dqcs_handle_t {
    api_return(0, || {
        let qubits_vec = vec![QubitRef::from_foreign(qa)
            .ok_or_else(oe_inv_arg("0 is not a valid qubit reference"))?];
        new_predef_helper(gate_type, qubits_vec, param_data)
    })
}

/// Constructs a new predefined unitary two-qubit gate.
///
/// This function is simply a shorthand for `dqcs_gate_new_predef()` with
/// two qubit in the `qubits` set, to make constructing two-qubit gates more
/// ergonomic. Refer to its documentation for more information.
#[no_mangle]
pub extern "C" fn dqcs_gate_new_predef_two(
    gate_type: dqcs_internal_gate_t,
    qa: dqcs_qubit_t,
    qb: dqcs_qubit_t,
    param_data: dqcs_handle_t,
) -> dqcs_handle_t {
    api_return(0, || {
        let qubits_vec = vec![
            QubitRef::from_foreign(qa)
                .ok_or_else(oe_inv_arg("0 is not a valid qubit reference"))?,
            QubitRef::from_foreign(qb)
                .ok_or_else(oe_inv_arg("0 is not a valid qubit reference"))?,
        ];
        if qa == qb {
            inv_arg(format!("cannot use qubit {} twice", qa))?;
        }
        new_predef_helper(gate_type, qubits_vec, param_data)
    })
}

/// Constructs a new predefined unitary three-qubit gate.
///
/// This function is simply a shorthand for `dqcs_gate_new_predef()` with
/// three qubit in the `qubits` set, to make constructing three-qubit gates
/// more ergonomic. Refer to its documentation for more information.
#[no_mangle]
pub extern "C" fn dqcs_gate_new_predef_three(
    gate_type: dqcs_internal_gate_t,
    qa: dqcs_qubit_t,
    qb: dqcs_qubit_t,
    qc: dqcs_qubit_t,
    param_data: dqcs_handle_t,
) -> dqcs_handle_t {
    api_return(0, || {
        let qubits_vec = vec![
            QubitRef::from_foreign(qa)
                .ok_or_else(oe_inv_arg("0 is not a valid qubit reference"))?,
            QubitRef::from_foreign(qb)
                .ok_or_else(oe_inv_arg("0 is not a valid qubit reference"))?,
            QubitRef::from_foreign(qc)
                .ok_or_else(oe_inv_arg("0 is not a valid qubit reference"))?,
        ];
        if qa == qb || qa == qc {
            inv_arg(format!("cannot use qubit {} twice", qa))?;
        }
        if qb == qc {
            inv_arg(format!("cannot use qubit {} twice", qb))?;
        }
        new_predef_helper(gate_type, qubits_vec, param_data)
    })
}

/// Constructs a new unitary gate.
///
/// `targets` must be a handle to a non-empty qubit set. The qubits in this set
/// correspond with the supplied unitary matrix.
///
/// `controls` optionally specifies a set of control qubits. You may pass 0 or
/// an empty qubit set if you don't need control qubits.
///
/// `matrix` must be a handle to an appropriately sized matrix.
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
/// The `targets` qubit set, (if specified) the `controls` qubit set, and the
/// matrix are consumed/deleted by this function if and only if it succeeds.
#[no_mangle]
pub extern "C" fn dqcs_gate_new_unitary(
    targets: dqcs_handle_t,
    controls: dqcs_handle_t,
    matrix: dqcs_handle_t,
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
        resolve!(matrix as pending Matrix);
        let matrix_ref: &Matrix = matrix.as_ref().unwrap();

        // Construct the gate.
        let gate = insert(Gate::new_unitary(
            target_vec,
            control_vec,
            matrix_ref.clone(),
        )?);

        // Everything went OK. Now make sure that the target and control set
        // handles are deleted.
        delete!(resolved targets);
        if let Some(mut controls) = controls {
            delete!(resolved controls);
        }
        delete!(resolved matrix);
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
        delete!(resolved measures);
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
/// `matrix` optionally specifies a handle to an appropriately sized matrix
/// for the `targets` qubit set.
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
    matrix: dqcs_handle_t,
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
        resolve!(optional matrix as pending Matrix);
        let matrix_clone: Option<Matrix> = {
            if let Some(matrix) = matrix.as_ref() {
                let x: &Matrix = matrix.as_ref().unwrap();
                Some(x.clone())
            } else {
                None
            }
        };

        // Construct the gate.
        let gate = insert(Gate::new_custom(
            name,
            target_vec,
            control_vec,
            measure_vec,
            matrix_clone,
            ArbData::default(),
        )?);

        // Everything went OK. Now make sure that the specified qubit set
        // handles are deleted.
        if let Some(mut targets) = targets {
            delete!(resolved targets);
        }
        if let Some(mut controls) = controls {
            delete!(resolved controls);
        }
        if let Some(mut measures) = measures {
            delete!(resolved measures);
        }
        if let Some(mut matrix) = matrix {
            delete!(resolved matrix);
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
#[no_mangle]
pub extern "C" fn dqcs_gate_targets(gate: dqcs_handle_t) -> dqcs_handle_t {
    api_return(0, || {
        resolve!(gate as &Gate);
        let targets = gate.get_targets();
        let targets: QubitReferenceSet = targets.iter().cloned().collect();
        Ok(insert(targets))
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
#[no_mangle]
pub extern "C" fn dqcs_gate_controls(gate: dqcs_handle_t) -> dqcs_handle_t {
    api_return(0, || {
        resolve!(gate as &Gate);
        let controls = gate.get_controls();
        let controls: QubitReferenceSet = controls.iter().cloned().collect();
        Ok(insert(controls))
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
#[no_mangle]
pub extern "C" fn dqcs_gate_measures(gate: dqcs_handle_t) -> dqcs_handle_t {
    api_return(0, || {
        resolve!(gate as &Gate);
        let measures = gate.get_measures();
        let measures: QubitReferenceSet = measures.iter().cloned().collect();
        Ok(insert(measures))
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
/// If this function succeeds, a new matrix handle is returned. If it fails,
/// 0 is returned.
#[no_mangle]
pub extern "C" fn dqcs_gate_matrix(gate: dqcs_handle_t) -> dqcs_handle_t {
    api_return(0, || {
        resolve!(gate as &Gate);
        Ok(insert(gate.get_matrix().ok_or_else(oe_inv_arg(
            "no matrix associated with gate",
        ))?))
    })
}

/// Utility function that detects control qubits in the `targets` list of the
/// gate by means of the gate matrix, and reduces them into `controls` qubits.
///>
///> This function borrows a handle to any gate with a matrix, and returns an
///> equivalent copy of said gate with any control qubits in the `targets` set
///> moved to the `controls` set. The associated gate matrix is accordingly
///> reduced in size. The control qubits are added at the end of the `controls`
///> set in the same order they appeared in the `targets` qubit set.
///>
///> `epsilon` specifies the maximum element-wise deviation from the identity
///> matrix for the relevant array elements for a qubit to be considered a
///> control qubit. Note that if this is greater than zero, the resulting gate
///> may not be exactly equivalent. If `ignore_gphase` is set, any global phase
///> in the matrix is ignored, but the global phase of the non-control submatrix
///> is not changed.
///>
///> This function returns a new gate handle with the modified gate, or a copy
///> of the input gate if the matrix could not be reduced. If the input gate
///> does not have a matrix (measurement gate, or custom gate without matrix) an
///> error is returned instead.
#[no_mangle]
pub extern "C" fn dqcs_gate_reduce_control(
    gate: dqcs_handle_t,
    epsilon: c_double,
    ignore_gphase: bool,
) -> dqcs_handle_t {
    api_return(0, || {
        resolve!(gate as &Gate);
        if gate.get_matrix().is_none() {
            inv_arg("no matrix associated with gate")
        } else {
            Ok(insert(gate.with_gate_controls(epsilon, ignore_gphase)))
        }
    })
}

/// Utility function that expands a gate matrix to account for all control
/// qubits.
///>
///> This function borrows a handle to any gate with a matrix, and returns an
///> equivalent copy of said gate with any control qubits in the `controls` set
///> moved to the `targets` set. The associated gate matrix is extended
///> accordingly. The control qubits are added at the front of the `targets`
///> set in the same order they appeared in the `controls` qubit set.
///>
///> This function returns a new gate handle with the modified gate, or a copy
///> of the input gate if the matrix could not be reduced. If the input gate
///> does not have a matrix (measurement gate, or custom gate without matrix) an
///> error is returned instead.
#[no_mangle]
pub extern "C" fn dqcs_gate_expand_control(gate: dqcs_handle_t) -> dqcs_handle_t {
    api_return(0, || {
        resolve!(gate as &Gate);
        if gate.get_matrix().is_none() {
            inv_arg("no matrix associated with gate")
        } else {
            Ok(insert(gate.with_matrix_controls()))
        }
    })
}
