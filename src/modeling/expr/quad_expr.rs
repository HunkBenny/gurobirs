use std::{
    collections::BTreeMap,
    ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign},
};

use crate::{ffi, modeling::IsModelingObject};
use crate::{
    modeling::{expr::lin_expr::GRBLinExpr, Objective},
    var::GRBVar,
};

pub struct GRBQuadExpr {
    pub(crate) quad_expr: BTreeMap<(usize, usize), f64>, // (var_idx1, var_idx2, coeff)
    pub(crate) linear_expr: GRBLinExpr,
}

impl GRBQuadExpr {
    pub fn new() -> Self {
        GRBQuadExpr {
            quad_expr: BTreeMap::new(),
            linear_expr: GRBLinExpr::new(),
        }
    }
}

impl Objective for GRBQuadExpr {
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

impl Add<GRBLinExpr> for GRBQuadExpr {
    type Output = GRBQuadExpr;

    fn add(mut self, lin_expr: GRBLinExpr) -> Self::Output {
        self.linear_expr += lin_expr;
        self
    }
}

impl Add<GRBQuadExpr> for GRBQuadExpr {
    type Output = GRBQuadExpr;
    fn add(mut self, rhs: GRBQuadExpr) -> Self::Output {
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

impl Add<&GRBVar> for GRBQuadExpr {
    type Output = GRBQuadExpr;

    fn add(mut self, var: &GRBVar) -> Self::Output {
        self.linear_expr += var;
        self
    }
}

impl Add<GRBQuadExpr> for &GRBVar {
    type Output = GRBQuadExpr;

    fn add(self, mut rhs: GRBQuadExpr) -> Self::Output {
        rhs.linear_expr += self;
        rhs
    }
}

impl AddAssign<&GRBVar> for GRBQuadExpr {
    fn add_assign(&mut self, rhs: &GRBVar) {
        self.linear_expr += rhs;
    }
}

impl AddAssign<GRBQuadExpr> for GRBQuadExpr {
    fn add_assign(&mut self, rhs: GRBQuadExpr) {
        // 1. add linear part (logic implemented in linexpr)
        self.linear_expr += rhs.linear_expr;
        // 2. add quadr part, looping over rhs quad expr and adding to self
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
    }
}

// OVERLOAD SUBTRACTION

impl Sub<GRBQuadExpr> for GRBQuadExpr {
    type Output = GRBQuadExpr;
    fn sub(mut self, rhs: GRBQuadExpr) -> Self::Output {
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

impl Sub<&GRBVar> for GRBQuadExpr {
    type Output = GRBQuadExpr;

    fn sub(mut self, var: &GRBVar) -> Self::Output {
        self.linear_expr -= var;
        self
    }
}

impl Sub<GRBQuadExpr> for &GRBVar {
    type Output = GRBQuadExpr;

    fn sub(self, mut rhs: GRBQuadExpr) -> Self::Output {
        rhs.linear_expr -= self;
        rhs
    }
}

impl SubAssign<&GRBVar> for GRBQuadExpr {
    fn sub_assign(&mut self, rhs: &GRBVar) {
        self.linear_expr -= rhs;
    }
}

impl SubAssign<GRBLinExpr> for GRBQuadExpr {
    fn sub_assign(&mut self, rhs: GRBLinExpr) {
        self.linear_expr -= rhs;
    }
}

// OVERLOAD MULTIPLICATION

impl Mul<GRBLinExpr> for GRBLinExpr {
    type Output = GRBQuadExpr;

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
        GRBQuadExpr {
            quad_expr,
            linear_expr,
        }
    }
}

impl Mul<&GRBVar> for GRBLinExpr {
    type Output = GRBQuadExpr;

    fn mul(self, var: &GRBVar) -> Self::Output {
        GRBLinExpr::from(var) * self
    }
}

impl Mul<&GRBVar> for &GRBVar {
    type Output = GRBQuadExpr;

    fn mul(self, rhs: &GRBVar) -> Self::Output {
        GRBLinExpr::from(self) * GRBLinExpr::from(rhs)
    }
}

macro_rules! impl_grbquadexpr_math_ops {
    ($($t:ty),*) => {
        $(
            // -----------------------------------------
            // 1. Add (Owned and Borrowed)
            // -----------------------------------------

            // Owned
            impl Add<$t> for GRBQuadExpr {
                type Output = GRBQuadExpr;

                fn add(self, scalar: $t) -> Self::Output {
                    let scalar = f64::from(scalar);
                    GRBQuadExpr {
                        quad_expr: self.quad_expr,
                        linear_expr: GRBLinExpr {
                        expr: self.linear_expr.expr,
                        scalar: self.linear_expr.scalar + scalar,
                        },
                    }
                }
            }

            impl Add<GRBQuadExpr> for $t {
                type Output = GRBQuadExpr;

                fn add(self, expr: GRBQuadExpr) -> Self::Output {
                    expr + self
                }
            }

            // Borrowed
            impl Add<&$t> for GRBQuadExpr {
                type Output = GRBQuadExpr;

                fn add(self, scalar: &$t) -> Self::Output {
                    // Dereference the scalar before converting to f64
                    let scalar = f64::from(*scalar);
                    GRBQuadExpr {
                        quad_expr: self.quad_expr,
                        linear_expr: GRBLinExpr {
                        expr: self.linear_expr.expr,
                        scalar: self.linear_expr.scalar + scalar,
                        },
                    }
                }
            }

            impl Add<GRBQuadExpr> for &$t {
                type Output = GRBQuadExpr;

                fn add(self, expr: GRBQuadExpr) -> Self::Output {
                    expr + self
                }
            }

            // -----------------------------------------
            // 2. AddAssign (Owned and Borrowed)
            // -----------------------------------------

            // Owned
            impl AddAssign<$t> for GRBQuadExpr {
                fn add_assign(&mut self, scalar: $t) {
                    self.linear_expr += f64::from(scalar);
                }
            }

            // Borrowed
            impl AddAssign<&$t> for GRBQuadExpr {
                fn add_assign(&mut self, scalar: &$t) {
                    // Dereference the scalar before adding
                    self.linear_expr += f64::from(*scalar);
                }
            }

            // -----------------------------------------
            // 3. Sub (Owned and Borrowed)
            // -----------------------------------------

            // Owned
            impl Sub<$t> for GRBQuadExpr {
                type Output = GRBQuadExpr;

                fn sub(self, scalar: $t) -> Self::Output {
                    let scalar_f64 = f64::from(scalar);
                    GRBQuadExpr {
                        quad_expr: self.quad_expr,
                        linear_expr: GRBLinExpr {
                            expr: self.linear_expr.expr,
                            scalar: self.linear_expr.scalar - scalar_f64,
                        },
                    }
                }
            }

            impl Sub<GRBQuadExpr> for $t {
                type Output = GRBQuadExpr;

                fn sub(self, expr: GRBQuadExpr) -> Self::Output {
                    -1.0 * expr + self
                }
            }

            // Borrowed
            impl Sub<&$t> for GRBQuadExpr {
                type Output = GRBQuadExpr;

                fn sub(self, scalar: &$t) -> Self::Output {
                    // Dereference the scalar before converting to f64
                    let scalar_f64 = f64::from(*scalar);
                    GRBQuadExpr {
                        quad_expr: self.quad_expr,
                        linear_expr: GRBLinExpr {
                            expr: self.linear_expr.expr,
                            scalar: self.linear_expr.scalar - scalar_f64,
                        },
                    }
                }
            }

            impl Sub<GRBQuadExpr> for &$t {
                type Output = GRBQuadExpr;

                fn sub(self, expr: GRBQuadExpr) -> Self::Output {
                    -1.0 * expr + self
                }
            }

            // -----------------------------------------
            // 4. SubAssign (Owned and Borrowed)
            // -----------------------------------------
            // Owned
            impl SubAssign<$t> for GRBQuadExpr {
                fn sub_assign(&mut self, scalar: $t) {
                    self.linear_expr -= f64::from(scalar);
                }
            }

            // Borrowed
            impl SubAssign<&$t> for GRBQuadExpr {
                fn sub_assign(&mut self, scalar: &$t) {
                    // Dereference the scalar before adding
                    self.linear_expr -= f64::from(*scalar);
                }
            }

            // -----------------------------------------
            // 5. Mul (Owned and Borrowed)
            // -----------------------------------------
            // Owned
            impl Mul<$t> for GRBQuadExpr {
                type Output = GRBQuadExpr;

                fn mul(mut self, scalar: $t) -> Self::Output {
                    let scalar = f64::from(scalar);
                    if scalar == 0.0 || scalar == -0.0 {
                        return GRBQuadExpr::new();
                    }
                    // multiply coefficients
                    for coeff in self.quad_expr.values_mut() {
                        *coeff *= scalar;
                    }
                    self.linear_expr *= scalar;

                    self
                }
            }

            impl Mul<GRBQuadExpr> for $t {
                type Output = GRBQuadExpr;

                fn mul(self, expr: GRBQuadExpr) -> Self::Output {
                    expr * self
                }
            }

            // Borrowed
            impl Mul<&$t> for GRBQuadExpr {
                type Output = GRBQuadExpr;

                fn mul(mut self, scalar: &$t) -> Self::Output {
                    let scalar = f64::from(*scalar);
                    if scalar == 0.0 || scalar == -0.0 {
                        return GRBQuadExpr::new();
                    }
                    // multiply coefficients
                    for coeff in self.quad_expr.values_mut() {
                        *coeff *= scalar;
                    }
                    self.linear_expr *= scalar;

                    self
                }
            }

            impl Mul<GRBQuadExpr> for &$t {
                type Output = GRBQuadExpr;

                fn mul(self, expr: GRBQuadExpr) -> Self::Output {
                    expr * self
                }
            }

            // -----------------------------------------
            // 6. MulAssign (Owned and Borrowed)
            // -----------------------------------------
            // Owned
            impl MulAssign<$t> for GRBQuadExpr
            {
                fn mul_assign(&mut self, scalar: $t) {
                    let scalar = f64::from(scalar);
                    if scalar == 0.0 || scalar == -0.0 {
                        *self = GRBQuadExpr::new();
                        return;
                    }

                    self.linear_expr *= scalar;

                    for (_var_idx, coeff) in self.quad_expr.iter_mut() {
                        *coeff *= scalar;
                    }
                }
            }
            // Borrowed
            impl MulAssign<&$t> for GRBQuadExpr
            {
                fn mul_assign(&mut self, scalar: &$t) {
                    let scalar = f64::from(*scalar);
                    if scalar == 0.0 || scalar == -0.0 {
                        *self = GRBQuadExpr::new();
                        return;
                    }

                    self.linear_expr *= scalar;

                    for (_var_idx, coeff) in self.quad_expr.iter_mut() {
                        *coeff *= scalar;
                    }
                }
            }
        )*
    };
}
// TODO: use macros like lin_expr.rs
impl_grbquadexpr_math_ops!(i8, i16, i32, u8, u16, u32, f64);
