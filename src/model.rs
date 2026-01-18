use std::{
    ffi::{c_char, CStr, CString},
    ptr::{null, null_mut},
    rc::Rc,
};

use crate::{
    constr::{GRBConstr, TempConstr, TempQConstr},
    env::GRBenv,
    error::check_err,
    ffi,
    modeling::{
        expr::nonlin_expr::GRBOpCode, AddAsIndicator, CanBeAddedToModel, IsModelingObject,
        Objective,
    },
    prelude::GRBVarBuilder,
    var::GRBVar,
};

pub struct GRBModelPtr(pub(crate) Rc<*mut ffi::GRBmodel>);

impl Drop for GRBModelPtr {
    fn drop(&mut self) {
        // if more than one reference, do not free
        if Rc::strong_count(&self.0) > 1 {
            return;
        }
        // if last reference, free model
        unsafe {
            ffi::GRBfreemodel(*self.0);
        }
    }
}

impl Clone for GRBModelPtr {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

pub struct GRBModel {
    pub(crate) inner: GRBModelPtr,
    var_index: usize,
    cons_index: usize,
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
            inner: GRBModelPtr(Rc::new(model)),
            var_index: 0,
            cons_index: 0,
        }
    }

    pub(crate) fn get_env(&self) -> *mut ffi::GRBenv {
        unsafe { ffi::GRBgetenv(*self.inner.0) }
    }

    pub fn add_var(&mut self, mut var: GRBVarBuilder) -> GRBVar {
        // TODO: Does this need to be a pinned box?
        let name = var.get_name();
        let name_ptr = match name {
            Some(ref s) => s.as_ptr(),
            None => null_mut(),
        };
        // add to model
        let error = var.add_to_model(*self.inner.0, name_ptr);
        self.get_error(error).unwrap();
        // create GRBVar Rust-object
        let var = GRBVar::new(self.var_index, self.inner(), name);
        self.var_index += 1;
        var
    }

    pub fn inner(&self) -> GRBModelPtr {
        self.inner.clone()
    }

    pub fn add_constr(&mut self, mut expr: TempConstr) -> GRBConstr {
        let name = expr.get_name();
        let name_ptr = match name {
            Some(ref s) => s.as_ptr(),
            None => null_mut(),
        };
        let error = expr.add_to_model(*self.inner.0, name_ptr);
        self.get_error(error).unwrap();
        let constr = GRBConstr {
            index: self.cons_index,
            inner: self.inner.clone(),
        };
        self.cons_index += 1;
        constr
    }

    pub fn add_qconstr(&mut self, mut expr: TempQConstr) -> GRBConstr {
        let name = expr.get_name();
        let name_ptr = match name {
            Some(ref s) => s.as_ptr(),
            None => null_mut(),
        };
        let error = expr.add_to_model(*self.inner.0, name_ptr);
        self.get_error(error).unwrap();
        let constr = GRBConstr {
            index: self.cons_index,
            inner: self.inner.clone(),
        };
        self.cons_index += 1;
        constr
    }

    pub fn add_genconstr_max(
        &mut self,
        res_var: GRBVar,
        xvars: Vec<GRBVar>,
        constant: f64,
        name: &str,
    ) -> GRBConstr {
        let name = CString::new(name).unwrap();
        let name = name.as_ptr() as *const std::ffi::c_char;
        let len = xvars.len();
        let xvars = xvars
            .iter()
            .map(|x| x.index() as std::ffi::c_int)
            .collect::<Vec<_>>();
        let error = unsafe {
            ffi::GRBaddgenconstrMax(
                *self.inner.0,
                name,
                res_var.index() as std::ffi::c_int,
                len as std::ffi::c_int,
                xvars.as_ptr(),
                constant,
            )
        };
        self.get_error(error).unwrap();

        let cons = GRBConstr {
            index: self.cons_index,
            inner: self.inner(),
        };
        self.cons_index += 1;
        cons
    }

    pub fn add_genconstr_min(
        &mut self,
        res_var: GRBVar,
        xvars: Vec<GRBVar>,
        constant: f64,
        name: &str,
    ) -> GRBConstr {
        // name
        let name = CString::new(name).unwrap();
        let name = name.as_ptr() as *const std::ffi::c_char;
        let len = xvars.len();
        let xvars = xvars
            .iter()
            .map(|x| x.index() as std::ffi::c_int)
            .collect::<Vec<_>>();
        let error = unsafe {
            ffi::GRBaddgenconstrMin(
                *self.inner.0,
                name,
                res_var.index() as std::ffi::c_int,
                len as std::ffi::c_int,
                xvars.as_ptr(),
                constant,
            )
        };
        self.get_error(error).unwrap();

        let cons = GRBConstr {
            index: self.cons_index,
            inner: self.inner(),
        };
        self.cons_index += 1;
        cons
    }

    pub fn add_genconstr_abs(&mut self, res_var: GRBVar, arg_var: GRBVar, name: &str) -> GRBConstr {
        let name = CString::new(name).unwrap();
        let name = name.as_ptr();
        let error = unsafe {
            ffi::GRBaddgenconstrAbs(
                *self.inner.0,
                name,
                res_var.index() as std::ffi::c_int,
                arg_var.index() as std::ffi::c_int,
            )
        };
        self.get_error(error);
        let cons = GRBConstr {
            index: self.cons_index,
            inner: self.inner(),
        };
        self.cons_index += 1;
        cons
    }

    pub fn add_genconstr_and(
        &mut self,
        res_var: GRBVar,
        xvars: Vec<GRBVar>,
        name: &str,
    ) -> GRBConstr {
        let name = CString::new(name).unwrap();
        let name = name.as_ptr();
        let xvars = xvars
            .iter()
            .map(|x| x.index() as std::ffi::c_int)
            .collect::<Vec<_>>();
        let len = xvars.len();
        let error = unsafe {
            ffi::GRBaddgenconstrAnd(
                *self.inner.0,
                name,
                res_var.index() as std::ffi::c_int,
                len as std::ffi::c_int,
                xvars.as_ptr(),
            )
        };
        self.get_error(error).unwrap();
        let cons = GRBConstr {
            index: self.cons_index,
            inner: self.inner(),
        };
        self.cons_index += 1;
        cons
    }

    pub fn add_genconstr_or(
        &mut self,
        res_var: GRBVar,
        xvars: Vec<GRBVar>,
        name: &str,
    ) -> GRBConstr {
        let name = CString::new(name).unwrap();
        let name = name.as_ptr();
        let xvars = xvars
            .iter()
            .map(|x| x.index() as std::ffi::c_int)
            .collect::<Vec<_>>();
        let len = xvars.len();
        let error = unsafe {
            ffi::GRBaddgenconstrOr(
                *self.inner.0,
                name,
                res_var.index() as std::ffi::c_int,
                len as std::ffi::c_int,
                xvars.as_ptr(),
            )
        };
        self.get_error(error).unwrap();
        let cons = GRBConstr {
            index: self.cons_index,
            inner: self.inner(),
        };
        self.cons_index += 1;
        cons
    }

    pub fn add_genconstr_norm(
        &mut self,
        res_var: GRBVar,
        xvars: Vec<GRBVar>,
        which: f64,
        name: &str,
    ) -> GRBConstr {
        let name = CString::new(name).unwrap();
        let name = name.as_ptr();
        let xvars = xvars
            .iter()
            .map(|x| x.index() as std::ffi::c_int)
            .collect::<Vec<_>>();
        let len = xvars.len();
        let error = unsafe {
            ffi::GRBaddgenconstrNorm(
                *self.inner.0,
                name,
                res_var.index() as std::ffi::c_int,
                len as std::ffi::c_int,
                xvars.as_ptr(),
                which as std::ffi::c_double,
            )
        };
        self.get_error(error).unwrap();
        let cons = GRBConstr {
            index: self.cons_index,
            inner: self.inner(),
        };
        self.cons_index += 1;
        cons
    }

    pub fn add_genconstr_indicator(
        &mut self,
        binvar: GRBVar,
        binval: i8,
        mut constr: TempConstr,
    ) -> GRBConstr {
        let name = constr.get_name();
        let name_ptr = match name {
            Some(ref s) => s.as_ptr(),
            None => null_mut(),
        };
        let error = constr.add_as_indicator(*self.inner.0, binvar, binval, name_ptr);
        self.get_error(error).unwrap();
        let cons = GRBConstr {
            index: self.cons_index,
            inner: self.inner(),
        };
        self.cons_index += 1;
        cons
    }

    pub fn add_genconstr_pwl(
        &mut self,
        xvar: GRBVar,
        yvar: GRBVar,
        npts: i32,
        xpts: Vec<f64>,
        ypts: Vec<f64>,
        name: &str,
    ) -> GRBConstr {
        let name = CString::new(name).unwrap();
        let name = name.as_ptr();
        let xpts = xpts
            .iter()
            .map(|x| *x as std::ffi::c_double)
            .collect::<Vec<_>>();
        let ypts = ypts
            .iter()
            .map(|y| *y as std::ffi::c_double)
            .collect::<Vec<_>>();
        let error = unsafe {
            ffi::GRBaddgenconstrPWL(
                *self.inner.0,
                name,
                xvar.index() as std::ffi::c_int,
                yvar.index() as std::ffi::c_int,
                npts,
                xpts.as_ptr(),
                ypts.as_ptr(),
            )
        };
        self.get_error(error).unwrap();
        let cons = GRBConstr {
            index: self.cons_index,
            inner: self.inner(),
        };
        self.cons_index += 1;
        cons
    }

    pub fn add_genconstr_nl(
        &mut self,
        res_var: GRBVar,
        opcodes: Vec<GRBOpCode>,
        data: Vec<f64>,
        parent: Vec<i32>,
        name: &str,
    ) -> GRBConstr {
        let name = CString::new(name).unwrap();
        let name = name.as_ptr();
        let len = opcodes.len();
        let opcode = opcodes
            .iter()
            .map(|&x| x as std::ffi::c_int)
            .collect::<Vec<_>>();
        let data = data
            .iter()
            .map(|&x| x as std::ffi::c_double)
            .collect::<Vec<_>>();
        let parent = parent
            .iter()
            .map(|&x| x as std::ffi::c_int)
            .collect::<Vec<_>>();

        let error = unsafe {
            ffi::GRBaddgenconstrNL(
                *self.inner.0,
                name,
                res_var.index() as std::ffi::c_int,
                len as std::ffi::c_int,
                opcode.as_ptr(),
                data.as_ptr(),
                parent.as_ptr(),
            )
        };
        self.get_error(error).unwrap();
        let cons = GRBConstr {
            index: self.cons_index,
            inner: self.inner(),
        };
        self.cons_index += 1;
        cons
    }

    pub fn set_objective<O: Objective>(&mut self, obj: O, sense: GRBModelSense) {
        obj.set_as_objective(self, sense);
    }

    pub fn optimize(&mut self) {
        let error = unsafe { ffi::GRBoptimize(*self.inner.0) };
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
                    CStr::from_ptr(ffi::GRBgetmerrormsg(*self.inner.0) as *mut c_char)
                        .to_string_lossy()
                ))
            },
            Ok(_o) => Ok(()),
        }
    }

    pub fn set<S: ModelSetter>(&mut self, what: S, value: S::Value) {
        let error = what.set(*self.inner.0, value);
        self.get_error(error).unwrap();
    }

    pub fn set_list<C, S>(&mut self, what: S, inds: Vec<C>, values: Vec<S::Value>)
    where
        C: IsModelingObject,
        S: ModelSetterList<C>,
    {
        let error = what.set_list(*self.inner.0, inds, values);
        self.get_error(error).unwrap();
    }

    pub fn get<G: ModelGetter>(&self, what: G) -> G::Value {
        what.get(*self.inner.0)
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
    fn get(&self, model: *mut ffi::GRBmodel) -> Self::Value;
}

pub trait ModelGetterList<C>
where
    C: IsModelingObject,
{
    type Value;
    fn get_list(&self, model: *mut ffi::GRBmodel, inds: Vec<C>) -> Vec<Self::Value>;
}

// trait used to set model attributes and parameters
pub trait ModelSetter {
    type Value;
    fn set(&self, model: *mut ffi::GRBmodel, value: Self::Value) -> i32;
}
// trait used to set model attributes and parameters
pub trait EnvSetter {
    type Value;
    fn set(&self, env: *mut ffi::GRBenv, value: Self::Value) -> i32;
}
// implement env setter for all modelsetters! We can access the env from the model
impl<E: EnvSetter> ModelSetter for E {
    type Value = E::Value;

    fn set(&self, model: *mut ffi::GRBmodel, value: Self::Value) -> i32 {
        // get env
        let env_ptr = unsafe { ffi::GRBgetenv(model) };
        // call set on env
        self.set(env_ptr, value)
    }
}

pub trait ModelSetterList<C>
where
    C: IsModelingObject,
{
    type Value;
    fn set_list(&self, model: *mut ffi::GRBmodel, inds: Vec<C>, values: Vec<Self::Value>) -> i32;
}

// TODO: setters

// TODO: getters
