use super::*;
use crate::common::gates::GateType;
use std::convert::TryFrom;

/// Converts a qubit count match number to an Option for slightly more
/// idiomatic rust code outside of this file.
fn expected_qubit_count(num_qubits: isize) -> Option<usize> {
    if num_qubits < 0 {
        None
    } else {
        Some(num_qubits as usize)
    }
}

/// Constructs a new gate map.
///>
///> Returns a handle to a gate map with no mappings attached to it yet. Use
///> `dqcs_gm_add_*()` to do that. The mappings are queried in the order in
///> which they are added, so be sure to add more specific gates first. Once
///> added, use `dqcs_gm_detect()` to detect incoming DQCsim gates, and
///> `dqcs_gm_construct*()` to (re)construct gates for transmission.
///>
///> Gate maps objects retain a cache to speed up detection of similar DQCsim
///> gates: if a gate is received for the second time, the cache will hit,
///> avoiding recomputation of the detector functions. What constitutes
///> "similar gates" is defined by the two booleans passed to this function. If
///> `strip_qubit_refs` is set, all qubit references associated with the gate
///> will be invalidated (i.e., set to 0), such that for instance an X gate
///> applied to qubit 1 will be considered equal to an X gate applied to qubit
///> 2. If `strip_data` is set, the `ArbData` associated with the incoming
///> gate is removed. Note that this also means that the `ArbData` returned by
///> `dqcs_gm_map()` will be based on an empty `ArbData` object.
///>
///> Gates are identified through user-defined `void*` keys. To do the above,
///> however, DQCsim needs to know the following things:
///>
///>  - how to delete an owned copy of a key if your semantics are that DQCsim
///>    owns it,
///>  - how to compare two keys (equality);
///>  - how to hash a key.
///>
///> The deletion function is passed when the key is passed. If the keys are
///> objects of different classes, this allows different constructors to be
///> passed here. There can only be a single comparison and hash function for
///> each gate map, though. They are passed here.
///>
///> `key_cmp` represents this comparison function. It takes two `void*` to
///> keys and must returns whether they are equal or not. If not specified,
///> the default is to compare the pointers themselves, instead of the values
///> they refer to. `key_cmp` must be a pure function, i.e., depend only on its
///> input values.
///>
///> `key_hash` represents the hashing function. It takes a `void*` key and
///> returns a 64-bit hash representative of the key. **For any pair of keys
///> for which `key_cmp` returns true, the hashes must be equal.** The default
///> behavior depends on whether `key_cmp` is defined: if it is, all keys will
///> have the same hash; if it isn't, the pointer is itself hashed. `key_hash`
///> must be a pure function, i.e., depend only on its input values.
///>
///> It is recommended to first preprocess incoming gates with
///> `dqcs_gate_reduce_control()`. In this case, controlled unitary gate
///> matrices will be reduced to their non-controlled submatrix, such that the
///> unitary gate detectors will operate on said submatrix. The predefined
///> unitary gate detectors are more-or-less based on this assumption (as there
///> are no predefined controlled matrices).
///>
///> Alternatively, you can preprocess with `dqcs_gate_expand_control()`. In
///> this case, you can use `dqcs_gm_add_fixed_unitary()` to detect the full
///> matrix in all cases, by specifying the CNOT matrix instead of an X matrix
///> with one control qubit.
///>
///> If you don't preprocess, the upstream plugin determines the
///> representation. That is, it may send a CNOT as a two-qubit gate with a
///> CNOT matrix or as a controlled X gate with a single target and single
///> control qubit. The gate map will then detect these as two different kinds
///> of gates.
#[no_mangle]
pub extern "C" fn dqcs_gm_new(
    strip_qubit_refs: bool,
    strip_data: bool,
    key_cmp: Option<extern "C" fn(*const c_void, *const c_void) -> bool>,
    key_hash: Option<extern "C" fn(*const c_void) -> u64>,
) -> dqcs_handle_t {
    let map = if strip_qubit_refs {
        if strip_data {
            ConverterMap::new(Some(Box::new(|gate: &Gate| gate.without_qubit_refs())))
        } else {
            ConverterMap::new(Some(Box::new(|gate: &Gate| {
                let mut gate = gate.without_qubit_refs();
                gate.data.clear();
                gate
            })))
        }
    } else if strip_data {
        ConverterMap::new(Some(Box::new(|gate: &Gate| {
            let mut gate = gate.clone();
            gate.data.clear();
            gate
        })))
    } else {
        ConverterMap::new(None)
    };
    insert(GateMap {
        map,
        key_cmp,
        key_hash,
    })
}

