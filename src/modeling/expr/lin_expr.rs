use std::{
    collections::BTreeMap,
    ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign},
};

use crate::var::GRBVar;

pub struct LinExpr {
    /// Tree of (variable index, coefficient) pairs
    /// Tree, because if two LinExpr are added together and have an overlap in variables, the
    /// coefficients need to be summed. So there needs to be an efficient way to look up variable indices.
    ///
    /// NOTE: Even though this will probably not happen often, this has no impact on the solving of
    /// the model, only on the construction of it.
    pub(crate) expr: BTreeMap<usize, f64>,
    /// The constant term
    pub(crate) scalar: f64,
}

// impl add, mult, sub etc
impl Add<f64> for LinExpr {
    type Output = LinExpr;

    fn add(self, scalar: f64) -> Self::Output {
        LinExpr {
            expr: self.expr,
            scalar: self.scalar + scalar,
        }
    }
}

impl AddAssign<f64> for LinExpr {
    fn add_assign(&mut self, scalar: f64) {
        self.scalar += scalar;
    }
}

impl Add<LinExpr> for LinExpr {
    type Output = LinExpr;
    // TODO: fix this to create a new linexpr
    fn add(mut self, rhs: LinExpr) -> Self::Output {
        // 1. add scalar
        self.scalar += rhs.scalar;
        // 2. add expr to self, consuming the other linexpr
        for (var_idx, coeff) in rhs.expr.iter() {
            match self.expr.get_mut(var_idx) {
                Some(existing_coeff) => {
                    *existing_coeff += coeff;
                }
                None => {
                    self.expr.insert(*var_idx, *coeff);
                }
            }
        }
        // return self
        self
    }
}

impl AddAssign<LinExpr> for LinExpr {
    fn add_assign(&mut self, rhs: LinExpr) {
        // 1. add scalar
        self.scalar += rhs.scalar;
        // 2. add expr to self, consuming the other linexpr
        for (var_idx, coeff) in rhs.expr.iter() {
            match self.expr.get_mut(var_idx) {
                Some(existing_coeff) => {
                    *existing_coeff += coeff;
                }
                None => {
                    self.expr.insert(*var_idx, *coeff);
                }
            }
        }
    }
}

impl Sub<f64> for LinExpr {
    type Output = LinExpr;

    fn sub(self, scalar: f64) -> Self::Output {
        LinExpr {
            expr: self.expr,
            scalar: self.scalar - scalar,
        }
    }
}

impl Sub<LinExpr> for f64 {
    type Output = LinExpr;

    fn sub(self, expr: LinExpr) -> Self::Output {
        expr - self
    }
}

impl SubAssign<f64> for LinExpr {
    fn sub_assign(&mut self, scalar: f64) {
        self.scalar -= scalar;
    }
}

impl Sub<LinExpr> for LinExpr {
    type Output = LinExpr;
    //TODO: fix this to create a new linexpr
    fn sub(mut self, rhs: LinExpr) -> Self::Output {
        // 1. add scalar
        self.scalar -= rhs.scalar;
        // 2. add expr to self, consuming the other linexpr
        for (var_idx, coeff) in rhs.expr.iter() {
            match self.expr.get_mut(var_idx) {
                Some(existing_coeff) => {
                    *existing_coeff -= coeff;
                }
                None => {
                    // neg coeff
                    self.expr.insert(*var_idx, -*coeff);
                }
            }
        }
        // return self
        self
    }
}

impl SubAssign<LinExpr> for LinExpr {
    fn sub_assign(&mut self, rhs: LinExpr) {
        // 1. add scalar
        self.scalar -= rhs.scalar;
        // 2. add expr to self, consuming the other linexpr
        for (var_idx, coeff) in rhs.expr.iter() {
            match self.expr.get_mut(var_idx) {
                Some(existing_coeff) => {
                    *existing_coeff -= coeff;
                }
                None => {
                    self.expr.insert(*var_idx, -*coeff);
                }
            }
        }
    }
}

// NOTE: multiplication only makes sense with scalars for linear expressions

impl Mul<f64> for LinExpr {
    type Output = LinExpr;

    fn mul(mut self, scalar: f64) -> Self::Output {
        self.scalar *= scalar;

        for (_var_idx, coeff) in self.expr.iter_mut() {
            *coeff *= scalar;
        }
        self
    }
}

impl MulAssign<f64> for LinExpr {
    fn mul_assign(&mut self, scalar: f64) {
        self.scalar *= scalar;

        for (_var_idx, coeff) in self.expr.iter_mut() {
            *coeff *= scalar;
        }
    }
}

// NOTE: OPERATOR OVERLOADING FOR GRBVar:
// Create possibility to make LinExpr from GRBvar;

impl From<GRBVar> for LinExpr {
    fn from(value: GRBVar) -> Self {
        let mut expr = BTreeMap::new();
        expr.insert(value.index(), 1.0);
        LinExpr { expr, scalar: 0.0 }
    }
}

// OVERLOAD ADDITION
impl Add<GRBVar> for LinExpr {
    type Output = LinExpr;

    fn add(self, var: GRBVar) -> Self::Output {
        self + LinExpr::from(var)
    }
}

impl Add<LinExpr> for GRBVar {
    type Output = LinExpr;

    fn add(self, expr: LinExpr) -> Self::Output {
        expr + self
    }
}

impl AddAssign<GRBVar> for LinExpr {
    fn add_assign(&mut self, var: GRBVar) {
        *self += LinExpr::from(var);
    }
}

impl Add<GRBVar> for f64 {
    type Output = LinExpr;

    fn add(self, var: GRBVar) -> Self::Output {
        LinExpr::from(var) + self
    }
}

impl Add<f64> for GRBVar {
    type Output = LinExpr;

    fn add(self, scalar: f64) -> Self::Output {
        LinExpr::from(self) + scalar
    }
}

impl Add<GRBVar> for GRBVar {
    type Output = LinExpr;

    fn add(self, rhs: GRBVar) -> Self::Output {
        self + LinExpr::from(rhs)
    }
}

// OVERLOAD SUBTRACTION
impl Sub<GRBVar> for LinExpr {
    type Output = LinExpr;

    fn sub(self, var: GRBVar) -> Self::Output {
        self - LinExpr::from(var)
    }
}

impl Sub<LinExpr> for GRBVar {
    type Output = LinExpr;

    fn sub(self, expr: LinExpr) -> Self::Output {
        expr - self
    }
}

impl SubAssign<GRBVar> for LinExpr {
    fn sub_assign(&mut self, var: GRBVar) {
        *self -= LinExpr::from(var);
    }
}

impl Sub<GRBVar> for f64 {
    type Output = LinExpr;

    fn sub(self, var: GRBVar) -> Self::Output {
        self - LinExpr::from(var)
    }
}

impl Sub<f64> for GRBVar {
    type Output = LinExpr;

    fn sub(self, scalar: f64) -> Self::Output {
        LinExpr::from(self) - scalar
    }
}

// OVERLOAD MULTIPLICATION
impl Mul<f64> for GRBVar {
    type Output = LinExpr;

    fn mul(self, scalar: f64) -> Self::Output {
        LinExpr::from(self) * scalar
    }
}

impl Mul<GRBVar> for f64 {
    type Output = LinExpr;

    fn mul(self, var: GRBVar) -> Self::Output {
        var * self
    }
}
