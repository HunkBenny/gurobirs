// Approach should be to create constr via builder pattern
// The build method should return a TempConstr that can be added to the model
// This way, we can overload '==', '<=', '>=' operators to create TempConstr

use std::{
    ffi::{c_void, CStr, CString},
    ptr::null_mut,
};

use crate::{
    error::check_err,
    ffi,
    model::GRBModelPtr,
    modeling::{
        expr::{lin_expr::GRBLinExpr, quad_expr::GRBQuadExpr, GRBSense},
        AddAsIndicator, CanBeAddedToCallback, CanBeAddedToModel, IsModelingObject,
    },
    prelude::GRBCallbackContext,
};

pub trait ConstrGetter {
    type Value;
    fn get(&self, constr: &GRBConstr) -> Self::Value;
}

pub trait ConstrSetter {
    type Value;
    fn set(&self, constr: &GRBConstr, value: Self::Value) -> i32;
}

pub struct TempConstr {
    linear_terms: Vec<(usize, f64)>,
    sense: GRBSense,
    rhs: f64,
    name: Option<CString>,
}

pub struct TempQConstr {
    linear_terms: Vec<(usize, f64)>,
    quadratic_terms: Vec<((usize, usize), f64)>,
    sense: GRBSense,
    rhs: f64,
    name: Option<CString>,
}

impl TempConstr {
    pub fn get_linear_inds_and_coeffs(&self) -> (Vec<std::ffi::c_int>, Vec<std::ffi::c_double>) {
        let mut linear_terms_inds = Vec::new();
        let mut linear_terms_coeffs = Vec::new();
        // linear terms
        for (var_idx, coeff) in self.linear_terms.iter() {
            linear_terms_inds.push(*var_idx as std::ffi::c_int);
            linear_terms_coeffs.push(*coeff as std::ffi::c_double);
        }

        (linear_terms_inds, linear_terms_coeffs)
    }

    pub fn name(mut self, name: &str) -> Self {
        self.name = Some(CString::new(name).unwrap());
        self
    }
}