/// Adds a unitary gate mapping for the given DQCsim-defined gate to the
/// given gate map.
///>
///> `gm` must be a handle to a gate map object (`dqcs_gm_new()`).
///> `key_free` is an optional callback function used to free `key_data` when
///> the gate map is destroyed, or when this function fails.
///> `key_data` is the user-specified value used to identify this mapping.
///> `gate` defines which predefined gate to use. Some of the predefined gates
///> are parameterized.
///> `num_controls` specifies the number of control qubits associated with this
///> gate type. If negative, the gate can have any number of control qubits.
///> If zero or positive, the number of control qubits must be as specified.
///> `epsilon` specifies the maximum element-wise root-mean-square error
///> between the incoming matrix and the to be detected matrix that results in a
///> positive match.
///> `ignore_phase` specifies whether the aforementioned check should ignore
///> global phase or not when there are no explicit control qubits.
///>
///> For most gate types, the parameterization `ArbData` object returned by
///> detection and consumed by construction is mapped one-to-one to the user
///> data of the gate in the DQCsim-protocol. Some of the detectors however
///> detect parameterized gate matrices. These detectors prefix a fixed number
///> of binary string arguments to the `ArbData` upon detection, and pop these
///> when constructing. The specs for this are as follows:
///>
///>  - `DQCS_GATE_RX`, `DQCS_GATE_RY`, `DQCS_GATE_RZ`, and `DQCS_GATE_PHASE`
///>    insert/pop a 64-bit double floating point with the angle at binary
///>    string index 0.
///>  - `DQCS_GATE_PHASE_K` inserts/pops a 64-bit unsigned integer with the k
///>    value at binary string index 0.
///>  - `DQCS_GATE_R` inserts/pops theta at binary string index 0, phi at index
///>    1, and lambda at index 2. They represent 64-bit double floating points.
///>  - `DQCS_GATE_U*` inserts/pops the entire matrix as a single argument at
///>    index 0, consisting of 2**N * 2**N * 2 doubles, in real-first row-major
///>    format (same as the other matrix definitions in DQCsim).
#[no_mangle]
pub extern "C" fn dqcs_gm_add_predef_unitary(
    gm: dqcs_handle_t,
    key_free: Option<extern "C" fn(user_data: *mut c_void)>,
    key_data: *mut c_void,
    gate: dqcs_predefined_gate_t,
    num_controls: isize,
    epsilon: c_double,
    ignore_gphase: bool,
) -> dqcs_return_t {
    api_return_none(|| {
        let key = UserKeyData::new(key_free, key_data);
        resolve!(gm as &mut GateMap);
        let key = gm.make_key(key);
        let gate = GateType::try_from(gate)?;
        let num_controls = expected_qubit_count(num_controls);
        gm.map.push(
            key,
            gate.into_gate_converter(num_controls, epsilon, ignore_gphase),
        );
        Ok(())
    })
}

/// Adds a unitary gate mapping for the given gate matrix to the given gate
/// map.
///>
///> `gm` must be a handle to a gate map object (`dqcs_gm_new()`).
///> `key_free` is an optional callback function used to free `key_data` when
///> the gate map is destroyed, or when this function fails.
///> `key_data` is the user-specified value used to identify this mapping.
///> `matrix` must be passed a handle to the matrix to detect. It is consumed
///> by this function.
///> `num_controls` specifies the number of control qubits associated with this
///> gate type. If negative, the gate can have any number of control qubits.
///> If zero or positive, the number of control qubits must be as specified.
///> `epsilon` specifies the maximum element-wise root-mean-square error
///> between the incoming matrix and the to be detected matrix that results in a
///> positive match.
///> `ignore_phase` specifies whether the aforementioned check should ignore
///> global phase or not when there are no explicit control qubits.
///>
///> The parameterization `ArbData` object returned by detection and consumed
///> by construction is mapped one-to-one to the user data of the gate in the
///> DQCsim-protocol.
#[no_mangle]
pub extern "C" fn dqcs_gm_add_fixed_unitary(
    gm: dqcs_handle_t,
    key_free: Option<extern "C" fn(key_data: *mut c_void)>,
    key_data: *mut c_void,
    matrix: dqcs_handle_t,
    num_controls: isize,
    epsilon: c_double,
    ignore_gphase: bool,
) -> dqcs_return_t {
    api_return_none(|| {
        let key = UserKeyData::new(key_free, key_data);
        resolve!(gm as &mut GateMap);
        let key = gm.make_key(key);
        take!(matrix as Matrix);
        let num_controls = expected_qubit_count(num_controls);
        gm.map.push(
            key,
            Box::new(UnitaryGateConverter::from(UnitaryConverter::new(
                FixedMatrixConverter::from(matrix),
                num_controls,
                epsilon,
                ignore_gphase,
            ))),
        );
        Ok(())
    })
}

