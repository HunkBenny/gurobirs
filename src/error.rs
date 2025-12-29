use crate::ffi;

pub(crate) fn check_err(error_code: i32) -> Result<i32, i32> {
    match error_code {
        ffi::GRB_ERROR_OVERFLOW
        | ffi::GRB_ERROR_GPU
        | ffi::GRB_ERROR_SECURITY
        | ffi::GRB_ERROR_TUNE_MODEL_TYPES
        | ffi::GRB_ERROR_CSWORKER
        | ffi::GRB_ERROR_MODEL_MODIFICATION
        | ffi::GRB_ERROR_CLOUD
        | ffi::GRB_ERROR_UPDATEMODE_CHANGE
        | ffi::GRB_ERROR_INVALID_PIECEWISE_OBJ
        | ffi::GRB_ERROR_EXCEED_2B_NONZEROS
        | ffi::GRB_ERROR_NOT_SUPPORTED
        | ffi::GRB_ERROR_JOB_REJECTED
        | ffi::GRB_ERROR_NETWORK
        | ffi::GRB_ERROR_QCP_EQUALITY_CONSTRAINT
        | ffi::GRB_ERROR_Q_NOT_PSD
        | ffi::GRB_ERROR_NODEFILE
        | ffi::GRB_ERROR_DUPLICATES
        | ffi::GRB_ERROR_OPTIMIZATION_IN_PROGRESS
        | ffi::GRB_ERROR_NOT_FOR_MIP
        | ffi::GRB_ERROR_IIS_NOT_INFEASIBLE
        | ffi::GRB_ERROR_NUMERIC
        | ffi::GRB_ERROR_FILE_WRITE
        | ffi::GRB_ERROR_FILE_READ
        | ffi::GRB_ERROR_CALLBACK
        | ffi::GRB_ERROR_SIZE_LIMIT_EXCEEDED
        | ffi::GRB_ERROR_NO_LICENSE
        | ffi::GRB_ERROR_VALUE_OUT_OF_RANGE
        | ffi::GRB_ERROR_UNKNOWN_PARAMETER
        | ffi::GRB_ERROR_INDEX_OUT_OF_RANGE
        | ffi::GRB_ERROR_DATA_NOT_AVAILABLE
        | ffi::GRB_ERROR_UNKNOWN_ATTRIBUTE
        | ffi::GRB_ERROR_INVALID_ARGUMENT
        | ffi::GRB_ERROR_NULL_ARGUMENT
        | ffi::GRB_ERROR_OUT_OF_MEMORY => Err(error_code),
        _ => Ok(error_code),
    }
}