impl TempQConstr {
    pub fn get_quadratic_inds_and_coeffs(
        &self,
    ) -> (Vec<i32>, Vec<f64>, Vec<i32>, Vec<i32>, Vec<f64>) {
        let mut linear_terms_inds = Vec::new();
        let mut linear_terms_coeffs = Vec::new();
        // linear terms
        for (var_idx, coeff) in self.linear_terms.iter() {
            linear_terms_inds.push(*var_idx as i32);
            linear_terms_coeffs.push(*coeff);
        }
        let mut quadratic_terms_inds_rows = Vec::new();
        let mut quadratic_terms_inds_cols = Vec::new();
        let mut quadratic_terms_coeffs = Vec::new();
        for ((var_idx1, var_idx2), coeff) in self.quadratic_terms.iter() {
            quadratic_terms_inds_rows.push(*var_idx1 as i32);
            quadratic_terms_inds_cols.push(*var_idx2 as i32);
            quadratic_terms_coeffs.push(*coeff);
        }
        // quadratic terms
        (
            linear_terms_inds,
            linear_terms_coeffs,
            quadratic_terms_inds_rows,
            quadratic_terms_inds_cols,
            quadratic_terms_coeffs,
        )
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

impl Expr for GRBLinExpr {
    type Output = TempConstr;

    fn eq(self, rhs: f64) -> Self::Output {
        let rhs = rhs - self.scalar;
        TempConstr {
            linear_terms: self.expr.into_iter().collect(),
            sense: GRBSense::Equal,
            rhs,
            name: None,
        }
    }

    fn ge(self, rhs: f64) -> Self::Output {
        let rhs = rhs - self.scalar;
        TempConstr {
            linear_terms: self.expr.into_iter().collect(),
            sense: GRBSense::GreaterEqual,
            rhs,
            name: None,
        }
    }

    fn le(self, rhs: f64) -> Self::Output {
        let rhs = rhs - self.scalar;
        TempConstr {
            linear_terms: self.expr.into_iter().collect(),
            sense: GRBSense::LessEqual,
            rhs,
            name: None,
        }
    }
}

impl Expr for GRBQuadExpr {
    type Output = TempQConstr;
    fn eq(self, rhs: f64) -> Self::Output {
        let rhs = rhs - self.linear_expr.scalar;
        TempQConstr {
            linear_terms: self.linear_expr.expr.into_iter().collect(),
            quadratic_terms: self.quad_expr.into_iter().collect(),
            sense: GRBSense::Equal,
            rhs,
            name: None,
        }
    }

    fn ge(self, rhs: f64) -> Self::Output {
        let rhs = rhs - self.linear_expr.scalar;
        TempQConstr {
            linear_terms: self.linear_expr.expr.into_iter().collect(),
            quadratic_terms: self.quad_expr.into_iter().collect(),
            sense: GRBSense::GreaterEqual,
            rhs,
            name: None,
        }
    }

    fn le(self, rhs: f64) -> Self::Output {
        let rhs = rhs - self.linear_expr.scalar;
        TempQConstr {
            linear_terms: self.linear_expr.expr.into_iter().collect(),
            quadratic_terms: self.quad_expr.into_iter().collect(),
            sense: GRBSense::LessEqual,
            rhs,
            name: None,
        }
    }
}

impl CanBeAddedToModel for TempConstr {
    fn add_to_model(self, model: *mut ffi::GRBmodel, name: *const std::ffi::c_char) -> i32 {
        // 1. collect indices and coefficients
        let (mut inds_linear, mut coeffs_linear) = self.get_linear_inds_and_coeffs();

        // 3. call GRBaddconstr or GRBaddqconstr based on presence of quadratic terms
        unsafe {
            ffi::GRBaddconstr(
                model,
                inds_linear.len() as std::ffi::c_int,
                inds_linear.as_mut_ptr(),
                coeffs_linear.as_mut_ptr(),
                self.sense.into(),
                self.rhs,
                name,
            )
        }
    }

    fn get_name(&mut self) -> Option<CString> {
        self.name.take()
    }
}

impl CanBeAddedToModel for TempQConstr {
    fn add_to_model(self, model: *mut ffi::GRBmodel, name: *const std::ffi::c_char) -> i32 {
        // 1. collect indices and coefficients
        let (
            mut inds_linear,
            mut coeffs_linear,
            mut inds_nonlinear_row,
            mut inds_nonlinear_col,
            mut coeffs_nonlinear,
        ) = self.get_quadratic_inds_and_coeffs();

        // 3. call GRBaddconstr or GRBaddqconstr based on presence of quadratic terms
        unsafe {
            ffi::GRBaddqconstr(
                model,
                inds_linear.len() as std::ffi::c_int,
                inds_linear.as_mut_ptr(),
                coeffs_linear.as_mut_ptr(),
                inds_nonlinear_row.len() as std::ffi::c_int,
                inds_nonlinear_row.as_mut_ptr(),
                inds_nonlinear_col.as_mut_ptr(),
                coeffs_nonlinear.as_mut_ptr(),
                self.sense.into(),
                self.rhs,
                name,
            )
        }
    }

    fn get_name(&mut self) -> Option<CString> {
        self.name.take()
    }
}

impl CanBeAddedToCallback for TempConstr {
    fn add_cut(self, callback: &mut GRBCallbackContext) -> i32 {
        // 1. collect indices and coefficients
        let (mut inds, mut coeffs) = self.get_linear_inds_and_coeffs();
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
        let (mut inds, mut coeffs) = self.get_linear_inds_and_coeffs();
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

impl GRBConstr {
    pub fn get_error(&self, error_code: i32) -> Result<(), String> {
        match check_err(error_code) {
            Err(e) => unsafe {
                Err(format!(
                    "ERROR CODE {}: {}",
                    e,
                    CStr::from_ptr(ffi::GRBgetmerrormsg(*self.inner.0) as *mut std::ffi::c_char)
                        .to_string_lossy()
                ))
            },
            Ok(_o) => Ok(()),
        }
    }

    pub fn set<V: ConstrSetter>(&self, setter: V, value: V::Value) {
        let err_code = setter.set(self, value);
        self.get_error(err_code).unwrap();
    }

    pub fn get<G: ConstrGetter>(&self, getter: G) -> G::Value {
        getter.get(self)
    }
}

pub struct GRBConstr {
    pub index: usize,
    pub(crate) inner: GRBModelPtr,
}

impl IsModelingObject for GRBConstr {
    fn index(&self) -> usize {
        self.index
    }
}

impl AddAsIndicator for TempConstr {
    fn add_as_indicator(
        self,
        model: *mut ffi::GRBmodel,
        binvar: crate::prelude::GRBVar,
        binval: i8,
        name: *const std::ffi::c_char,
    ) -> i32 {
        let (inds, coeffs) = self.get_linear_inds_and_coeffs();
        let len = inds.len();
        unsafe {
            ffi::GRBaddgenconstrIndicator(
                model,
                name,
                binvar.index() as std::ffi::c_int,
                binval as std::ffi::c_int,
                len as std::ffi::c_int,
                inds.as_ptr(),
                coeffs.as_ptr(),
                self.sense.into(),
                self.rhs as std::ffi::c_double,
            )
        }
    }
}