/// Adds a custom unitary gate mapping to the given gate map.
///>
///> `gm` must be a handle to a gate map object (`dqcs_gm_new()`).
///> `key_free` is an optional callback function used to free `key_data` when
///> the gate map is destroyed, or when this function fails.
///> `key_data` is the user-specified value used to identify this mapping.
///> `detector` is the detector function pointer. It is optional; if null, this
///> mapping only supports construction.
///> `detector_user_free` is an optional callback function used to free
///> `detector_user_data` when the gate map is destroyed, when this function
///> fails, or when `detector` was null.
///> `detector_user_data` is a user-specified value that is passed to the
///> `detector` callback function. It is not used by DQCsim.
///> `constructor` is the constructor function pointer. It is optional; if
///> null, this mapping only supports detection.
///> `constructor_user_free` is an optional callback function used to free
///> `constructor_user_data` when the gate map is destroyed, when this function
///> fails, or when `constructor` was null.
///> `constructor_user_data` is a user-specified value that is passed to the
///> `constructor` callback function. It is not used by DQCsim.
///>
///> If both `constructor` and `detector` are null for some reason, the
///> function is no-op (besides possibly calling the `*_free()` callbacks.
///>
///> The detector callback receives a matrix and control qubit information for
///> the user to match. The matrix is passed through the borrowed `matrix`
///> handle. `num_controls` is passed the number of explicit control qubits
///> that exist besides the matrix (that is, if nonzero, the matrix is actually
///> only the non-controlled submatrix of the controlled gate). `param_data` is
///> given a borrowed `ArbData` handle initialized with the `ArbData` attached
///> to the gate. If the gate matches, the detector function must return
///> `DQCS_TRUE`. In this case, it can mutate the `param_data` to add the
///> detected gate parameters. If it doesn't match, it must return
///> `DQCS_FALSE`. If an error occurs, it must call `dqcs_error_set()` with the
///> error message and return `DQCS_BOOL_FAILURE`.
///>
///> The constructor callback performs the reverse operation. It receives an
///> `ArbData` handle containing the parameterization data, and must construct
///> the matrix, return the bound on the number of control qubits, and must
///> return the `ArbData` associated with the gate by mutating the `param_data`
///> handle. `num_controls` will point to a variable initialized to -1
///> representing a constraint on the number of control qubits. This works as
///> follows: if negative, any number of qubits is allowed; if zero or
///> positive, only that number is allowed. If construction succeeds, the
///> constructor function must return a handle to the constructed matrix. If
///> it fails, it must call `dqcs_error_set()` with an error message and return
///> 0.
///>
///> It is up to the user how to do the matching and constructing, but the
///> converter functions must always return the same value for the same input.
///> In other words, they must be pure functions. Otherwise, the caching
///> behavior of the `GateMap` will make the results inconsistent.
#[no_mangle]
pub extern "C" fn dqcs_gm_add_custom_unitary(
    gm: dqcs_handle_t,
    key_free: Option<extern "C" fn(key_data: *mut c_void)>,
    key_data: *mut c_void,
    detector: Option<
        extern "C" fn(
            user_data: *const c_void,
            matrix: dqcs_handle_t,
            num_controls: size_t,
            param_data: dqcs_handle_t,
        ) -> dqcs_bool_return_t,
    >,
    detector_user_free: Option<extern "C" fn(user_data: *mut c_void)>,
    detector_user_data: *mut c_void,
    constructor: Option<
        extern "C" fn(
            user_data: *const c_void,
            param_data: dqcs_handle_t,
            num_controls: *mut isize,
        ) -> dqcs_handle_t,
    >,
    constructor_user_free: Option<extern "C" fn(user_data: *mut c_void)>,
    constructor_user_data: *mut c_void,
) -> dqcs_return_t {
    api_return_none(|| {
        let key = UserKeyData::new(key_free, key_data);
        let detector_user_data = UserData::new(detector_user_free, detector_user_data);
        let constructor_user_data = UserData::new(constructor_user_free, constructor_user_data);
        resolve!(gm as &mut GateMap);
        let key = gm.make_key(key);
        gm.map.push(
            key,
            Box::new(CustomGateConverter::new(
                move |gate| {
                    if let Some(detector) = detector {
                        if gate.get_name().is_some() || !gate.get_measures().is_empty() {
                            // Custom gate, measurement gate or unitary + measure compound; not
                            // (just) a unitary so no match.
                            Ok(None)
                        } else if let Some(matrix) = gate.get_matrix() {
                            // Unitary gate. Check conditions.
                            let matrix = insert(matrix);
                            let num_controls = gate.get_controls().len();
                            let param_data = insert(gate.data.clone());
                            let result = detector(
                                detector_user_data.data(),
                                matrix,
                                num_controls,
                                param_data,
                            );
                            delete!(matrix);
                            take!(param_data as ArbData);
                            if cb_return_bool(result)? {
                                // Matrix match; construct qubit argument vector.
                                let mut qubits = vec![];
                                qubits.extend(gate.get_controls().iter());
                                qubits.extend(gate.get_targets().iter());
                                Ok(Some((qubits, param_data)))
                            } else {
                                // Matrix didn't match.
                                Ok(None)
                            }
                        } else {
                            // A gate with no name, matrix, or measured qubits is illegal.
                            unreachable!();
                        }
                    } else {
                        // No detector defined.
                        Ok(None)
                    }
                },
                move |qubits, param_data| {
                    if let Some(constructor) = constructor {
                        // Construct the matrix and data.
                        let param_data = insert(param_data.clone());
                        let mut expected_num_controls = -1isize;
                        let result = constructor(
                            constructor_user_data.data(),
                            param_data,
                            &mut expected_num_controls as *mut isize,
                        );
                        take!(param_data as ArbData);
                        let matrix = cb_return(0, result)?;
                        take!(matrix as Matrix);

                        // Parse qubit argument vector.
                        let expected_num_controls = expected_qubit_count(expected_num_controls);
                        let num_targets = matrix.num_qubits().unwrap();
                        let num_controls =
                            qubits
                                .len()
                                .checked_sub(num_targets)
                                .ok_or_else(oe_inv_arg(format!(
                                    "need at least {} qubits",
                                    num_targets
                                )))?;
                        if let Some(expected) = expected_num_controls {
                            if num_controls != expected {
                                inv_arg(format!(
                                    "expected {} control and {} target qubits",
                                    expected, num_targets
                                ))?;
                            }
                        }
                        let controls = &qubits[..num_controls];
                        let targets = &qubits[num_controls..];

                        // Construct the gate.
                        let mut gate = Gate::new_unitary(
                            targets.iter().cloned(),
                            controls.iter().cloned(),
                            matrix,
                        )?;
                        gate.data.copy_from(&param_data);

                        Ok(gate)
                    } else {
                        // No constructor defined.
                        inv_arg("no constructor function defined")
                    }
                },
            )),
        );
        Ok(())
    })
}

