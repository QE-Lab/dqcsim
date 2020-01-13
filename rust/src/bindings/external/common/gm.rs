use super::*;
use crate::common::gates::GateType;
use std::{convert::TryFrom, rc::Rc};

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
    insert(GateMap::new(
        strip_qubit_refs,
        strip_data,
        key_cmp,
        key_hash,
    ))
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
///>  - `DQCS_GATE_RX`, `DQCS_GATE_RY`, and `DQCS_GATE_RZ` insert/pop a 64-bit
///>    double floating point with the angle at binary string index 0.
///>  - `DQCS_GATE_R` inserts/pops theta at binary string index 0, phi at index
///>    1, and lambda at index 2. They represent 64-bit double floating points.
///>  - `DQCS_GATE_U` inserts/pops the entire matrix as a single argument at
///>    index 0, consisting of 2**N * 2**N * 2 doubles, in real-first row-major
///>    format (same as the other matrix definitions in DQCsim).
#[no_mangle]
pub extern "C" fn dqcs_gm_add_predef_unitary(
    gm: dqcs_handle_t,
    key_free: Option<extern "C" fn(user_data: *mut c_void)>,
    key_data: *mut c_void,
    gate: dqcs_internal_gate_t,
    num_controls: c_int,
    epsilon: c_double,
    ignore_gphase: bool,
) -> dqcs_return_t {
    api_return_none(|| {
        resolve!(gm as &mut GateMap);
        let key = UserKeyData::Owned(Rc::new(UserData::new(key_free, key_data)));
        let gate_type = GateType::try_from(gate)?;
        gm.add_predef_unitary(key, gate_type, num_controls, epsilon, ignore_gphase);
        Ok(())
    })
}

/// Uses a gate map object to convert an incoming DQCsim gate to the plugin's
/// representation.
///>
///> `mm` must be a handle to a gate map object (`dqcs_mm_new()`).
///> `gate` must be a handle to a gate. The handle is borrowed; it is not
///> mutated or deleted.
///> `key_data` serves as an optional return value; if non-NULL and a match is
///> found, the `key_data` specified when the respective detector was added is
///> returned here as a `const void *`. If no match is found, `*key_data` is
///> not assigned.
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
    _gm: dqcs_handle_t,
    _gate: dqcs_handle_t,
    _skey_data: *mut *const c_void,
    param_data: *mut dqcs_handle_t,
) -> dqcs_bool_return_t {
    api_return_bool(|| {
        if !param_data.is_null() {
            unsafe { *param_data = 0 };
        }
        // resolve!(gm as &GateMap);
        // resolve!(gate as &Gate);
        todo!()
    })
}
