use super::*;

/// Creates a new set of qubit references.
///
/// Returns the handle of the newly created set. The set is initially empty.
/// Qubit sets are ordered, meaning that the order in which qubits are popped
/// from the set equals the order in which they were pushed. To iterate over a
/// set, simply make a copy and drain the copy using pop.
#[no_mangle]
pub extern "C" fn dqcs_qbset_new() -> dqcs_handle_t {
    insert(QubitReferenceSet::new())
}

/// Returns whether the given qubit set contains the given qubit,
///
/// This function will fail if the specified qubit was already part of the set.
pub extern "C" fn dqcs_qbset_contains(
    qbset: dqcs_handle_t,
    qubit: dqcs_qubit_t,
) -> dqcs_bool_return_t {
    api_return_bool(|| {
        resolve!(qbset as &mut QubitReferenceSet);
        let qubit = QubitRef::from_foreign(qubit)
            .ok_or_else(oe_inv_arg("0 is not a valid qubit reference"))?;
        Ok(qbset.contains(&qubit))
    })
}

/// Pushes a qubit reference into a qubit reference set.
///
/// This function will fail if the specified qubit was already part of the set.
pub extern "C" fn dqcs_qbset_push(qbset: dqcs_handle_t, qubit: dqcs_qubit_t) -> dqcs_return_t {
    api_return_none(|| {
        resolve!(qbset as &mut QubitReferenceSet);
        let qubit = QubitRef::from_foreign(qubit)
            .ok_or_else(oe_inv_arg("0 is not a valid qubit reference"))?;
        if qbset.contains(&qubit) {
            inv_arg("the specified qubit is already part of the set")
        } else {
            qbset.push_back(qubit);
            Ok(())
        }
    })
}

/// Pops a qubit reference off of a qubit reference set.
///
/// Qubits are popped in the same order in which they were pushed. That is,
/// they are FIFO-ordered.
pub extern "C" fn dqcs_qbset_pop(qbset: dqcs_handle_t) -> dqcs_qubit_t {
    api_return(0, || {
        resolve!(qbset as &mut QubitReferenceSet);
        if let Some(qubit) = qbset.pop_front() {
            Ok(qubit.to_foreign())
        } else {
            inv_arg("the qubit set is already empty")
        }
    })
}

/// Returns the number of qubits in the given set.
///
/// This function returns -1 to indicate failure.
pub extern "C" fn dqcs_qbset_len(qbset: dqcs_handle_t) -> ssize_t {
    api_return(-1, || {
        resolve!(qbset as &QubitReferenceSet);
        Ok(qbset.len() as ssize_t)
    })
}

/// Returns a copy of the given qubit set, intended for non-destructive
/// iteration.
///
/// Iteration works like this.
///
/// ```C
/// dqcs_handle_t it = dqcs_qbset_copy(qbset);
/// dqcs_qubit_t qubit = 0;
/// while (qubit = dqcs_qbset_pop(it)) {
///     ...
/// }
/// dqcs_handle_delete(it);
/// ```
///
/// Of course, if you don't care about keeping the set intact, you don't have
/// to make a copy.
pub extern "C" fn dqcs_qbset_copy(qbset: dqcs_handle_t) -> dqcs_handle_t {
    api_return(0, || {
        resolve!(qbset as &QubitReferenceSet);
        Ok(insert(qbset.clone()))
    })
}
