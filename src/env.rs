use std::{
    ffi::{c_char, CStr, CString},
    ptr::{null, null_mut},
    rc::Rc,
};

use crate::{error::check_err, ffi};

#[derive(Clone)]
pub struct GRBEnv {
    inner: Rc<*mut ffi::GRBenv>,
}

impl GRBEnv {
    pub fn inner(&self) -> *mut ffi::GRBenv {
        *self.inner
    }

    pub fn new(empty: bool, logfilename: Option<&str>) -> Result<GRBEnv, String> {
        // Create the GRBenv pointer
        let mut env_ptr = null_mut();
        // Prepare the logfilename CString (must live until after the FFI call)
        let logfilename_cstring = logfilename
            .map(|s| CString::new(s))
            .transpose()
            .map_err(|_| "Failed to convert logfilename to CString".to_string())?;

        let logfilename_ptr = logfilename_cstring
            .as_ref()
            .map(|cstr| cstr.as_ptr())
            .unwrap_or(null());

        // Call the appropriate FFI function
        let error = if empty {
            unsafe { ffi::GRBemptyenv(&mut env_ptr) }
        } else {
            unsafe { ffi::GRBloadenv(&mut env_ptr, logfilename_ptr) }
        };
        let env = GRBEnv {
            inner: Rc::new(env_ptr),
        };
        env.get_error(error).unwrap();
        Ok(env)
    }

    pub fn start(&mut self) {
        unsafe {
            ffi::GRBstartenv(*self.inner);
        }
    }

    pub fn get_error(&self, error_code: i32) -> Result<(), String> {
        match check_err(error_code) {
            Err(e) => unsafe {
                Err(format!(
                    "ERROR CODE {}: {}",
                    e,
                    CStr::from_ptr(ffi::GRBgeterrormsg(self.inner()) as *mut c_char)
                        .to_string_lossy()
                ))
            },
            Ok(_o) => Ok(()),
        }
    }
}

impl Default for GRBEnv {
    fn default() -> Self {
        GRBEnv::new(false, None).expect("Failed to create default GRBenv")
    }
}

impl Drop for GRBEnv {
    fn drop(&mut self) {
        if Rc::strong_count(&self.inner) > 1 {
            // If there are other references to the inner pointer, we should not free it yet
            return;
        }
        unsafe {
            ffi::GRBfreeenv(*self.inner);
        }
    }
}

// TODO: Implement;
// [get](https://docs.gurobi.com/projects/optimizer/en/current/reference/cpp/env.html#_CPPv4N6GRBEnv3getE15GRB_DoubleParam)
// [getErrorMsg](https://docs.gurobi.com/projects/optimizer/en/current/reference/cpp/env.html#_CPPv4N6GRBEnv11getErrorMsgEv)
// [getParamInfo](https://docs.gurobi.com/projects/optimizer/en/current/reference/cpp/env.html#_CPPv4N6GRBEnv12getParamInfoE15GRB_StringParamP6stringP6string)
// also set, readParams, resetParams, writeParams etc.
