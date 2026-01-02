// Approach should be to create constr via builder pattern
// The build method should return a TempConstr that can be added to the model
// This way, we can overload '==', '<=', '>=' operators to create TempConstr

use std::{
    ffi::{c_void, CString},
    ptr::null_mut,
};

use crate::{
    ffi,
    modeling::{
        expr::{lin_expr::LinExpr, GRBSense},
        CanBeAddedToCallback, CanBeAddedToModel, IsModelingObject,
    },
    prelude::GRBCallbackContext,
};

pub struct TempConstr {
    lhs: LinExpr,
    sense: GRBSense,
    rhs: f64,
    name: Option<CString>,
}

impl TempConstr {
    pub fn get_inds_and_coeffs(&self) -> (Vec<i32>, Vec<f64>) {
        let mut inds = Vec::new();
        let mut coeffs = Vec::new();
        for (var_idx, coeff) in self.lhs.expr.iter() {
            inds.push(*var_idx as i32);
            coeffs.push(*coeff);
        }
        (inds, coeffs)
    }

    pub fn name(mut self, name: &str) -> Self {
        self.name = Some(CString::new(name).unwrap());
        self
    }
}

/// Trait that allows LinExpr to become constraint
pub trait Expr {
    type Output;

    fn eq(self, rhs: f64) -> Self::Output;
    fn ge(self, rhs: f64) -> Self::Output;
    fn le(self, rhs: f64) -> Self::Output;
}

impl Expr for LinExpr {
    type Output = TempConstr;

    fn eq(self, rhs: f64) -> Self::Output {
        TempConstr {
            lhs: self,
            sense: GRBSense::Equal,
            rhs,
            name: None,
        }
    }

    fn ge(self, rhs: f64) -> Self::Output {
        TempConstr {
            lhs: self,
            sense: GRBSense::GreaterEqual,
            rhs,
            name: None,
        }
    }

    fn le(self, rhs: f64) -> Self::Output {
        TempConstr {
            lhs: self,
            sense: GRBSense::LessEqual,
            rhs,
            name: None,
        }
    }
}

impl CanBeAddedToModel for TempConstr {
    fn add_to_model(self, model: &mut crate::model::GRBModel) {
        // 1. collect indices and coefficients
        let (mut inds, mut coeffs) = self.get_inds_and_coeffs();

        // 2. handle name
        let name_ptr = match self.name {
            Some(cname) => cname.as_ptr(),
            None => null_mut(),
        };

        // 3. call GRBaddconstr
        let error = unsafe {
            ffi::GRBaddconstr(
                model.inner(),
                inds.len() as i32,
                inds.as_mut_ptr(),
                coeffs.as_mut_ptr(),
                self.sense.into(),
                self.rhs,
                name_ptr,
            )
        };
        model.get_error(error).unwrap();
    }
}

impl CanBeAddedToCallback for TempConstr {
    fn add_cut(self, callback: &mut GRBCallbackContext) -> i32 {
        // 1. collect indices and coefficients
        let (mut inds, mut coeffs) = self.get_inds_and_coeffs();
        // 2. call GRBcbcut
        unsafe {
            ffi::GRBcbcut(
                callback as *mut GRBCallbackContext as *mut c_void,
                inds.len() as i32,
                inds.as_mut_ptr(),
                coeffs.as_mut_ptr(),
                self.sense.into(),
                self.rhs,
            )
        }
    }

    fn add_lazy(self, callback: &mut GRBCallbackContext) -> i32 {
        // 1. collect indices and coefficients
        let (mut inds, mut coeffs) = self.get_inds_and_coeffs();
        // 2. call GRBcbcut
        unsafe {
            ffi::GRBcblazy(
                callback as *mut GRBCallbackContext as *mut c_void,
                inds.len() as i32,
                inds.as_mut_ptr(),
                coeffs.as_mut_ptr(),
                self.sense.into(),
                self.rhs,
            )
        }
    }
}

pub struct GRBConstr {
    pub index: usize,
}

impl IsModelingObject for GRBConstr {
    fn index(&self) -> usize {
        self.index
    }
}
