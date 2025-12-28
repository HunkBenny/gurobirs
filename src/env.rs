use std::{
    ffi::CString,
    ptr::{null, null_mut},
};

use crate::ffi;

pub struct GRBenv {
    inner: *mut ffi::GRBenv,
}

impl GRBenv {
    pub fn inner(&self) -> *mut ffi::GRBenv {
        self.inner
    }

    pub fn new(empty: bool, logfilename: Option<&str>) -> Result<GRBenv, String> {
        // Create the GRBenv pointer
        let mut env_ptr = null_mut();
        // Prepare the logfilename pointer
        let logfilename_ptr = match logfilename.map(|s| CString::new(s)) {
            Some(Ok(cstr)) => cstr.as_ptr(),
            Some(Err(_)) => return Err("Failed to convert logfilename to CString".to_string()),
            None => null(),
        };
        // Call the appropriate FFI function
        let error = if empty {
            unsafe { ffi::GRBemptyenv(&mut env_ptr) }
        } else {
            unsafe { ffi::GRBloadenv(&mut env_ptr, logfilename_ptr) }
        };
        // TODO: handle error
        Ok(GRBenv { inner: env_ptr })
    }

    pub fn start(&mut self) -> () {
        unsafe {
            ffi::GRBstartenv(self.inner);
        }
    }
}

impl Default for GRBenv {
    fn default() -> Self {
        GRBenv::new(false, None).expect("Failed to create default GRBenv")
    }
}

impl Drop for GRBenv {
    fn drop(&mut self) {
        unsafe {
            ffi::GRBfreeenv(self.inner);
        }
    }
}

// TODO: Implement;
// [get](https://docs.gurobi.com/projects/optimizer/en/current/reference/cpp/env.html#_CPPv4N6GRBEnv3getE15GRB_DoubleParam)
// [getErrorMsg](https://docs.gurobi.com/projects/optimizer/en/current/reference/cpp/env.html#_CPPv4N6GRBEnv11getErrorMsgEv)
// [getParamInfo](https://docs.gurobi.com/projects/optimizer/en/current/reference/cpp/env.html#_CPPv4N6GRBEnv12getParamInfoE15GRB_StringParamP6stringP6string)
// also set, readParams, resetParams, writeParams etc.