/// Adds a measurement gate mapping to the given gate map.
///>
///> `gm` must be a handle to a gate map object (`dqcs_gm_new()`).
///> `key_free` is an optional callback function used to free `key_data` when
///> the gate map is destroyed, or when this function fails.
///> `key_data` is the user-specified value used to identify this mapping.
///> `num_measures` specifies the number of measured qubits for this gate type.
///> If negative, the gate can have any number of measured qubits. If zero or
///> positive, the number of measured qubits must be as specified.
///>
///> The parameterization `ArbData` object returned by detection and consumed
///> by construction is mapped one-to-one to the user data of the gate in the
///> DQCsim-protocol.
#[no_mangle]
pub extern "C" fn dqcs_gm_add_measure(
    gm: dqcs_handle_t,
    key_free: Option<extern "C" fn(user_data: *mut c_void)>,
    key_data: *mut c_void,
    num_measures: isize,
) -> dqcs_return_t {
    api_return_none(|| {
        let key = UserKeyData::new(key_free, key_data);
        resolve!(gm as &mut GateMap);
        let key = gm.make_key(key);
        let num_measures = expected_qubit_count(num_measures);
        gm.map
            .push(key, Box::new(MeasurementGateConverter::new(num_measures)));
        Ok(())
    })
}

/// Adds a fully customizable gate mapping to the given gate map.
///>
///> Note that this is the only type of mapping that can handle custom/named
///> gates.
///>
///> `detector` is the detector function pointer. It is optional; if null, this
///> mapping only supports construction.
///> `detector_user_free` is an optional callback function used to free
///> `detector_user_data` when the gate map is destroyed, when this function
///> fails, or when `detector` was null.
///> `detector_user_data` is a user-specified value that is passed to the
///> `detector` callback function. It is not used by DQCsim.
///> `constructor` is the constructor function pointer. It is optional; if
///> null, this mapping only supports detection.
///> `constructor_user_free` is an optional callback function used to free
///> `constructor_user_data` when the gate map is destroyed, when this function
///> fails, or when `constructor` was null.
///> `constructor_user_data` is a user-specified value that is passed to the
///> `constructor` callback function. It is not used by DQCsim.
///>
///> If both `constructor` and `detector` are null for some reason, the
///> function is no-op (besides possibly calling the `*_free()` callbacks.
///>
///> The detector callback receives the complete gate passed to the gate map
///> for it to match as it pleases. If the gate matches, the detector function
///> must return `DQCS_TRUE`. It may  assign `qubits` to a `qbset` object
///> representing the qubit arguments (substituted with an empty set if it
///> doesn't), and may assign `param_data` to an `arb` handle with the
///> parameterization data (if it doesn't, the data from the gate is used; if
///> this was modified by the callback, the modified data is used). If the gate
///> doesn't match, it must return `DQCS_FALSE`. If an error occurs, it must
///> call `dqcs_error_set()` with the error message and return
///> `DQCS_BOOL_FAILURE`.
///>
///> The constructor callback performs the reverse operation. It receives an
///> `ArbData` handle containing the parameterization data and a qubit set, and
///> must construct a gate based on this information. If construction succeeds,
///> the constructor function must return the gate handle. If an error occurs,
///> it must call `dqcs_error_set()` with the error message and return 0.
///>
///> It is up to the user how to do the matching and constructing, but the
///> converter functions must always return the same value for the same input.
///> In other words, they must be pure functions. Otherwise, the caching
///> behavior of the `GateMap` will make the results inconsistent.
#[no_mangle]
pub extern "C" fn dqcs_gm_add_custom(
    gm: dqcs_handle_t,
    key_free: Option<extern "C" fn(key_data: *mut c_void)>,
    key_data: *mut c_void,
    detector: Option<
        extern "C" fn(
            user_data: *const c_void,
            gate: dqcs_handle_t,
            qubits: *mut dqcs_handle_t,
            param_data: *mut dqcs_handle_t,
        ) -> dqcs_bool_return_t,
    >,
    detector_user_free: Option<extern "C" fn(user_data: *mut c_void)>,
    detector_user_data: *mut c_void,
    constructor: Option<
        extern "C" fn(
            user_data: *const c_void,
            qubits: dqcs_handle_t,
            param_data: dqcs_handle_t,
        ) -> dqcs_handle_t,
    >,
    constructor_user_free: Option<extern "C" fn(user_data: *mut c_void)>,
    constructor_user_data: *mut c_void,
) -> dqcs_return_t {
    api_return_none(|| {
        let key = UserKeyData::new(key_free, key_data);
        let detector_user_data = UserData::new(detector_user_free, detector_user_data);
        let constructor_user_data = UserData::new(constructor_user_free, constructor_user_data);
        resolve!(gm as &mut GateMap);
        let key = gm.make_key(key);
        gm.map.push(
            key,
            Box::new(CustomGateConverter::new(
                move |gate| {
                    if let Some(detector) = detector {
                        let gate = insert(gate.clone());
                        let mut qubits: dqcs_handle_t = 0;
                        let mut param_data: dqcs_handle_t = 0;
                        let result = detector(
                            detector_user_data.data(),
                            gate,
                            &mut qubits as *mut dqcs_handle_t,
                            &mut param_data as *mut dqcs_handle_t,
                        );
                        let param_data = if param_data == 0 {
                            take!(gate as Gate);
                            gate.data
                        } else {
                            delete!(gate);
                            take!(param_data as ArbData);
                            param_data
                        };
                        let qubits = if qubits == 0 {
                            vec![]
                        } else {
                            take!(qubits as QubitReferenceSet);
                            qubits.into_iter().collect()
                        };
                        if cb_return_bool(result)? {
                            // Detector returned match.
                            Ok(Some((qubits, param_data)))
                        } else {
                            // Detector returned no match.
                            Ok(None)
                        }
                    } else {
                        // No detector defined.
                        Ok(None)
                    }
                },
                move |qubits, param_data| {
                    if let Some(constructor) = constructor {
                        let qubits: QubitReferenceSet = qubits.iter().cloned().collect();
                        let qubits = insert(qubits);
                        let param_data = insert(param_data.clone());
                        let result = constructor(constructor_user_data.data(), qubits, param_data);
                        delete!(param_data);
                        delete!(qubits);
                        let gate = cb_return(0, result)?;
                        take!(gate as Gate);
                        Ok(gate)
                    } else {
                        // No constructor defined.
                        inv_arg("no constructor function defined")
                    }
                },
            )),
        );
        Ok(())
    })
}

