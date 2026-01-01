use crate::{model::GRBModel, prelude::GRBCallbackContext};

pub(crate) trait CanBeAddedToModel {
    fn add_to_model(self, model: &mut GRBModel);
}

// returns i32, because we need access to either a GRBModel or GRBEnv in order to handle errors
pub trait CanBeAddedToCallback {
    fn add_cut(self, callback: &mut GRBCallbackContext) -> i32;
    fn add_lazy(self, callback: &mut GRBCallbackContext) -> i32;
}

pub mod builder;
pub mod expr;
