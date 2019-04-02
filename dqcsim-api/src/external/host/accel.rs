use super::*;

/// Starts a program on the accelerator.
///
/// This is an asynchronous call: nothing happens until `yield()`,
/// `recv()`, or `wait()` is called.
///
/// The `ArbData` handle is optional; if 0 is passed, an empty data object is
/// used. If a handle is passed, it is consumed if and only if the API call
/// succeeds.
#[no_mangle]
pub extern "C" fn dqcs_accel_start(accel: dqcs_handle_t, data: dqcs_handle_t) -> dqcs_return_t {
    api_return_none(|| {
        resolve!(accel as &mut APIObject);
        resolve!(data as pending ArbData);
        let data_ob = {
            let x: &ArbData = data.as_ref().unwrap();
            x.clone()
        };
        match accel {
            APIObject::Simulator(x) => x.simulation.start(data_ob)?,
            _ => inv_arg("object does not support the accel interface".to_string())?,
        }
        take!(resolved data as ArbData);
        let _ = data;
        Ok(())
    })
}

/// Waits for the accelerator to finish its current program.
///
/// When this succeeds, the return value of the accelerator's `run()`
/// function is returned in the form of a new handle. When it fails, 0 is
/// returned.
///
/// Deadlocks are detected and prevented by returning an error.
#[no_mangle]
pub extern "C" fn dqcs_accel_wait(accel: dqcs_handle_t) -> dqcs_handle_t {
    api_return(0, || {
        resolve!(accel as &mut APIObject);
        Ok(insert(match accel {
            APIObject::Simulator(x) => x.simulation.wait()?,
            _ => inv_arg("object does not support the accel interface".to_string())?,
        }))
    })
}

/// Sends a message to the accelerator.
///
/// This is an asynchronous call: nothing happens until `yield()`,
/// `recv()`, or `wait()` is called.
///
/// The `ArbData` handle is optional; if 0 is passed, an empty data object is
/// used. If a handle is passed, it is consumed if and only if the API call
/// succeeds.
#[no_mangle]
pub extern "C" fn dqcs_accel_send(accel: dqcs_handle_t, data: dqcs_handle_t) -> dqcs_return_t {
    api_return_none(|| {
        resolve!(accel as &mut APIObject);
        resolve!(data as pending ArbData);
        let data_ob = {
            let x: &ArbData = data.as_ref().unwrap();
            x.clone()
        };
        match accel {
            APIObject::Simulator(x) => x.simulation.send(data_ob)?,
            _ => inv_arg("object does not support the accel interface".to_string())?,
        }
        take!(resolved data as ArbData);
        let _ = data;
        Ok(())
    })
}

/// Waits for the accelerator to send a message to us.
///
/// When this succeeds, the received data is returned in the form of a new
/// handle. When it fails, 0 is returned.
///
/// Deadlocks are detected and prevented by returning an error.
#[no_mangle]
pub extern "C" fn dqcs_accel_recv(accel: dqcs_handle_t) -> dqcs_handle_t {
    api_return(0, || {
        resolve!(accel as &mut APIObject);
        Ok(insert(match accel {
            APIObject::Simulator(x) => x.simulation.recv()?,
            _ => inv_arg("object does not support the accel interface".to_string())?,
        }))
    })
}
