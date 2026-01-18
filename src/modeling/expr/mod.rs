use crate::ffi;
pub mod lin_expr;
pub mod nonlin_expr;
pub mod quad_expr;

pub enum GRBSense {
    LessEqual,
    Equal,
    GreaterEqual,
}
impl From<GRBSense> for std::ffi::c_char {
    fn from(sense: GRBSense) -> Self {
        match sense {
            GRBSense::LessEqual => ffi::GRB_LESS_EQUAL as std::ffi::c_char,
            GRBSense::Equal => ffi::GRB_EQUAL as std::ffi::c_char,
            GRBSense::GreaterEqual => ffi::GRB_GREATER_EQUAL as std::ffi::c_char,
        }
    }
}

// NOTE: I think I should keep track of the LHS and keep all scalars on the RHS?
// Then I can call ffi::GRBaddconstr() with the correct inputs
