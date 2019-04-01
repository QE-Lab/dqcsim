use super::*;
use std::ptr::null_mut;

/// Enumeration of all objects that can be associated with an handle, including
/// the object data.
#[derive(Debug)]
#[allow(dead_code, clippy::large_enum_variant)] // FIXME: clippy probably has a point here
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

impl From<ArbData> for APIObject {
    fn from(x: ArbData) -> APIObject {
        APIObject::ArbData(x)
    }
}

impl From<ArbCmd> for APIObject {
    fn from(x: ArbCmd) -> APIObject {
        APIObject::ArbCmd(x)
    }
}

impl From<PluginConfiguration> for APIObject {
    fn from(x: PluginConfiguration) -> APIObject {
        APIObject::PluginConfiguration(x)
    }
}

impl From<SimulatorConfiguration> for APIObject {
    fn from(x: SimulatorConfiguration) -> APIObject {
        APIObject::SimulatorConfiguration(x)
    }
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

/// Convenience function for converting a Result to an API return value and
/// possibly an error string.
///
/// Sets the `last_error` field in the `APIState` and returns `error_value` if
/// the result of `call()` is `Err`. Otherwise, the result of the callback is
/// returned without modification.
pub fn api_return<T>(error_value: T, call: impl FnOnce() -> Result<T>) -> T {
    match call() {
        Ok(x) => x,
        Err(e) => {
            API_STATE.with(|state| {
                state.borrow_mut().last_error.replace(
                    CString::new(e.to_string())
                        .unwrap_or_else(|_| CString::new("<UNKNOWN>").unwrap()),
                )
            });
            error_value
        }
    }
}

/// Same as `api_return()`, but specialized for `dqcs_return_t`.
pub fn api_return_none(call: impl FnOnce() -> Result<()>) -> dqcs_return_t {
    api_return(dqcs_return_t::DQCS_FAILURE, || {
        call().map(|()| dqcs_return_t::DQCS_SUCCESS)
    })
}

/// Same as `api_return()`, but specialized for `dqcs_bool_return_t`.
pub fn api_return_bool(call: impl FnOnce() -> Result<bool>) -> dqcs_bool_return_t {
    api_return(dqcs_bool_return_t::DQCS_BOOL_FAILURE, || {
        call().map(dqcs_bool_return_t::from)
    })
}

/// Same as `api_return()`, but specialized for returning strings.
pub fn api_return_string(call: impl FnOnce() -> Result<String>) -> *mut c_char {
    api_return(null_mut(), || {
        call().and_then(|s| {
            let s = CString::new(&s[..])?;
            let s = unsafe { strdup(s.as_ptr()) as *mut c_char };
            if s.is_null() {
                err("failed to allocate return value")
            } else {
                Ok(s)
            }
        })
    })
}

/// Structure used to access objects stored in the thread-local object pool.
///
/// While this object is in scope, the API object is removed from the pool.
/// Therefore, all forms of aliasing are prevented
pub struct ResolvedHandle {
    ob: Option<APIObject>,
    handle: dqcs_handle_t,
}

impl Drop for ResolvedHandle {
    /// If no ownership was taken over the API object with the given handle,
    /// insert it back into the thread-local object pool.
    fn drop(&mut self) {
        if let Some(ob) = self.ob.take() {
            API_STATE.with(|state| state.borrow_mut().objects.insert(self.handle, ob));
        }
    }
}

pub trait UseHandleAs<T> {
    /// Obtains an immutable reference to the embedded API object using the
    /// interface identified by T.
    fn as_ref(&self) -> Result<&T>;

    /// Obtains a mutable reference to the embedded API object using the
    /// interface identified by T.
    fn as_mut(&mut self) -> Result<&mut T>;

    /// Takes ownership of the embedded API object using the interface
    /// identified by T.
    fn take(&mut self) -> Result<T>;
}

impl UseHandleAs<APIObject> for ResolvedHandle {
    fn as_ref(&self) -> Result<&APIObject> {
        Ok(self
            .ob
            .as_ref()
            .expect("object ownership was already given away"))
    }

    fn as_mut(&mut self) -> Result<&mut APIObject> {
        Ok(self
            .ob
            .as_mut()
            .expect("object ownership was already given away"))
    }

