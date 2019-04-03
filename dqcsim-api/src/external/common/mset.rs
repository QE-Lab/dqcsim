use super::*;

/// Creates a new set of qubit measurement results.
///
/// Returns the handle of the newly created set. The set is initially empty.
#[no_mangle]
pub extern "C" fn dqcs_mset_new() -> dqcs_handle_t {
    insert(QubitMeasurementResultSet::new())
}

/// Returns whether the given qubit measurement set contains data for the given
/// qubit.
#[no_mangle]
pub extern "C" fn dqcs_mset_contains(
    mset: dqcs_handle_t,
    qubit: dqcs_qubit_t,
) -> dqcs_bool_return_t {
    api_return_bool(|| {
        resolve!(mset as &mut QubitMeasurementResultSet);
        let qubit = QubitRef::from_foreign(qubit)
            .ok_or_else(oe_inv_arg("0 is not a valid qubit reference"))?;
        Ok(mset.contains_key(&qubit))
    })
}

/// Adds a measurement result to a measurement result set.
///
/// If there was already a measurement for the specified qubit, the previous
/// measurement result is overwritten. The measurement result object is deleted
/// if and only if the function succeeds.
#[no_mangle]
pub extern "C" fn dqcs_mset_set(mset: dqcs_handle_t, meas: dqcs_handle_t) -> dqcs_return_t {
    api_return_none(|| {
        resolve!(mset as &mut QubitMeasurementResultSet);
        resolve!(meas as pending QubitMeasurementResult);
        let meas_ob: &QubitMeasurementResult = meas.as_ref()?;
        mset.insert(meas_ob.qubit, meas_ob.clone());
        delete!(resolved meas);
        Ok(())
    })
}

/// Returns a copy of the measurement result for the given qubit from a
/// measurement result set.
#[no_mangle]
pub extern "C" fn dqcs_mset_get(mset: dqcs_handle_t, qubit: dqcs_qubit_t) -> dqcs_handle_t {
    api_return(0, || {
        resolve!(mset as &QubitMeasurementResultSet);
        let qubit = QubitRef::from_foreign(qubit)
            .ok_or_else(oe_inv_arg("0 is not a valid qubit reference"))?;
        Ok(insert(
            mset.get(&qubit)
                .ok_or_else(oe_inv_arg("qubit not included in measurement set"))?
                .clone(),
        ))
    })
}

/// Returns the measurement result for the given qubit from a measurement
/// result set and removes it from the set.
#[no_mangle]
pub extern "C" fn dqcs_mset_take(mset: dqcs_handle_t, qubit: dqcs_qubit_t) -> dqcs_handle_t {
    api_return(0, || {
        resolve!(mset as &mut QubitMeasurementResultSet);
        let qubit = QubitRef::from_foreign(qubit)
            .ok_or_else(oe_inv_arg("0 is not a valid qubit reference"))?;
        Ok(insert(
            mset.remove(&qubit)
                .ok_or_else(oe_inv_arg("qubit not included in measurement set"))?
                .clone(),
        ))
    })
}

/// Removes the measurement result for the given qubit from a measurement
/// result set.
#[no_mangle]
pub extern "C" fn dqcs_mset_remove(mset: dqcs_handle_t, qubit: dqcs_qubit_t) -> dqcs_return_t {
    api_return_none(|| {
        resolve!(mset as &mut QubitMeasurementResultSet);
        let qubit = QubitRef::from_foreign(qubit)
            .ok_or_else(oe_inv_arg("0 is not a valid qubit reference"))?;
        mset.remove(&qubit)
            .ok_or_else(oe_inv_arg("qubit not included in measurement set"))?;
        Ok(())
    })
}