/// Uses a gate map object to convert an incoming DQCsim gate to the plugin's
/// representation.
///>
///> `gm` must be a handle to a gate map object (`dqcs_mm_new()`).
///> `gate` must be a handle to a gate. The handle is borrowed; it is not
///> mutated or deleted.
///> `key_data` serves as an optional return value; if non-NULL and a match is
///> found, the `key_data` specified when the respective detector was added is
///> returned here as a `const void *`. If no match is found, `*key_data` is
///> not assigned.
///> `qubits` serves as an optional return value; if non-NULL and a match
///> is found, it is set to a handle to a new `QubitSet` object representing the
///> gate's qubits. Ownership of this handle is passed to the user, so it
///> is up to the user to eventually delete it. If no match is found,
///> `*qubits` is set to 0.
///> `param_data` serves as an optional return value; if non-NULL and a match
///> is found, it is set to a handle to a new `ArbData` object representing the
///> gate's parameters. Ownership of this handle is passed to the user, so it
///> is up to the user to eventually delete it. If no match is found,
///> `*param_data` is set to 0.
///>
///> This function returns `DQCS_TRUE` if a match was found, `DQCS_FALSE` if no
///> match was found, or `DQCS_BOOL_FAILURE` if an error occurs.
#[no_mangle]
pub extern "C" fn dqcs_gm_detect(
    gm: dqcs_handle_t,
    gate: dqcs_handle_t,
    key_data: *mut *const c_void,
    qubits: *mut dqcs_handle_t,
    param_data: *mut dqcs_handle_t,
) -> dqcs_bool_return_t {
    api_return_bool(|| {
        if !qubits.is_null() {
            unsafe { *qubits = 0 };
        }
        if !param_data.is_null() {
            unsafe { *param_data = 0 };
        }
        resolve!(gm as &GateMap);
        resolve!(gate as &Gate);
        if let Some((key, (args, data))) = gm.map.detect(&gate)? {
            if !key_data.is_null() {
                unsafe { *key_data = key.raw() };
            }
            if !qubits.is_null() {
                let args: QubitReferenceSet = args.iter().cloned().collect();
                let handle = insert(args);
                unsafe { *qubits = handle };
            }
            if !param_data.is_null() {
                let handle = insert(data);
                unsafe { *param_data = handle };
            }
            Ok(true)
        } else {
            Ok(false)
        }
    })
}