    fn take(&mut self) -> Result<APIObject> {
        Ok(self
            .ob
            .take()
            .expect("object ownership was already given away"))
    }
}

macro_rules! mutate_api_object_as {
    {$x:ty, $y:ident: $($p:pat=>$r:expr,$m:expr,$t:expr,)+} => (
        impl UseHandleAs<$x> for ResolvedHandle {
            fn as_ref(&self) -> Result<&$x> {
                match self.ob.as_ref().expect("object ownership was already given away") {
                    $($p => Ok($r),)+
                    _ => inv_arg(format!("object does not support the {} interface", stringify!($y))),
                }
            }

            fn as_mut(&mut self) -> Result<&mut $x> {
                match self.ob.as_mut().expect("object ownership was already given away") {
                    $($p => Ok($m),)+
                    _ => inv_arg(format!("object does not support the {} interface", stringify!($y))),
                }
            }

            fn take(&mut self) -> Result<$x> {
                match self.ob.take().expect("object ownership was already given away") {
                    $($p => Ok($t),)+
                    x => {
                        self.ob.replace(x);
                        inv_arg(format!("object does not support the {} interface", stringify!($y)))
                    }
                }
            }
        }
    )
}

mutate_api_object_as! {ArbData, arb:
    APIObject::ArbData(x) => x, x, x,
    APIObject::ArbCmd(x) => x.data(), x.data_mut(), x.into(),
}

mutate_api_object_as! {ArbCmd, cmd:
    APIObject::ArbCmd(x) => x, x, x,
}

mutate_api_object_as! {PluginConfiguration, pcfg:
    APIObject::PluginConfiguration(x) => x, x, x,
}

mutate_api_object_as! {SimulatorConfiguration, scfg:
    APIObject::SimulatorConfiguration(x) => x, x, x,
}

/// Resolves an object from a handle.
///
/// This takes an object from the thread-local pool, so no reference is
/// maintained. The object is reinserted into the pool when the returned
/// container object is dropped, unless ownership of the object was taken
/// over.
pub fn resolve(handle: dqcs_handle_t) -> Result<ResolvedHandle> {
    if let Some(ob) = API_STATE.with(|state| state.borrow_mut().objects.remove(&handle)) {
        Ok(ResolvedHandle {
            ob: Some(ob),
            handle,
        })
    } else {
        inv_arg(format!("handle {} is invalid", handle))
    }
}

/*
/// Resolves an optional object from an optional handle.
///
/// This takes an object from the thread-local pool, so no reference is
/// maintained. The object is reinserted into the pool when the returned
/// container object is dropped, unless ownership of the object was taken
/// over.
pub fn resolve_opt(handle: dqcs_handle_t) -> Result<Option<ResolvedHandle>> {
    if handle == 0 {
        Ok(None)
    } else {
        Ok(Some(resolve(handle)?))
    }
}
*/

/// Resolves a handle into an underlying object.
///
/// This is a convenience macro for calling `resolve()` followed by `as_ref()`,
/// `as_mut()`, or `take()`. The following forms are available:
///
///  - `resolve!(x as &Type)`: converts `x` to `&Type` using let statements and
///    `as_ref()`.
///  - `resolve!(x as &mut Type)`: as above, but `as_mut()`.
///  - `resolve!(x as pending Type)`: only resolves the handle and ensures that
///    it CAN be downcast to `Type`, but doesn't perform the downcast yet. This
///    can be done later with `take!`. Reason being, that all possible errors
///    need to have been caught before `take()` is called, otherwise the object
///    cannot be placed back into the pool.
///
/// Note that this macro uses `?` to throw errors, so the callee must return
/// `Result<_>`.
#[macro_export]
macro_rules! resolve {
    ($i:ident as &$t:ty) => {
        let $i = resolve($i)?;
        let $i: &$t = $i.as_ref()?;
    };
    ($i:ident as &mut $t:ty) => {
        let mut $i = resolve($i)?;
        let $i: &mut $t = $i.as_mut()?;
    };
    ($i:ident as pending $t:ty) => {
        let mut $i = resolve($i)?;
        let _: &mut $t = $i.as_mut()?;
    };
}

/// Takes ownership of an handle previously resolved using the
/// `resolve!(x as pending Type)` syntax.
///
/// The following forms are available:
///
///  - `take!(x as Type)`: converts `x` to `Type` using `take()`.
///  - `take!(resolved x as Type)`: as above, but `x` must have previously been
///    resolved using `resolve!(x as pending Type)`. This cannot fail.
///  - `take!(x as mut Type)`: converts `x` to `mut Type` using `take()`.
///  - `take!(resolved x as mut Type)`: as above, but `x` must have previously
///    been resolved using `resolve!(x as pending Type)`. This cannot fail.
///
/// This macro does not throw errors, but it can panic if the downcast fails.
/// The downcast should have been previously checked by the user through the
/// appropriate `resolve!` macro; this will only fail if the types specified
/// for `resolve!` and `take!` differ (don't do this).
#[macro_export]
macro_rules! take {
    ($i:ident as $t:ty) => {
        let mut $i = resolve($i)?;
        let $i: $t = $i.take()?;
    };
    ($i:ident as mut $t:ty) => {
        let mut $i = resolve($i)?;
        let mut $i: $t = $i.take()?;
    };
    (resolved $i:ident as $t:ty) => {
        let $i: $t = $i.take().unwrap();
    };
    (resolved $i:ident as mut $t:ty) => {
        let mut $i: $t = $i.take().unwrap();
    };
}

/// Inserts an object into the thread-local pool.
///
/// The handle to the object is returned.
pub fn insert(ob: impl Into<APIObject>) -> dqcs_handle_t {
    API_STATE.with(|state| {
        let mut state = state.borrow_mut();
        state.push(ob.into())
    })
}
