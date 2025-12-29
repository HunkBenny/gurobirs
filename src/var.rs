use crate::ffi;

#[allow(clippy::upper_case_acronyms, non_camel_case_types)]
pub enum GRBVarType {
    CONTINUOUS,
    BINARY,
    INTEGER,
    SEMICONT,
    SEMIINT,
}

impl GRBVarType {
    /// Get the Type as a c_char for FFI calls
    pub fn grb_type(&self) -> std::ffi::c_char {
        match self {
            GRBVarType::CONTINUOUS => ffi::GRB_CONTINUOUS as std::ffi::c_char,
            GRBVarType::BINARY => ffi::GRB_BINARY as std::ffi::c_char,
            GRBVarType::INTEGER => ffi::GRB_INTEGER as std::ffi::c_char,
            GRBVarType::SEMICONT => ffi::GRB_SEMICONT as std::ffi::c_char,
            GRBVarType::SEMIINT => ffi::GRB_SEMIINT as std::ffi::c_char,
        }
    }
}

#[derive(Clone, Copy)]
pub struct GRBVar {
    index: usize,
}

impl GRBVar {
    pub fn index(&self) -> usize {
        self.index
    }

    pub fn new(index: usize) -> GRBVar {
        GRBVar { index }
    }
}

// TODO: Get int attr
