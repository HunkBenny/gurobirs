use std::ffi::CStr;

use crate::{error::check_err, ffi, model::GRBModelInner, modeling::IsModelingObject};

pub trait VariableSetter {
    type Value;
    fn set(&self, var: &GRBVar<'_>, value: Self::Value) -> i32;
}

pub trait VariableGetter {
    type Value;
    fn get(&self, var: &GRBVar<'_>) -> Self::Value;
}

#[allow(clippy::upper_case_acronyms, non_camel_case_types)]
pub enum GRBVarType {
    CONTINUOUS,
    BINARY,
    INTEGER,
    SEMICONT,
    SEMIINT,
}

impl From<GRBVarType> for std::ffi::c_char {
    fn from(value: GRBVarType) -> Self {
        match value {
            GRBVarType::CONTINUOUS => ffi::GRB_CONTINUOUS as std::ffi::c_char,
            GRBVarType::BINARY => ffi::GRB_BINARY as std::ffi::c_char,
            GRBVarType::INTEGER => ffi::GRB_INTEGER as std::ffi::c_char,
            GRBVarType::SEMICONT => ffi::GRB_SEMICONT as std::ffi::c_char,
            GRBVarType::SEMIINT => ffi::GRB_SEMIINT as std::ffi::c_char,
        }
    }
}

#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(Clone, Copy)]
pub struct GRBVar<'a> {
    index: usize,
    pub(crate) inner: &'a GRBModelInner,
}

impl<'a> GRBVar<'a> {
    pub fn new(index: usize, inner: &'a GRBModelInner) -> GRBVar<'a> {
        GRBVar { index, inner }
    }

    pub fn set<V: VariableSetter>(&self, setter: V, value: V::Value) {
        let err_code = setter.set(self, value);
        self.get_error(err_code).unwrap();
    }

    pub fn get<G: VariableGetter>(&self, getter: G) -> G::Value {
        getter.get(self)
    }

    pub fn get_error(&self, error_code: i32) -> Result<(), String> {
        match check_err(error_code) {
            Err(e) => unsafe {
                Err(format!(
                    "ERROR CODE {}: {}",
                    e,
                    CStr::from_ptr(ffi::GRBgetmerrormsg(self.inner.model) as *mut std::ffi::c_char)
                        .to_string_lossy()
                ))
            },
            Ok(_o) => Ok(()),
        }
    }
}

impl IsModelingObject for GRBVar<'_> {
    fn index(&self) -> usize {
        self.index
    }
}
