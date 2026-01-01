use std::{
    ffi::{c_char, CStr},
    ptr::{null, null_mut},
};

use crate::{
    callback::GRBCallback,
    constr::GRBConstr,
    env::GRBenv,
    error::check_err,
    ffi,
    modeling::{builder::CanBeAddedToModel, expr::lin_expr::LinExpr},
    var::GRBVar,
};

pub struct GRBModel {
    inner: *mut ffi::GRBmodel,
    var_index: usize,
    cons_index: usize,
    pub(crate) callback: Option<fn()>,
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
        env.get_error(error).unwrap();
        // start indexes at 0 (per docs)
        GRBModel {
            inner: model,
            var_index: 0,
            cons_index: 0,
            callback: None,
        }
    }

    pub fn add_var<T>(&mut self, var: T) -> GRBVar
    where
        T: CanBeAddedToModel,
    {
        // add to model
        var.add_to_model(self);
        // create GRBVar Rust-object
        let var = GRBVar::new(self.var_index);
        self.var_index += 1;
        var
    }

    pub fn inner(&self) -> *mut ffi::GRBmodel {
        self.inner
    }

    pub fn add_constr<E: CanBeAddedToModel>(&mut self, expr: E) -> GRBConstr {
        expr.add_to_model(self);
        let constr = GRBConstr {
            index: self.cons_index,
        };
        self.cons_index += 1;
        constr
    }

    pub fn set_objective(&mut self, obj: LinExpr, sense: GRBModelSense) {
        // set constant term
        let constant_term = obj.scalar;

        let error = unsafe {
            ffi::GRBsetdblattr(
                self.inner(),
                ffi::GRB_DBL_ATTR_OBJCON.as_ptr(),
                constant_term,
            )
        };
        self.get_error(error).unwrap();
        // set coeffs
        for (var_idx, coeff) in obj.expr {
            let error = unsafe {
                ffi::GRBsetdblattrelement(
                    self.inner(),
                    ffi::GRB_DBL_ATTR_OBJ.as_ptr(),
                    var_idx as i32,
                    coeff,
                )
            };
            self.get_error(error).unwrap();
        }
        // Set model sense
        let error = unsafe {
            ffi::GRBsetintattr(
                self.inner(),
                ffi::GRB_INT_ATTR_MODELSENSE.as_ptr(),
                GRBModelSense::get(sense),
            )
        };
        self.get_error(error).unwrap();
    }

    pub fn optimize(&mut self) {
        let error = unsafe { ffi::GRBoptimize(self.inner()) };
        match self.get_error(error) {
            Ok(_) => (),
            Err(e) => {
                panic!("{}", e);
            }
        }
    }

    pub fn get_error(&self, error_code: i32) -> Result<(), String> {
        match check_err(error_code) {
            Err(e) => unsafe {
                Err(format!(
                    "ERROR CODE {}: {}",
                    e,
                    CStr::from_ptr(ffi::GRBgetmerrormsg(self.inner()) as *mut c_char)
                        .to_string_lossy()
                ))
            },
            Ok(_o) => Ok(()),
        }
    }
}

pub enum GRBModelSense {
    MAXIMIZE,
    MINIMIZE,
}
// FIX: call this on object itself instead of associated function
impl GRBModelSense {
    pub fn get(sense: GRBModelSense) -> i32 {
        match sense {
            GRBModelSense::MINIMIZE => ffi::GRB_MINIMIZE,
            GRBModelSense::MAXIMIZE => ffi::GRB_MAXIMIZE,
        }
    }
}
// TODO: getters
