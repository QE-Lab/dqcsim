use super::*;

/// Enumeration of all objects that can be associated with an handle, including
/// the object data.
#[derive(Debug)]
#[allow(dead_code)]
pub enum APIObject {
    /// ArbData object.
    ArbData(ArbData),

    /// ArbData object.
    ArbCmd(ArbCmd),

    /// PluginConfiguration object.
    PluginConfiguration(PluginConfiguration),

    /// SimulatorConfiguration object.
    SimulatorConfiguration(SimulatorConfiguration),
}

/// Thread-local state storage structure.
pub struct APIState {
    /// Mapping from handle to object.
    ///
    /// This contains all API-managed data.
    pub objects: HashMap<dqcs_handle_t, APIObject>,

    /// This variable stores the handle value that will be used for the next
    /// allocation.
    pub handle_counter: dqcs_handle_t,

    /// This variable records the error message associated with the latest
    /// failure.
    pub last_error: Option<CString>,
}

impl APIState {
    /// Sets the `last_error` field in the `APIState` if val is `Err` and
    /// calls `error` for the return value, or returns the value carried in the
    /// `Ok`.
    pub fn result_to_api<T>(&mut self, val: Result<T>, error: impl FnOnce() -> T) -> T {
        match val {
            Ok(x) => x,
            Err(e) => {
                self.last_error = Some(
                    CString::new(e.to_string())
                        .unwrap_or_else(|_| CString::new("<UNKNOWN>").unwrap()),
                );
                error()
            }
        }
    }

    /// Stuffs the given object into the API-managed storage. Returns the
    /// handle created for it.
    pub fn push(&mut self, object: APIObject) -> dqcs_handle_t {
        let handle = self.handle_counter;
        self.objects.insert(handle, object);
        self.handle_counter = handle + 1;
        handle
    }
}

thread_local! {
    /// Thread-local state storage. Be careful not to call user callback functions
    /// while holding a reference to the state: those callbacks can and probably
    /// will claim the reference mutably at some point. Basically, once user
    /// callbacks need to become "callable" from the Rust world, for instance when
    /// a configuration object is consumed into the object it configures, they
    /// should move out of this state.
    pub static API_STATE: RefCell<APIState> = RefCell::new(APIState {
        objects: HashMap::new(),
        handle_counter: 1,
        last_error: None,
    });
}

/// Convenience function for operating on the thread-local state object.
pub fn with_api_state<T>(
    error: impl FnOnce() -> T,
    call: impl FnOnce(std::cell::RefMut<APIState>) -> Result<T>,
) -> T {
    API_STATE.with(|state| {
        let result = call(state.borrow_mut());
        state.borrow_mut().result_to_api(result, error)
    })
}

/// Convenience function for converting a Result to an API return value and
/// possibly an error string while API_STATE is *not* already borrowed.
/// Sets the `last_error` field in the `APIState` if val is `Err` and
/// calls `error` for the return value, or returns the value carried in the
/// `Ok`.
///
/// Use `ApiState::result_to_api()` if you already have already borrowed a
/// reference.
pub fn result_to_api<T>(val: Result<T>, error: impl FnOnce() -> T) -> T {
    if let Ok(x) = val {
        x
    } else {
        API_STATE.with(|state| state.borrow_mut().result_to_api(val, error))
    }
}
