use super::*;
use std::collections::VecDeque;
use std::ptr::null_mut;
use std::thread::JoinHandle;

pub type ArbCmdQueue = VecDeque<ArbCmd>;

pub type QubitReferenceSet = VecDeque<QubitRef>;

pub type QubitMeasurementResultSet = HashMap<QubitRef, QubitMeasurementResult>;

pub type PluginJoinHandle = JoinHandle<Result<()>>;

pub type BoxedPluginConfiguration = Box<dyn PluginConfiguration>;

macro_rules! api_object_types {
    ($($(#[$m:meta])* $i:ident,)+) => (
        /// Enumeration of all objects that can be associated with an handle, including
        /// the object data.
        #[derive(Debug)]
        #[allow(dead_code, clippy::large_enum_variant)]
        pub enum APIObject {
            $(
                $(#[$m])*
                $i($i),
            )+
        }

        $(
            impl From<$i> for APIObject {
                fn from(x: $i) -> APIObject {
                    APIObject::$i(x)
                }
            }
        )+
    )
}

api_object_types!(
    /// `ArbData` object.
    ArbData,
    /// `ArbCmd` object.
    ArbCmd,
    /// Queue of `ArbCmd` objects.
    ArbCmdQueue,
    /// Set of qubit references.
    QubitReferenceSet,
    /// Quantum gate object.
    Gate,
    /// Qubit measurement object.
    QubitMeasurementResult,
    /// Set of qubit measurement objects.
    QubitMeasurementResultSet,
    /// `PluginProcessConfiguration` object.
    PluginProcessConfiguration,
    /// `PluginThreadConfiguration` object.
    PluginThreadConfiguration,
    /// `SimulatorConfiguration` object.
    SimulatorConfiguration,
    /// DQCsim simulation instance, behaving as an accelerator.
    Simulator,
    /// `PluginDefinition` object.
    PluginDefinition,
    /// Join handle for a plugin thread.
    PluginJoinHandle,
);

/// Thread-local state storage structure.
pub struct APIState {
    /// Mapping from handle to object.
    ///
    /// This contains all API-managed data.
    pub objects: HashMap<dqcs_handle_t, APIObject>,

    /// This variable stores the handle value that will be used for the next
    /// allocation.
    pub handle_counter: dqcs_handle_t,

    /// The handle that currently owns DQCsim's thread-local resources. This
    /// serves as a lock to ensure that no more than one object using
    /// thread-locals is constructed.
    pub thread_locals_used_by: Option<dqcs_handle_t>,

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

    /// Claims the thread-local storage lock for the given handle.
    ///
    /// The lock is released when the handle becomes invalid.
    pub fn thread_locals_claim(&mut self, handle: dqcs_handle_t) -> Result<()> {
        self.thread_locals_assert_free()?;
        self.thread_locals_used_by.replace(handle);
        Ok(())
    }

    /// Returns whether DQCsim's thread-locals are currently in use.
    pub fn thread_locals_claimed(&self) -> bool {
        if let Some(h) = self.thread_locals_used_by {
            self.objects.contains_key(&h)
        } else {
            false
        }
    }

    /// Asserts that DQCsim's thread-locals are not in use.
    pub fn thread_locals_assert_free(&self) -> Result<()> {
        if self.thread_locals_claimed() {
            inv_op(format!(
                "cannot claim DQCsim thread-local storage; already claimed by handle {}",
                self.thread_locals_used_by.unwrap()
            ))
        } else {
            Ok(())
        }
    }
}

impl Drop for APIState {
    fn drop(&mut self) {
        let mut warn = false;
        for (_, v) in self.objects.drain() {
            if let APIObject::Simulator(_) = v {
                warn = true;
                std::mem::forget(v);
            }
        }
        if warn {
            eprintln!(
                "DQCsim API error: you've leaked one or more Simulator objects! \
                 You should always call dqcs_handle_delete() on simulator objects or call \
                 dqcs_handle_clear() to delete all handles before exiting, otherwise \
                 things are not destroyed in the right order."
            );
        }
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
        thread_locals_used_by: None,
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

/// Convenience function for converting an API callback return value to a
/// Result object.
///
/// If `actual_value` equals `error_value`, `Err` is returned, taking the
/// message from the `last_error` field in the `APIState`. Otherwise, `Ok`
/// is returned, containing `actual_value`.
pub fn cb_return<T>(error_value: T, actual_value: T) -> Result<T>
where
    T: std::cmp::PartialEq,
{
    if actual_value != error_value {
        Ok(actual_value)
    } else {
        API_STATE.with(|state| {
            let state = state.borrow();
            if let Some(e) = state.last_error.as_ref() {
                err(e
                    .clone()
                    .into_string()
                    .unwrap_or_else(|_| "Unknown error".to_string()))
            } else {
                err("Unknown error")
            }
        })
    }
}

/// Same as `cb_return()`, but specialized for `dqcs_return_t`.
pub fn cb_return_none(actual_value: dqcs_return_t) -> Result<()> {
    cb_return(dqcs_return_t::DQCS_FAILURE, actual_value).map(|_| ())
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
            #[allow(unreachable_code, unused_variables)]
            fn as_ref(&self) -> Result<&$x> {
                match self.ob.as_ref().expect("object ownership was already given away") {
                    $($p => Ok($r),)+
                    _ => inv_arg(format!("object does not support the {} interface", stringify!($y))),
                }
            }

            #[allow(unreachable_code, unused_variables)]
            fn as_mut(&mut self) -> Result<&mut $x> {
                match self.ob.as_mut().expect("object ownership was already given away") {
                    $($p => Ok($m),)+
                    _ => inv_arg(format!("object does not support the {} interface", stringify!($y))),
                }
            }

            #[allow(unreachable_code, unused_variables)]
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
    APIObject::ArbCmdQueue(x) => {
        if let Some(x) = x.front() {
            x.data()
        } else {
            return inv_arg("empty command queue does not support arb interface");
        }
    }, {
        if let Some(x) = x.front_mut() {
            x.data_mut()
        } else {
            return inv_arg("empty command queue does not support arb interface");
        }
    }, {
        let mut x = x;
        if let Some(x) = x.pop_front() {
            x.into()
        } else {
            return inv_arg("empty command queue does not support arb interface");
        }
    },
    APIObject::Gate(x) => &x.data, &mut x.data, x.data,
    APIObject::QubitMeasurementResult(x) => &x.data, &mut x.data, x.data,
}

mutate_api_object_as! {ArbCmd, cmd:
    APIObject::ArbCmd(x) => x, x, x,
    APIObject::ArbCmdQueue(x) => {
        if let Some(x) = x.front() {
            x
        } else {
            return inv_arg("empty command queue does not support cmd interface");
        }
    }, {
        if let Some(x) = x.front_mut() {
            x
        } else {
            return inv_arg("empty command queue does not support cmd interface");
        }
    }, {
        let mut x = x;
        if let Some(x) = x.pop_front() {
            x
        } else {
            return inv_arg("empty command queue does not support cmd interface");
        }
    },
}

mutate_api_object_as! {ArbCmdQueue, cq:
    APIObject::ArbCmdQueue(x) => x, x, x,
}

mutate_api_object_as! {QubitReferenceSet, qbset:
    APIObject::QubitReferenceSet(x) => x, x, x,
}

mutate_api_object_as! {Gate, gate:
    APIObject::Gate(x) => x, x, x,
}

mutate_api_object_as! {QubitMeasurementResult, meas:
    APIObject::QubitMeasurementResult(x) => x, x, x,
}

mutate_api_object_as! {QubitMeasurementResultSet, mset:
    APIObject::QubitMeasurementResult(x) => {
        return inv_arg("handle does not support the mset interface");
    }, {
        return inv_arg("handle does not support the mset interface");
    }, {
        let mut s = QubitMeasurementResultSet::new();
        s.insert(x.qubit, x);
        s
    },
    APIObject::QubitMeasurementResultSet(x) => x, x, x,
}

mutate_api_object_as! {PluginProcessConfiguration, pcfg:
    APIObject::PluginProcessConfiguration(x) => x, x, x,
}

mutate_api_object_as! {PluginThreadConfiguration, tcfg:
    APIObject::PluginThreadConfiguration(x) => x, x, x,
}

mutate_api_object_as! {BoxedPluginConfiguration, xcfg:
    APIObject::PluginProcessConfiguration(x) => panic!(), panic!(), Box::new(x),
    APIObject::PluginThreadConfiguration(x) => panic!(), panic!(), Box::new(x),
}

mutate_api_object_as! {SimulatorConfiguration, scfg:
    APIObject::SimulatorConfiguration(x) => x, x, x,
}

mutate_api_object_as! {Simulator, sim:
    APIObject::Simulator(x) => x, x, x,
}

mutate_api_object_as! {PluginDefinition, scfg:
    APIObject::PluginDefinition(x) => x, x, x,
}

mutate_api_object_as! {PluginJoinHandle, pjoin:
    APIObject::PluginJoinHandle(x) => x, x, x,
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
#[doc(hidden)]
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
    (optional $i:ident as &$t:ty) => {
        let $i = resolve($i).ok();
        let $i: Option<&$t> = if let Some($i) = $i.as_ref() {
            Some($i.as_ref()?)
        } else {
            None
        };
    };
    (optional $i:ident as &mut $t:ty) => {
        let mut $i = resolve($i).ok();
        let $i: Option<&$t> = if let Some($i) = $i.as_mut() {
            Some($i.as_ref()?)
        } else {
            None
        };
    };
    (optional $i:ident as pending $t:ty) => {
        let mut $i = resolve($i).ok();
        if let Some($i) = $i.as_mut() {
            let _: &mut $t = $i.as_mut()?;
        }
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
#[doc(hidden)]
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

#[doc(hidden)]
#[macro_export]
macro_rules! clone {
    ($clone:ident: $t:ty = resolved $i:ident) => {
        let $clone: &$t = $i.as_ref().unwrap();
        let $clone = $clone.clone();
    };
    (mut $clone:ident: $t:ty = resolved $i:ident) => {
        let $clone: &$t = $i.as_ref().unwrap();
        let mut $clone = $clone.clone();
    };
}

/// Deletes a handle.
///
/// This never returns an error, and "double deletes" are fine (the second
/// delete will be no-op).
#[doc(hidden)]
#[macro_export]
macro_rules! delete {
    ($i:ident) => {
        let $i = resolve($i);
        if let Ok(mut $i) = $i {
            let _: APIObject = $i.take().unwrap();
        }
    };
    (resolved $i:ident) => {
        let _: APIObject = $i.take().unwrap();
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
