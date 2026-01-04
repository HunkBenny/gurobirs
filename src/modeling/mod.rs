use crate::ffi;
use crate::{model::GRBModel, prelude::GRBCallbackContext};

pub(crate) trait CanBeAddedToModel {
    fn add_to_model(self, model: *mut ffi::GRBmodel) -> i32;
}

/// Marker trait for modeling objects (variables, constraints, etc)
/// This can then be used to implement generic functions that work with any modeling object
pub trait IsModelingObject {
    fn index(&self) -> usize;
}
// returns i32, because we need access to either a GRBModel or GRBEnv in order to handle errors
pub trait CanBeAddedToCallback {
    fn add_cut(self, callback: &mut GRBCallbackContext) -> i32;
    fn add_lazy(self, callback: &mut GRBCallbackContext) -> i32;
}

pub mod builder;
pub mod expr;
