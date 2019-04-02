use super::*;

/// Creates a new `ArbCmd` queue object.
///
/// Returns the handle of the newly created `ArbCmd` queue. The queue is
/// initially empty. Queues implement a "first-in, first-out" model.
///
/// `ArbCmd` queue objects support the `handle`, `arb`, `cmd`, and `cq` APIs.
///
/// The `arb` and `cmd` APIs refer to the `ArbCmd` at the front of the queue.
/// Use `dqcs_cq_next()` to remove the front entry, allowing access to the next
/// command.
#[no_mangle]
pub extern "C" fn dqcs_cq_new() -> dqcs_handle_t {
    insert(ArbCmdQueue::new())
}

/// Pushes an `ArbCmd` object into the given `ArbCmd` queue.
///
/// This function returns -1 to indicate failure. The `ArbCmd` object specified
/// by `cmd` is moved into the queue. That is, the handle is consumed if and
/// only if the function succeeds.
pub extern "C" fn dqcs_cq_push(cq: dqcs_handle_t, cmd: dqcs_handle_t) -> dqcs_return_t {
    api_return_none(|| {
        resolve!(cq as &mut ArbCmdQueue);
        take!(cmd as ArbCmd);
        cq.push_back(cmd);
        Ok(())
    })
}

/// Advances an `ArbCmd` queue to the next command.
///
/// Use the `dqcs_arb_*` and `dqcs_cmd_*` interfaces to read out the command
/// before calling this function.
///
/// To iterate over a queue in C, use the following snippit:
///
/// ```C
/// for (; dqcs_cq_len(queue) > 0; dqcs_cq_next(queue)) {
///     dqcs_cmd_...(queue, ...)
///     dqcs_arb_...(queue, ...)
/// }
/// ```
pub extern "C" fn dqcs_cq_next(cq: dqcs_handle_t) -> dqcs_return_t {
    api_return_none(|| {
        resolve!(cq as &mut ArbCmdQueue);
        if cq.pop_front().is_some() {
            Ok(())
        } else {
            inv_arg("the command queue is already empty")
        }
    })
}

/// Returns the number of `ArbCmd` objects in the given `ArbCmd` queue.
///
/// This function returns -1 to indicate failure.
pub extern "C" fn dqcs_cq_len(cq: dqcs_handle_t) -> ssize_t {
    api_return(-1, || {
        resolve!(cq as &ArbCmdQueue);
        Ok(cq.len() as ssize_t)
    })
}
