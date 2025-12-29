// Approach should be to create constr via builder pattern
// The build method should return a TempConstr that can be added to the model
// This way, we can overload '==', '<=', '>=' operators to create TempConstr

use std::{ffi::CString, ptr::null_mut};

use crate::{
    ffi,
    model::GRBModel,
    modeling::builder::CanBeAddedToModel,
    modeling::expr::{lin_expr::LinExpr, GRBSense},
};

pub struct TempConstr {
    lhs: LinExpr,
    sense: GRBSense,
    rhs: f64,
    name: Option<CString>,
}

impl TempConstr {
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
        let mut inds = Vec::new();
        let mut coeffs = Vec::new();
        for (var_idx, coeff) in self.lhs.expr.into_iter() {
            inds.push(var_idx as i32);
            coeffs.push(coeff);
        }
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
                self.sense.to_grb_char(),
                self.rhs,
                name_ptr,
            )
        };
        model.get_error(error).unwrap();
    }
}

pub struct GRBConstr {
    pub index: usize,
}

impl GRBModel {}
