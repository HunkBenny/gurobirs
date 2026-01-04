use std::{
    collections::BTreeMap,
    ops::{Add, AddAssign, Mul, Sub, SubAssign},
};

use crate::{modeling::expr::lin_expr::LinExpr, var::GRBVar};

pub struct QuadExpr {
    pub(crate) quad_expr: BTreeMap<(usize, usize), f64>, // (var_idx1, var_idx2, coeff)
    pub(crate) linear_expr: LinExpr,
}

// OVERLOAD ADDITION
impl Add<f64> for QuadExpr {
    type Output = QuadExpr;

    fn add(self, scalar: f64) -> Self::Output {
        QuadExpr {
            quad_expr: self.quad_expr,
            linear_expr: LinExpr {
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
            linear_expr: LinExpr {
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
                    self.quad_expr.insert(*idxs, *coeff);
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

impl Mul<LinExpr> for LinExpr {
    type Output = QuadExpr;

    fn mul(self, rhs: LinExpr) -> Self::Output {
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
