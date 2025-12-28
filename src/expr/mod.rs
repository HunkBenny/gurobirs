use crate::ffi;
pub mod lin_expr;

pub enum GRBSense {
    LessEqual,
    Equal,
    GreaterEqual,
}

impl GRBSense {
    pub fn to_grb_char(&self) -> std::ffi::c_char {
        match self {
            GRBSense::LessEqual => ffi::GRB_LESS_EQUAL as std::ffi::c_char,
            GRBSense::Equal => ffi::GRB_EQUAL as std::ffi::c_char,
            GRBSense::GreaterEqual => ffi::GRB_GREATER_EQUAL as std::ffi::c_char,
        }
    }
}

//NOTE: I think I should keep track of the LHS and keep all scalars on the RHS?
// Then I can call ffi::GRBaddconstr() with the correct inputs
