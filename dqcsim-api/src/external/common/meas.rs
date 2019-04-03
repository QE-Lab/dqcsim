use super::*;

/// Constructs a new measurement object.
///
/// `qubit` must be set to the qubit that was measured, `value` must be set to
/// its value. The return value is the handle to the measurement object, or 0
/// if something went wrong.
///
/// Note that measurement objects implement the `arb` interface, so additional
/// data can be attached to the object.
#[no_mangle]
pub extern "C" fn dqcs_meas_new(qubit: dqcs_qubit_t, value: dqcs_measurement_t) -> dqcs_handle_t {
    api_return(0, || {
        let value: Option<QubitMeasurementValue> = value.into();
        Ok(insert(QubitMeasurementResult {
            qubit: QubitRef::from_foreign(qubit)
                .ok_or_else(oe_inv_arg("0 is not a valid qubit reference"))?,
            value: value.ok_or_else(oe_inv_arg("invalid measurement value specified"))?,
            data: ArbData::default(),
        }))
    })
}

/// Returns the qubit reference associated with a measurement object.
#[no_mangle]
pub extern "C" fn dqcs_meas_qubit_get(meas: dqcs_handle_t) -> dqcs_qubit_t {
    api_return(0, || {
        resolve!(meas as &QubitMeasurementResult);
        Ok(meas.qubit.to_foreign())
    })
}

/// Sets the qubit reference associated with a measurement object.
#[no_mangle]
pub extern "C" fn dqcs_meas_qubit_set(meas: dqcs_handle_t, qubit: dqcs_qubit_t) -> dqcs_return_t {
    api_return_none(|| {
        resolve!(meas as &mut QubitMeasurementResult);
        meas.qubit = QubitRef::from_foreign(qubit)
            .ok_or_else(oe_inv_arg("0 is not a valid qubit reference"))?;
        Ok(())
    })
}

/// Returns the measurement value associated with a measurement object.
#[no_mangle]
pub extern "C" fn dqcs_meas_value_get(meas: dqcs_handle_t) -> dqcs_measurement_t {
    api_return(dqcs_measurement_t::DQCS_MEAS_INVALID, || {
        resolve!(meas as &QubitMeasurementResult);
        Ok(meas.value.into())
    })
}

/// Sets the measurement value associated with a measurement object.
#[no_mangle]
pub extern "C" fn dqcs_meas_value_set(
    meas: dqcs_handle_t,
    value: dqcs_measurement_t,
) -> dqcs_return_t {
    api_return_none(|| {
        resolve!(meas as &mut QubitMeasurementResult);
        let value: Option<QubitMeasurementValue> = value.into();
        meas.value = value.ok_or_else(oe_inv_arg("invalid measurement value specified"))?;
        Ok(())
    })
}
