use std::{
    ffi::{c_char, CStr},
    ptr::{null, null_mut},
};

use crate::{
    constr::GRBConstr,
    env::GRBenv,
    error::check_err,
    ffi,
    modeling::{expr::lin_expr::LinExpr, CanBeAddedToModel, IsModelingObject},
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

    pub(crate) fn get_env(&self) -> *mut ffi::GRBenv {
        unsafe { ffi::GRBgetenv(self.inner()) }
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

    pub fn set<S: ModelSetter>(&mut self, what: S, value: S::Value) {
        let error = what.set(self, value);
        self.get_error(error).unwrap();
    }

    pub fn set_list<C, S>(&mut self, what: S, inds: Vec<C>, values: Vec<S::Value>)
    where
        C: IsModelingObject,
        S: ModelSetterList<C>,
    {
        let error = what.set_list(self, inds, values);
        self.get_error(error).unwrap();
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

#[allow(clippy::upper_case_acronyms, non_camel_case_types)]
pub enum GRBStatus {
    LOADED,
    OPTIMAL,
    INFEASIBLE,
    INF_OR_UNBD,
    UNBOUNDED,
    CUTOFF,
    ITERATION_LIMIT,
    NODE_LIMIT,
    TIME_LIMIT,
    SOLUTION_LIMIT,
    INTERRUPTED,
    NUMERIC,
    SUBOPTIMAL,
    INPROGRESS,
    USER_OBJ_LIMIT,
    WORK_LIMIT,
    MEM_LIMIT,
    LOCALLY_OPTIMAL,
    LOCALLY_INFEASIBLE,
}

impl From<GRBStatus> for std::ffi::c_int {
    fn from(value: GRBStatus) -> Self {
        match value {
            GRBStatus::LOADED => ffi::GRB_LOADED,
            GRBStatus::OPTIMAL => ffi::GRB_OPTIMAL,
            GRBStatus::INFEASIBLE => ffi::GRB_INFEASIBLE,
            GRBStatus::INF_OR_UNBD => ffi::GRB_INF_OR_UNBD,
            GRBStatus::UNBOUNDED => ffi::GRB_UNBOUNDED,
            GRBStatus::CUTOFF => ffi::GRB_CUTOFF,
            GRBStatus::ITERATION_LIMIT => ffi::GRB_ITERATION_LIMIT,
            GRBStatus::NODE_LIMIT => ffi::GRB_NODE_LIMIT,
            GRBStatus::TIME_LIMIT => ffi::GRB_TIME_LIMIT,
            GRBStatus::SOLUTION_LIMIT => ffi::GRB_SOLUTION_LIMIT,
            GRBStatus::INTERRUPTED => ffi::GRB_INTERRUPTED,
            GRBStatus::NUMERIC => ffi::GRB_NUMERIC,
            GRBStatus::SUBOPTIMAL => ffi::GRB_SUBOPTIMAL,
            GRBStatus::INPROGRESS => ffi::GRB_INPROGRESS,
            GRBStatus::USER_OBJ_LIMIT => ffi::GRB_USER_OBJ_LIMIT,
            GRBStatus::WORK_LIMIT => ffi::GRB_WORK_LIMIT,
            GRBStatus::MEM_LIMIT => ffi::GRB_MEM_LIMIT,
            GRBStatus::LOCALLY_OPTIMAL => ffi::GRB_LOCALLY_OPTIMAL,
            GRBStatus::LOCALLY_INFEASIBLE => ffi::GRB_LOCALLY_INFEASIBLE,
        }
    }
}

pub trait ModelGetter {
    type Value;
    fn get(&self, model: &GRBModel) -> Self::Value;
}

pub trait ModelGetterList<C>
where
    C: IsModelingObject,
{
    type Value;
    fn get_list(&self, model: &GRBModel, inds: Vec<C>) -> Vec<Self::Value>;
}

// trait used to set model attributes and parameters
pub trait ModelSetter {
    type Value;
    fn set(&self, model: &mut GRBModel, value: Self::Value) -> i32;
}

pub trait ModelSetterList<C>
where
    C: IsModelingObject,
{
    type Value;
    fn set_list(&self, model: &mut GRBModel, inds: Vec<C>, values: Vec<Self::Value>) -> i32;
}

// TODO: setters

// TODO: getters
