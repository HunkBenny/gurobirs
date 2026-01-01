// The plan should be as follows:
// 1. user writes a Rust function (`my_callback_fn`).
// 2. user uses `model.set_callback(my_callback_fn)` to register the callback.
// 3. internally, we create a C-compatible function pointer that calls `my_callback_fn`.
//    This way, we can guarantee that we don't continuously call a callback_function.

use gurobi_sys::GRBmsg;

use crate::error::check_err;
use crate::ffi;
use crate::model::GRBModel;
use crate::modeling::CanBeAddedToCallback;
use crate::prelude::{GRBSense, LinExpr};
use std::ffi::{c_char, CStr};
use std::panic::{catch_unwind, AssertUnwindSafe};

pub struct GRBCallbackContext {
    model: *mut ffi::GRBmodel,
    cb_data: *mut std::ffi::c_void,
    pub where_: i32,
}

pub struct GRBCallback<C: CallbackTrait> {
    callback: C,
}

pub trait CallbackTrait {
    fn callback(&self, cb_ctx: GRBCallbackContext);
}

impl<C: CallbackTrait> GRBCallback<C> {
    pub fn new(callback: C) -> Self {
        Self { callback }
    }
}

unsafe extern "C" fn c_shim<C: CallbackTrait>(
    model: *mut ffi::GRBmodel,
    cb_data: *mut std::ffi::c_void,
    where_: i32,
    user_data: *mut std::ffi::c_void,
) -> i32 {
    let wrapper = user_data as *mut GRBCallback<C>;
    let cb_ctx = GRBCallbackContext {
        model,
        cb_data,
        where_,
    };
    let result = catch_unwind(AssertUnwindSafe(|| unsafe {
        (*wrapper).callback.callback(cb_ctx)
    }));
    match result {
        Ok(_) => 0,
        Err(_) => ffi::GRB_ERROR_CALLBACK,
    }
}

impl GRBModel {
    pub fn set_callback<C: CallbackTrait>(&mut self, callback: &mut GRBCallback<C>) {
        unsafe {
            // PERF: Check if [GRBsetcallbackfuncadv](https://docs.gurobi.com/projects/optimizer/en/current/reference/c/logging.html#c.GRBsetcallbackfuncadv) could lead to performance improvements in certain scenarios.
            ffi::GRBsetcallbackfunc(
                self.inner(),
                Some(c_shim::<C>),
                callback as *mut _ as *mut std::ffi::c_void,
            );
        }
    }
}

#[macro_export]
macro_rules! __internal_wrap_callback {
    ($user_func:expr) => {{
        unsafe extern "C" fn cb(
            _model: *mut ffi::GRBmodel,
            _cb_data: *mut std::ffi::c_void,
            _where: i32,
            _data: *mut std::ffi::c_void,
        ) -> i32 {
            let res = std::panic::catch_unwind(|| $user_func());
            match res {
                Ok(_) => 0,
                Err(_) => 1,
            }
        }
        cb
    }};
}

impl GRBCallbackContext {
    pub fn get_error(&self, error_code: i32) -> Result<(), String> {
        match check_err(error_code) {
            Err(e) => unsafe {
                Err(format!(
                    "ERROR CODE {}: {}",
                    e,
                    CStr::from_ptr(ffi::GRBgetmerrormsg(self.model) as *mut c_char)
                        .to_string_lossy()
                ))
            },
            Ok(_o) => Ok(()),
        }
    }

    pub fn abort(&self) {
        unsafe {
            ffi::GRBterminate(self.model);
        }
    }

    pub fn add_cut<E: CanBeAddedToCallback>(&mut self, expr: E) {
        let error = expr.add_cut(self);
        self.get_error(error).unwrap();
    }

    pub fn add_lazy<E: CanBeAddedToCallback>(&mut self, expr: E) {
        let error = expr.add_lazy(self);
        self.get_error(error).unwrap();
    }
}
