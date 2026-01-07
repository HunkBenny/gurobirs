use std::{
    collections::BTreeMap,
    ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign},
};

use crate::{
    model::GRBModelSense,
    modeling::{IsModelingObject, Objective},
    var::GRBVar,
};

use crate::ffi;

#[derive(Clone)]
pub struct GRBLinExpr {
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

impl GRBLinExpr {
    pub fn new() -> Self {
        GRBLinExpr {
            expr: BTreeMap::new(),
            scalar: 0.0,
        }
    }
}

impl Objective for GRBLinExpr {
    fn set_as_objective(self, model: &mut crate::prelude::GRBModel, sense: GRBModelSense) {
        // set constant term
        let constant_term = self.scalar;

        let error = unsafe {
            ffi::GRBsetdblattr(
                *model.inner.0,
                ffi::GRB_DBL_ATTR_OBJCON.as_ptr(),
                constant_term,
            )
        };
        model.get_error(error).unwrap();
        // set coeffs
        for (var_idx, coeff) in self.expr {
            let error = unsafe {
                ffi::GRBsetdblattrelement(
                    *model.inner.0,
                    ffi::GRB_DBL_ATTR_OBJ.as_ptr(),
                    var_idx as i32,
                    coeff,
                )
            };
            model.get_error(error).unwrap();
        }

        // Set model sense
        let error = unsafe {
            ffi::GRBsetintattr(
                *model.inner.0,
                ffi::GRB_INT_ATTR_MODELSENSE.as_ptr(),
                GRBModelSense::get(sense),
            )
        };
        model.get_error(error).unwrap();
    }
}

// impl add, mult, sub etc
impl Add<f64> for GRBLinExpr {
    type Output = GRBLinExpr;

    fn add(self, scalar: f64) -> Self::Output {
        GRBLinExpr {
            expr: self.expr,
            scalar: self.scalar + scalar,
        }
    }
}

impl AddAssign<f64> for GRBLinExpr {
    fn add_assign(&mut self, scalar: f64) {
        self.scalar += scalar;
    }
}

impl Add<GRBLinExpr> for GRBLinExpr {
    type Output = GRBLinExpr;
    // TODO: fix this to create a new linexpr
    fn add(mut self, rhs: GRBLinExpr) -> Self::Output {
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

impl AddAssign<GRBLinExpr> for GRBLinExpr {
    fn add_assign(&mut self, rhs: GRBLinExpr) {
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

impl Sub<f64> for GRBLinExpr {
    type Output = GRBLinExpr;

    fn sub(self, scalar: f64) -> Self::Output {
        GRBLinExpr {
            expr: self.expr,
            scalar: self.scalar - scalar,
        }
    }
}

impl Sub<GRBLinExpr> for f64 {
    type Output = GRBLinExpr;

    fn sub(self, expr: GRBLinExpr) -> Self::Output {
        expr - self
    }
}

impl SubAssign<f64> for GRBLinExpr {
    fn sub_assign(&mut self, scalar: f64) {
        self.scalar -= scalar;
    }
}

impl Sub<GRBLinExpr> for GRBLinExpr {
    type Output = GRBLinExpr;
    //TODO: fix this to create a new linexpr
    fn sub(mut self, rhs: GRBLinExpr) -> Self::Output {
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

impl SubAssign<GRBLinExpr> for GRBLinExpr {
    fn sub_assign(&mut self, rhs: GRBLinExpr) {
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

impl Mul<f64> for GRBLinExpr {
    type Output = GRBLinExpr;

    fn mul(mut self, scalar: f64) -> Self::Output {
        self.scalar *= scalar;

        for (_var_idx, coeff) in self.expr.iter_mut() {
            *coeff *= scalar;
        }
        self
    }
}

impl Mul<GRBLinExpr> for f64 {
    type Output = GRBLinExpr;

    fn mul(self, expr: GRBLinExpr) -> Self::Output {
        expr * self
    }
}

impl MulAssign<f64> for GRBLinExpr {
    fn mul_assign(&mut self, scalar: f64) {
        self.scalar *= scalar;

        for (_var_idx, coeff) in self.expr.iter_mut() {
            *coeff *= scalar;
        }
    }
}

// NOTE: OPERATOR OVERLOADING FOR GRBVar:
// Create possibility to make LinExpr from GRBvar;

impl From<&GRBVar> for GRBLinExpr {
    fn from(value: &GRBVar) -> Self {
        let mut expr = BTreeMap::new();
        expr.insert(value.index(), 1.0);
        GRBLinExpr { expr, scalar: 0.0 }
    }
}

// OVERLOAD ADDITION
impl Add<&GRBVar> for GRBLinExpr {
    type Output = GRBLinExpr;

    fn add(self, var: &GRBVar) -> Self::Output {
        self + GRBLinExpr::from(var)
    }
}

impl Add<GRBLinExpr> for &GRBVar {
    type Output = GRBLinExpr;

    fn add(self, expr: GRBLinExpr) -> Self::Output {
        expr + self
    }
}

impl AddAssign<&GRBVar> for GRBLinExpr {
    fn add_assign(&mut self, var: &GRBVar) {
        *self += GRBLinExpr::from(var);
    }
}

impl Add<&GRBVar> for f64 {
    type Output = GRBLinExpr;

    fn add(self, var: &GRBVar) -> Self::Output {
        GRBLinExpr::from(var) + self
    }
}

impl Add<f64> for &GRBVar {
    type Output = GRBLinExpr;

    fn add(self, scalar: f64) -> Self::Output {
        GRBLinExpr::from(self) + scalar
    }
}

impl Add<&GRBVar> for &GRBVar {
    type Output = GRBLinExpr;

    fn add(self, rhs: &GRBVar) -> Self::Output {
        rhs + GRBLinExpr::from(self)
    }
}

// OVERLOAD SUBTRACTION
impl Sub<&GRBVar> for GRBLinExpr {
    type Output = GRBLinExpr;

    fn sub(self, var: &GRBVar) -> Self::Output {
        self - GRBLinExpr::from(var)
    }
}

impl Sub<GRBLinExpr> for &GRBVar {
    type Output = GRBLinExpr;

    fn sub(self, expr: GRBLinExpr) -> Self::Output {
        expr - self
    }
}

impl SubAssign<&GRBVar> for GRBLinExpr {
    fn sub_assign(&mut self, var: &GRBVar) {
        *self -= GRBLinExpr::from(var);
    }
}

impl Sub<&GRBVar> for f64 {
    type Output = GRBLinExpr;

    fn sub(self, var: &GRBVar) -> Self::Output {
        self - GRBLinExpr::from(var)
    }
}

impl Sub<f64> for &GRBVar {
    type Output = GRBLinExpr;

    fn sub(self, scalar: f64) -> Self::Output {
        GRBLinExpr::from(self) - scalar
    }
}

// OVERLOAD MULTIPLICATION
impl Mul<f64> for &GRBVar {
    type Output = GRBLinExpr;

    fn mul(self, scalar: f64) -> Self::Output {
        GRBLinExpr::from(self) * scalar
    }
}

impl Mul<&GRBVar> for f64 {
    type Output = GRBLinExpr;

    fn mul(self, var: &GRBVar) -> Self::Output {
        var * self
    }
}
