use std::{
    collections::BTreeMap,
    ops::{Add, AddAssign, Mul, Sub, SubAssign},
};

use crate::{ffi, modeling::IsModelingObject};
use crate::{
    modeling::{expr::lin_expr::GRBLinExpr, Objective},
    var::GRBVar,
};

pub struct QuadExpr {
    pub(crate) quad_expr: BTreeMap<(usize, usize), f64>, // (var_idx1, var_idx2, coeff)
    pub(crate) linear_expr: GRBLinExpr,
}

impl Objective for QuadExpr {
    fn set_as_objective(
        self,
        model: &mut crate::prelude::GRBModel,
        sense: crate::prelude::GRBModelSense,
    ) {
        // set linear part
        self.linear_expr.set_as_objective(model, sense);
        // set quadratic part
        let len = self.quad_expr.len();
        let mut row = Vec::with_capacity(len);
        let mut col = Vec::with_capacity(len);
        let mut val = Vec::with_capacity(len);
        self.quad_expr
            .into_iter()
            .for_each(|((idx1, idx2), coeff)| {
                row.push(idx1 as i32);
                col.push(idx2 as i32);
                val.push(coeff);
            });
        let error = unsafe {
            ffi::GRBaddqpterms(
                *model.inner.0,
                len as std::ffi::c_int,
                row.as_mut_ptr(),
                col.as_mut_ptr(),
                val.as_mut_ptr(),
            )
        };
        model.get_error(error).unwrap();
    }
}

// OVERLOAD ADDITION
impl Add<f64> for QuadExpr {
    type Output = QuadExpr;

    fn add(self, scalar: f64) -> Self::Output {
        QuadExpr {
            quad_expr: self.quad_expr,
            linear_expr: GRBLinExpr {
                expr: self.linear_expr.expr,
                scalar: self.linear_expr.scalar + scalar,
            },
        }
    }
}

impl Add<QuadExpr> for QuadExpr {
    type Output = QuadExpr;
    fn add(mut self, rhs: QuadExpr) -> Self::Output {
        self.linear_expr += rhs.linear_expr;
        for (idxs, coeff) in rhs.quad_expr.iter() {
            match self.quad_expr.get_mut(idxs) {
                Some(existing_coeff) => {
                    *existing_coeff += coeff;
                }
                None => {
                    self.quad_expr.insert(*idxs, *coeff);
                }
            }
        }

        // return self
        self
    }
}

impl Add<&GRBVar> for QuadExpr {
    type Output = QuadExpr;

    fn add(mut self, var: &GRBVar) -> Self::Output {
        self.linear_expr += var;
        self
    }
}

impl Add<QuadExpr> for &GRBVar {
    type Output = QuadExpr;

    fn add(self, mut rhs: QuadExpr) -> Self::Output {
        rhs.linear_expr += self;
        rhs
    }
}

impl AddAssign<&GRBVar> for QuadExpr {
    fn add_assign(&mut self, rhs: &GRBVar) {
        self.linear_expr += rhs;
    }
}

// OVERLOAD SUBTRACTION

impl Sub<f64> for QuadExpr {
    type Output = QuadExpr;

    fn sub(self, scalar: f64) -> Self::Output {
        QuadExpr {
            quad_expr: self.quad_expr,
            linear_expr: GRBLinExpr {
                expr: self.linear_expr.expr,
                scalar: self.linear_expr.scalar - scalar,
            },
        }
    }
}

impl Sub<QuadExpr> for QuadExpr {
    type Output = QuadExpr;
    fn sub(mut self, rhs: QuadExpr) -> Self::Output {
        self.linear_expr -= rhs.linear_expr;
        for (idxs, coeff) in rhs.quad_expr.iter() {
            match self.quad_expr.get_mut(idxs) {
                Some(existing_coeff) => {
                    *existing_coeff -= coeff;
                }
                None => {
                    self.quad_expr.insert(*idxs, -*coeff);
                }
            }
        }
        self
    }
}

impl Sub<&GRBVar> for QuadExpr {
    type Output = QuadExpr;

    fn sub(mut self, var: &GRBVar) -> Self::Output {
        self.linear_expr -= var;
        self
    }
}

impl Sub<QuadExpr> for &GRBVar {
    type Output = QuadExpr;

    fn sub(self, mut rhs: QuadExpr) -> Self::Output {
        rhs.linear_expr -= self;
        rhs
    }
}

impl SubAssign<&GRBVar> for QuadExpr {
    fn sub_assign(&mut self, rhs: &GRBVar) {
        self.linear_expr -= rhs;
    }
}

// OVERLOAD MULTIPLICATION
impl Mul<f64> for QuadExpr {
    type Output = QuadExpr;

    fn mul(mut self, scalar: f64) -> Self::Output {
        // multiply coefficients
        for coeff in self.quad_expr.values_mut() {
            *coeff *= scalar;
        }
        self.linear_expr *= scalar;

        self
    }
}

impl Mul<QuadExpr> for f64 {
    type Output = QuadExpr;

    fn mul(self, expr: QuadExpr) -> Self::Output {
        expr * self
    }
}

impl Mul<GRBLinExpr> for GRBLinExpr {
    type Output = QuadExpr;

    fn mul(self, rhs: GRBLinExpr) -> Self::Output {
        // linear term can remain, bc of scalar mult
        let linear_expr = rhs.scalar * self.clone() + self.scalar * rhs.clone();
        let mut quad_expr = BTreeMap::new();
        // loop over all variables
        for (idx1, coeff1) in self.expr {
            for (idx2, coeff2) in rhs.expr.iter() {
                let key = (idx1, *idx2);
                let value = coeff1 * coeff2;
                quad_expr.insert(key, value);
            }
        }
        QuadExpr {
            quad_expr,
            linear_expr,
        }
    }
}

impl Mul<&GRBVar> for GRBLinExpr {
    type Output = QuadExpr;

    fn mul(self, var: &GRBVar) -> Self::Output {
        let mut linear_expr = self.scalar * GRBLinExpr::from(var);
        linear_expr.scalar = 0.0;
        let mut quad_expr = BTreeMap::new();
        for (idx, coeff) in self.expr {
            let key = (idx, var.index());
            let value = coeff;
            quad_expr.insert(key, value);
        }
        QuadExpr {
            quad_expr,
            linear_expr,
        }
    }
}

impl Mul<&GRBVar> for &GRBVar {
    type Output = QuadExpr;

    fn mul(self, rhs: &GRBVar) -> Self::Output {
        let quad_expr = BTreeMap::from([((self.index(), rhs.index()), 1.0)]);
        QuadExpr {
            quad_expr,
            linear_expr: GRBLinExpr {
                expr: BTreeMap::new(),
                scalar: 0.0,
            },
        }
    }
}
