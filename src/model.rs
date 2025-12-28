use std::ptr::{null, null_mut};

use crate::{
    builder::CanBeAddedToModel,
    env::GRBenv,
    ffi,
    var::{GRBVar, GRBVarType},
};

pub struct GRBModel {
    inner: *mut ffi::GRBmodel,
    var_index: usize,
    cons_index: usize,
}

impl GRBModel {
    pub fn new(env: GRBenv) -> GRBModel {
        let mut model = null_mut();
        let error = unsafe {
            ffi::GRBnewmodel(
                env.inner(),
                &mut model,
                null(),
                0,
                null_mut(),
                null_mut(),
                null_mut(),
                null_mut(),
                null_mut(),
            )
        };
        // start indexes at 0 (per docs)
        GRBModel {
            inner: model,
            var_index: 0,
            cons_index: 0,
        }
    }

    // TODO: Add GRBVarBuilder support
    pub fn add_var<T>(&mut self, var: T) -> GRBVar
    where
        T: CanBeAddedToModel,
    {
        // add to model
        var.add_to_model(self);
        // create GRBVar Rust - object
        let var = GRBVar::new(self.var_index);
        self.var_index += 1;
        var
    }

    pub fn inner_mut(&mut self) -> *mut ffi::GRBmodel {
        self.inner
    }
}
// TODO: getters
