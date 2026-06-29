use std::{
    collections::BTreeMap,
    ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign},
};

use crate::{
    model::GRBModelSense,
    modeling::{IsModelingObject, Objective},
    prelude::GRBIntAttr,
    var::GRBVar,
};

use crate::ffi;

#[cfg_attr(debug_assertions, derive(Debug))]
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

    pub fn lin_terms(&self) -> impl Iterator<Item = (&usize, &f64)> {
        self.expr.iter()
    }

    pub fn scalar_term(&self) -> f64 {
        self.scalar
    }
}

impl Objective for GRBLinExpr {
    fn set_as_objective(self, model: &mut crate::prelude::GRBModel, sense: GRBModelSense) {
        // update model status first. push pending updates
        model.update();
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
        let num_vars = model.get(GRBIntAttr::NUMVARS);
        let mut coeffs = vec![0.0; num_vars as usize];

        for (var_idx, coeff) in self.expr {
            coeffs[var_idx] = coeff;
        }

        let error = unsafe {
            ffi::GRBsetdblattrarray(
                *model.inner.0,
                ffi::GRB_DBL_ATTR_OBJ.as_ptr(),
                0,
                coeffs.len() as std::ffi::c_int,
                coeffs.as_mut_ptr(),
            )
        };
        model.get_error(error).unwrap();

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
impl Add<GRBLinExpr> for GRBLinExpr {
    type Output = GRBLinExpr;
    // TODO: fix this to create a new linexpr
    fn add(mut self, rhs: GRBLinExpr) -> Self::Output {
        // 1. add scalar
        self.scalar += rhs.scalar;
        // 2. add expr to self, consuming the other linexpr
        for (var_idx, coeff) in rhs.expr.iter() {
            if coeff == &0.0 || coeff == &-0.0 {
                continue;
            }
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
            if coeff == &0.0 || coeff == &-0.0 {
                continue;
            }
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

// OVERLOAD MULTIPLICATION

macro_rules! impl_grblinexpr_math_ops {
    ($($t:ty),*) => {
        $(
            // -----------------------------------------
            // 1. Add (Owned and Borrowed)
            // -----------------------------------------

            // Owned
            impl Add<$t> for GRBLinExpr {
                type Output = GRBLinExpr;

                fn add(self, scalar: $t) -> Self::Output {
                    let scalar_f64 = f64::from(scalar);
                    GRBLinExpr {
                        expr: self.expr, // self is consumed, so we can just move expr
                        scalar: self.scalar + scalar_f64,
                    }
                }
            }

            impl Add<GRBLinExpr> for $t {
                type Output = GRBLinExpr;

                fn add(self, expr: GRBLinExpr) -> Self::Output {
                    expr + self
                }
            }

            // Borrowed
            impl Add<&$t> for GRBLinExpr {
                type Output = GRBLinExpr;

                fn add(self, scalar: &$t) -> Self::Output {
                    // Dereference the scalar before converting to f64
                    let scalar_f64 = f64::from(*scalar);
                    GRBLinExpr {
                        expr: self.expr,
                        scalar: self.scalar + scalar_f64,
                    }
                }
            }

            impl Add<GRBLinExpr> for &$t {
                type Output = GRBLinExpr;

                fn add(self, expr: GRBLinExpr) -> Self::Output {
                    expr + *self
                }
            }

            // -----------------------------------------
            // 2. AddAssign (Owned and Borrowed)
            // -----------------------------------------

            // Owned
            impl AddAssign<$t> for GRBLinExpr {
                fn add_assign(&mut self, scalar: $t) {
                    self.scalar += f64::from(scalar);
                }
            }

            // Borrowed
            impl AddAssign<&$t> for GRBLinExpr {
                fn add_assign(&mut self, scalar: &$t) {
                    // Dereference the scalar before adding
                    self.scalar += f64::from(*scalar);
                }
            }

            // -----------------------------------------
            // 3. Sub (Owned and Borrowed)
            // -----------------------------------------

            // Owned
            impl Sub<$t> for GRBLinExpr {
                type Output = GRBLinExpr;

                fn sub(self, scalar: $t) -> Self::Output {
                    let scalar_f64 = f64::from(scalar);
                    GRBLinExpr {
                        expr: self.expr, // self is consumed, so we can just move expr
                        scalar: self.scalar - scalar_f64,
                    }
                }
            }

            impl Sub<GRBLinExpr> for $t {
                type Output = GRBLinExpr;

                fn sub(self, expr: GRBLinExpr) -> Self::Output {
                    - 1.0 * expr + self
                 }
            }

            // Borrowed
            impl Sub<&$t> for GRBLinExpr {
                type Output = GRBLinExpr;

                fn sub(self, scalar: &$t) -> Self::Output {
                    // Dereference the scalar before converting to f64
                    let scalar_f64 = f64::from(*scalar);
                    GRBLinExpr {
                        expr: self.expr,
                        scalar: self.scalar - scalar_f64,
                    }
                }
            }

            impl Sub<GRBLinExpr> for &$t {
                type Output = GRBLinExpr;

                fn sub(self, expr: GRBLinExpr) -> Self::Output {
                    - 1.0 * expr + *self
                 }
            }

            // -----------------------------------------
            // 4. SubAssign (Owned and Borrowed)
            // -----------------------------------------
            // Owned
            impl SubAssign<$t> for GRBLinExpr {
                fn sub_assign(&mut self, scalar: $t) {
                    self.scalar -= f64::from(scalar);
                }
            }

            // Borrowed
            impl SubAssign<&$t> for GRBLinExpr {
                fn sub_assign(&mut self, scalar: &$t) {
                    // Dereference the scalar before adding
                    self.scalar -= f64::from(*scalar);
                }
            }

            // -----------------------------------------
            // 5. Mul (Owned and Borrowed)
            // -----------------------------------------
            // Owned
            impl Mul<$t> for GRBLinExpr {
                type Output = GRBLinExpr;

                fn mul(mut self, scalar: $t) -> Self::Output {
                    let scalar = f64::from(scalar);
                    if scalar == 0.0 || scalar == -0.0 {
                        return GRBLinExpr::new();
                    }
                    self.scalar *= scalar;

                    for (_var_idx, coeff) in self.expr.iter_mut() {
                        *coeff *= scalar;
                    }
                    self
                }
            }

            impl Mul<GRBLinExpr> for $t {
                type Output = GRBLinExpr;

                fn mul(mut self, expr: GRBLinExpr) -> Self::Output {
                    expr * self
                }
            }

            // Borrowed
            impl Mul<&$t> for GRBLinExpr {
                type Output = GRBLinExpr;

                fn mul(mut self, scalar: &$t) -> Self::Output {
                    let scalar = f64::from(*scalar);
                    if scalar == 0.0 || scalar == -0.0 {
                        return GRBLinExpr::new();
                    }
                    self.scalar *= scalar;

                    for (_var_idx, coeff) in self.expr.iter_mut() {
                        *coeff *= scalar;
                    }
                    self
                }
            }

            impl Mul<GRBLinExpr> for &$t {
                type Output = GRBLinExpr;

                fn mul(self, expr: GRBLinExpr) -> Self::Output {
                    expr * *self
                }
            }
            // -----------------------------------------
            // 6. MulAssign (Owned and Borrowed)
            // -----------------------------------------
            // Owned
            impl MulAssign<$t> for GRBLinExpr
            {
                fn mul_assign(&mut self, scalar: $t) {
                    let scalar = f64::from(scalar);
                    self.scalar *= scalar;

                    for (_var_idx, coeff) in self.expr.iter_mut() {
                        *coeff *= scalar;
                    }
                }
            }
            // Borrowed
            impl MulAssign<&$t> for GRBLinExpr
            {
                fn mul_assign(&mut self, scalar: &$t) {
                    let scalar = f64::from(*scalar);
                    self.scalar *= scalar;

                    for (_var_idx, coeff) in self.expr.iter_mut() {
                        *coeff *= scalar;
                    }
                }
            }
        )*
    };
}

// Generate the implementations!
impl_grblinexpr_math_ops!(i8, i16, i32, u8, u16, u32, f64);

macro_rules! impl_grbvar_math_ops {
    ($($t:ty),*) => {
        $(
            // 1. &GRBVar + &$t -> GRBLinExpr
            // This specifically satisfies your failing trait bound!
            impl std::ops::Add<&$t> for &GRBVar {
                type Output = GRBLinExpr;

                fn add(self, scalar: &$t) -> Self::Output {
                    // Convert the GRBVar into a GRBLinExpr here.
                    // Assuming self.expr is a collection of (Variable, Coefficient):
                    GRBLinExpr::from(self) + f64::from(*scalar)
                }
            }

            impl std::ops::Add<&GRBVar> for $t {
                type Output = GRBLinExpr;

                fn add(self, var: &GRBVar) -> Self::Output {
                    var + self
                }
            }

            // 2. &GRBVar + $t -> GRBLinExpr
            // For good measure, so you can do `&my_var + 42` directly
            impl std::ops::Add<$t> for &GRBVar {
                type Output = GRBLinExpr;

                fn add(self, scalar: $t) -> Self::Output {
                    GRBLinExpr::from(self) + f64::from(scalar)
                }
            }

            impl std::ops::Add<&GRBVar> for &$t {
                type Output = GRBLinExpr;

                fn add(self, var: &GRBVar) -> Self::Output {
                    var + *self
                }
            }

            // 3. &GRBVar * $t -> GRBLinExpr
            impl std::ops::Mul<$t> for &GRBVar
            {
                type Output = GRBLinExpr;

                fn mul(self, scalar: $t) -> Self::Output {
                    let scalar = f64::from(scalar);
                    if scalar == 0.0 || scalar == -0.0 {
                        return GRBLinExpr::new();
                    }
                    GRBLinExpr::from(self) * scalar
                }
            }

            impl std::ops::Mul<&GRBVar> for $t
            {
                type Output = GRBLinExpr;

                fn mul(self, var: &GRBVar) -> Self::Output {
                    var * self
                }
            }

            // 4. &GRBVar * $t -> GRBLinExpr
            impl std::ops::Mul<&$t> for &GRBVar
            {
                type Output = GRBLinExpr;

                fn mul(self, scalar: &$t) -> Self::Output {
                    let scalar = f64::from(*scalar);
                    if scalar == 0.0 || scalar == -0.0 {
                        return GRBLinExpr::new();
                    }
                    GRBLinExpr::from(self) * scalar
                }
            }

            impl std::ops::Mul<&GRBVar> for &$t
            {
                type Output = GRBLinExpr;

                fn mul(self, var: &GRBVar) -> Self::Output {
                    var * *self
                }
            }
        )*
    };
}

// Generate the implementations
impl_grbvar_math_ops!(i8, i16, i32, u8, u16, u32, f64);
