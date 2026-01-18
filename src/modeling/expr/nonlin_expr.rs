// Would be nice if this file would contain functionality that resembles scip. For the moment it is
// not necessary though.

use crate::ffi;
#[derive(Copy, Clone)]
pub enum GRBOpCode {
    Constant,
    Variable,
    Plus,
    Minus,
    Multiply,
    Divide,
    Uminus,
    Square,
    Sqrt,
    Sin,
    Cos,
    Tan,
    Pow,
    Exp,
    Log,
    Log2,
    Log10,
    Logistic,
    Tanh,
    Signpow,
}

impl From<GRBOpCode> for std::ffi::c_int {
    fn from(value: GRBOpCode) -> Self {
        match value {
            GRBOpCode::Constant => ffi::GRB_OPCODE_CONSTANT,
            GRBOpCode::Variable => ffi::GRB_OPCODE_VARIABLE,
            GRBOpCode::Plus => ffi::GRB_OPCODE_PLUS,
            GRBOpCode::Minus => ffi::GRB_OPCODE_MINUS,
            GRBOpCode::Multiply => ffi::GRB_OPCODE_MULTIPLY,
            GRBOpCode::Divide => ffi::GRB_OPCODE_DIVIDE,
            GRBOpCode::Uminus => ffi::GRB_OPCODE_UMINUS,
            GRBOpCode::Square => ffi::GRB_OPCODE_SQUARE,
            GRBOpCode::Sqrt => ffi::GRB_OPCODE_SQRT,
            GRBOpCode::Sin => ffi::GRB_OPCODE_SIN,
            GRBOpCode::Cos => ffi::GRB_OPCODE_COS,
            GRBOpCode::Tan => ffi::GRB_OPCODE_TAN,
            GRBOpCode::Pow => ffi::GRB_OPCODE_POW,
            GRBOpCode::Exp => ffi::GRB_OPCODE_EXP,
            GRBOpCode::Log => ffi::GRB_OPCODE_LOG,
            GRBOpCode::Log2 => ffi::GRB_OPCODE_LOG2,
            GRBOpCode::Log10 => ffi::GRB_OPCODE_LOG10,
            GRBOpCode::Logistic => ffi::GRB_OPCODE_LOGISTIC,
            GRBOpCode::Tanh => ffi::GRB_OPCODE_TANH,
            GRBOpCode::Signpow => ffi::GRB_OPCODE_SIGNPOW,
        }
    }
}
