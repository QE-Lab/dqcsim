use super::*;
use std::cell::RefCell;
use std::ptr::null_mut;

struct UserData {
    user_free: Option<extern "C" fn(*mut c_void)>,
    user_data: *mut c_void,
}

impl Drop for UserData {
    fn drop(&mut self) {
        if let Some(user_free) = self.user_free {
            user_free(self.user_data);
        }
    }
}

impl UserData {
    pub fn new(user_free: Option<extern "C" fn(*mut c_void)>, user_data: *mut c_void) -> UserData {
        UserData {
            user_free,
            user_data,
        }
    }
}

type TestCallback = dyn Fn(i32, i32) -> Result<i32, Error>;

struct Banana {
    test: Option<Box<TestCallback>>,
}

thread_local! {
    static BANANA: RefCell<Banana> = RefCell::new(Banana { test: None });
}

/// Installs a test callback.
///
/// `callback` is the callback function to install. It is always called with
/// the `user_data` pointer to make calling stuff like class member functions
/// or closures possible. The `user_free` function, if non-null, will be called
/// when the callback is uninstalled in any way. If `callback` is null, any
/// current callback is uninstalled instead. For consistency, if `user_free` is
/// non-null while `callback` is null, `user_free` is called immediately, under
/// the assumption that the caller has allocated resources unbeknownst that the
/// callback it's trying to install is null.
#[no_mangle]
pub extern "C" fn dqcs_cb_test_install(
    callback: Option<extern "C" fn(*mut c_void, i32, i32) -> i32>,
    user_free: Option<extern "C" fn(*mut c_void)>,
    user_data: *mut c_void,
) -> dqcs_return_t {
    BANANA.with(|banana| {
        let mut banana = banana.borrow_mut();

        let data = UserData::new(user_free, user_data);

        if let Some(callback) = callback {
            banana.test = Some(Box::new(move |a: i32, b: i32| {
                match callback(data.user_data, a, b) {
                    -1 => Err(APIError::Generic(receive_str(dqcs_explain())?.to_string()).into()),
                    x => Ok(x),
                }
            }));
        } else {
            banana.test = None;
        }
    });
    dqcs_return_t::DQCS_SUCCESS
}

/// This should also work...
#[no_mangle]
#[allow(unused_variables)]
pub extern "C" fn dqcs_cb_test_foobar_install(
    a: i32,
    b: *const c_char,
    callback: Option<extern "C" fn(*mut c_void, i32, i32) -> i32>,
    user_free: Option<extern "C" fn(*mut c_void)>,
    user_data: *mut c_void,
) -> dqcs_return_t {
    dqcs_cb_test_install(callback, user_free, user_data)
}

/// Uninstalls the test callback.
///
/// Same as calling `dqcs_cb_test_install(NULL, NULL, NULL)`.
#[no_mangle]
pub extern "C" fn dqcs_cb_test_uninstall() -> dqcs_return_t {
    dqcs_cb_test_install(None, None, null_mut())
}

/// Calls the test callback.
///
/// If no callback is installed, -1 is returned.
#[no_mangle]
pub extern "C" fn dqcs_cb_test_call(a: i32, b: i32) -> i32 {
    BANANA.with(|banana| {
        let banana = banana.borrow();
        if let Some(test) = banana.test.as_ref() {
            let x = test(a, b);
            with_state(|| -1, |_| x)
        } else {
            -1
        }
    })
}