/// Helper function with some common logic for the `dqcs_gm_construct*()`
/// functions.
fn construct_helper(
    gm: dqcs_handle_t,
    key_data: *const c_void,
    qubits: Vec<QubitRef>,
    param_data: dqcs_handle_t,
) -> Result<dqcs_handle_t> {
    let key = UserKeyData::new_borrowed(key_data);
    resolve!(gm as &GateMap);
    let key = gm.make_key(key);
    resolve!(optional param_data as pending ArbData);
    let data: ArbData = {
        if let Some(data) = param_data.as_ref() {
            let x: &ArbData = data.as_ref().unwrap();
            x.clone()
        } else {
            ArbData::default()
        }
    };
    let gate = insert(gm.map.construct(&(key, (qubits, data)))?);
    if let Some(mut param_data) = param_data {
        delete!(resolved param_data);
    }
    Ok(gate)
}

/// Uses a gate map object to construct a multi-qubit DQCsim gate from the
/// plugin's representation.
///>
///> `gm` must be a handle to a gate map object (`dqcs_mm_new()`).
///> `gate` must be a handle to a gate. The handle is borrowed; it is not
///> mutated or deleted.
///> `key_data` specifies the gate mapping key for the constructor to use. Note
///> that the *pointer* must match exactly to what was specified when the
///> mapping(s) was/were added.
///> `qubits` specifies the qubits arguments for the constructed gate. It is
///> up to the constructor function to determine how to interpret these. The
///> parameter is optional; passing 0 is equivalent to passing an empty qubit
///> set. The handle is deleted if the function succeeds.
///> `param_data` specifies the `ArbData` object used to parameterize the gate.
///> It is optional; if 0, an empty `ArbData` is automatically constructed by
///> DQCsim. The handle is deleted if the function succeeds.
///>
/// This function returns the handle to the gate, or 0 to indicate failure.
/// The qubit set and parameterization data (if specified) are consumed/deleted
/// by this function if and only if it succeeds.
#[no_mangle]
pub extern "C" fn dqcs_gm_construct(
    gm: dqcs_handle_t,
    key_data: *const c_void,
    qubits: dqcs_handle_t,
    param_data: dqcs_handle_t,
) -> dqcs_handle_t {
    api_return(0, || {
        // Resolve the optional qubit set.
        resolve!(optional qubits as pending QubitReferenceSet);
        let qubits_vec: Vec<QubitRef> = {
            if let Some(qubits) = qubits.as_ref() {
                let x: &QubitReferenceSet = qubits.as_ref().unwrap();
                x.iter().cloned().collect()
            } else {
                vec![]
            }
        };

        let gate = construct_helper(gm, key_data, qubits_vec, param_data)?;

        // Delete the qubit set when successful.
        if let Some(mut qubits) = qubits {
            delete!(resolved qubits);
        }

        Ok(gate)
    })
}

/// Uses a gate map object to construct a one-qubit DQCsim gate from the
/// plugin's representation.
///>
///> This function is simply a shorthand for `dqcs_gm_construct()` with
///> one qubit in the `qubits` set, to make constructing one-qubit gates more
///> ergonomic. Refer to its documentation for more information.
#[no_mangle]
pub extern "C" fn dqcs_gm_construct_one(
    gm: dqcs_handle_t,
    key_data: *const c_void,
    qa: dqcs_qubit_t,
    param_data: dqcs_handle_t,
) -> dqcs_handle_t {
    api_return(0, || {
        let qubits_vec = vec![QubitRef::from_foreign(qa)
            .ok_or_else(oe_inv_arg("0 is not a valid qubit reference"))?];
        construct_helper(gm, key_data, qubits_vec, param_data)
    })
}

/// Uses a gate map object to construct a two-qubit DQCsim gate from the
/// plugin's representation.
///>
///> This function is simply a shorthand for `dqcs_gm_construct()` with
///> two qubits in the `qubits` set, to make constructing two-qubit gates more
///> ergonomic. Refer to its documentation for more information.
#[no_mangle]
pub extern "C" fn dqcs_gm_construct_two(
    gm: dqcs_handle_t,
    key_data: *const c_void,
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
        construct_helper(gm, key_data, qubits_vec, param_data)
    })
}

/// Uses a gate map object to construct a three-qubit DQCsim gate from the
/// plugin's representation.
///>
///> This function is simply a shorthand for `dqcs_gm_construct()` with
///> three qubits in the `qubits` set, to make constructing three-qubit gates
///> more ergonomic. Refer to its documentation for more information.
#[no_mangle]
pub extern "C" fn dqcs_gm_construct_three(
    gm: dqcs_handle_t,
    key_data: *const c_void,
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
        construct_helper(gm, key_data, qubits_vec, param_data)
    })
}
