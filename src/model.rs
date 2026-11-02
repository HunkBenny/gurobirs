use std::{
    cell::Cell,
    ffi::{c_char, CStr, CString},
    ptr::{null, null_mut},
};

use crate::{
    attributes::GRBIntAttr,
    constr::{GRBConstr, TempConstr, TempQConstr},
    env::GRBEnv,
    error::check_err,
    ffi,
    modeling::{
        expr::nonlin_expr::GRBOpCode, AddAsIndicator, CanBeAddedToModel, IsModelingObject,
        Objective,
    },
    prelude::GRBVarBuilder,
    var::GRBVar,
};

#[cfg_attr(debug_assertions, derive(Debug))]
pub(crate) struct GRBModelInner {
    pub(crate) model: *mut ffi::GRBmodel,
}

impl Drop for GRBModelInner {
    fn drop(&mut self) {
        unsafe {
            ffi::GRBfreemodel(self.model);
        }
    }
}

pub struct GRBModel {
    pub(crate) inner: Box<GRBModelInner>,
    var_index: Cell<usize>,
    rows_index: Cell<usize>,
    qconstraints_index: Cell<usize>,
    genconstrs_index: Cell<usize>,
}

impl GRBModel {
    pub fn new_from_file(env: &GRBEnv, filename: &str) -> GRBModel {
        let filename = CString::new(filename).unwrap();

        let mut model = null_mut();

        let error =
            unsafe { ffi::GRBreadmodel(env.inner(), filename.as_ptr(), &mut model) };
        env.get_error(error).unwrap();

        let mut model = GRBModel {
            inner: Box::new(GRBModelInner { model }),
            var_index: Cell::new(0),
            rows_index: Cell::new(0),
            qconstraints_index: Cell::new(0),
            genconstrs_index: Cell::new(0),
        };
        // update attributes
        let var_index = model.get(GRBIntAttr::NUMVARS);
        let rows_index = model.get(GRBIntAttr::NUMCONSTRS);
        let qconstr_index = model.get(GRBIntAttr::NUMQCONSTRS);
        let genconstr_index = model.get(GRBIntAttr::NUMGENCONSTRS);
        model.var_index.set(var_index as usize);
        model.rows_index.set(rows_index as usize);
        model.qconstraints_index.set(qconstr_index as usize);
        model.genconstrs_index.set(genconstr_index as usize);

        model
    }

    pub fn update(&self) {
        let error = unsafe { ffi::GRBupdatemodel(self.inner.model) };
        self.get_error(error).unwrap();
    }

    pub fn read(&mut self, filename: &str) {
        let c_filename = CString::new(filename).unwrap();
        let error = unsafe { ffi::GRBread(self.inner.model, c_filename.as_ptr()) };
        self.get_error(error).unwrap();
    }

    pub fn fixed_model(&self) -> GRBModel {
        let model_ptr = unsafe { ffi::GRBfixedmodel(self.inner.model) };
        let mut model = GRBModel {
            inner: Box::new(GRBModelInner { model: model_ptr }),
            var_index: Cell::new(0),
            rows_index: Cell::new(0),
            qconstraints_index: Cell::new(0),
            genconstrs_index: Cell::new(0),
        };
        let var_index = model.get(GRBIntAttr::NUMVARS);
        let rows_index = model.get(GRBIntAttr::NUMCONSTRS);
        let qconstr_index = model.get(GRBIntAttr::NUMQCONSTRS);
        let genconstr_index = model.get(GRBIntAttr::NUMGENCONSTRS);
        model.var_index.set(var_index as usize);
        model.rows_index.set(rows_index as usize);
        model.qconstraints_index.set(qconstr_index as usize);
        model.genconstrs_index.set(genconstr_index as usize);
        model
    }

    pub fn write(&self, filename: &str) {
        let c_filename = CString::new(filename).unwrap();
        let error = unsafe { ffi::GRBwrite(self.inner.model, c_filename.as_ptr()) };
        self.get_error(error).unwrap();
    }

    pub fn new(env: &GRBEnv) -> GRBModel {
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
        GRBModel {
            inner: Box::new(GRBModelInner { model }),
            var_index: Cell::new(0),
            rows_index: Cell::new(0),
            qconstraints_index: Cell::new(0),
            genconstrs_index: Cell::new(0),
        }
    }

    pub fn get_env(&self) -> *mut ffi::GRBenv {
        unsafe { ffi::GRBgetenv(self.inner.model) }
    }

    pub fn add_var(&self, mut var: GRBVarBuilder) -> GRBVar {
        let name = var.get_name();
        let name_ptr = match name {
            Some(ref s) => s.as_ptr(),
            None => null_mut(),
        };
        let error = var.add_to_model(self.inner.model, name_ptr);
        self.get_error(error).unwrap();
        let inner: &'static GRBModelInner = unsafe { &*(self.inner.as_ref() as *const GRBModelInner) };
        let var = GRBVar::new(self.var_index.get(), inner);
        self.var_index.set(self.var_index.get() + 1);
        var
    }

    pub fn add_constr(&self, mut expr: TempConstr) -> GRBConstr {
        let name = expr.get_name();
        let name_ptr = match name {
            Some(ref s) => s.as_ptr(),
            None => null_mut(),
        };
        let error = expr.add_to_model(self.inner.model, name_ptr);
        self.get_error(error).unwrap();
        let inner: &'static GRBModelInner = unsafe { &*(self.inner.as_ref() as *const GRBModelInner) };
        let constr = GRBConstr {
            index: self.rows_index.get(),
            inner,
        };
        self.rows_index.set(self.rows_index.get() + 1);
        constr
    }

    pub fn add_qconstr(&self, mut expr: TempQConstr) -> GRBConstr {
        let name = expr.get_name();
        let name_ptr = match name {
            Some(ref s) => s.as_ptr(),
            None => null_mut(),
        };
        let error = expr.add_to_model(self.inner.model, name_ptr);
        self.get_error(error).unwrap();
        let inner: &'static GRBModelInner = unsafe { &*(self.inner.as_ref() as *const GRBModelInner) };
        let constr = GRBConstr {
            index: self.qconstraints_index.get(),
            inner,
        };
        self.qconstraints_index.set(self.qconstraints_index.get() + 1);
        constr
    }

    pub fn add_genconstr_max(
        &self,
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
                self.inner.model,
                name,
                res_var.index() as std::ffi::c_int,
                len as std::ffi::c_int,
                xvars.as_ptr(),
                constant,
            )
        };
        self.get_error(error).unwrap();

        let inner: &'static GRBModelInner = unsafe { &*(self.inner.as_ref() as *const GRBModelInner) };
        let cons = GRBConstr {
            index: self.genconstrs_index.get(),
            inner,
        };
        self.genconstrs_index.set(self.genconstrs_index.get() + 1);
        cons
    }

    pub fn add_genconstr_min(
        &self,
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
            ffi::GRBaddgenconstrMin(
                self.inner.model,
                name,
                res_var.index() as std::ffi::c_int,
                len as std::ffi::c_int,
                xvars.as_ptr(),
                constant,
            )
        };
        self.get_error(error).unwrap();

        let inner: &'static GRBModelInner = unsafe { &*(self.inner.as_ref() as *const GRBModelInner) };
        let cons = GRBConstr {
            index: self.genconstrs_index.get(),
            inner,
        };
        self.genconstrs_index.set(self.genconstrs_index.get() + 1);
        cons
    }

    pub fn add_genconstr_abs(
        &self,
        res_var: GRBVar,
        arg_var: GRBVar,
        name: &str,
    ) -> GRBConstr {
        let name = CString::new(name).unwrap();
        let name = name.as_ptr();
        let error = unsafe {
            ffi::GRBaddgenconstrAbs(
                self.inner.model,
                name,
                res_var.index() as std::ffi::c_int,
                arg_var.index() as std::ffi::c_int,
            )
        };
        self.get_error(error).unwrap();
        let inner: &'static GRBModelInner = unsafe { &*(self.inner.as_ref() as *const GRBModelInner) };
        let cons = GRBConstr {
            index: self.genconstrs_index.get(),
            inner,
        };
        self.genconstrs_index.set(self.genconstrs_index.get() + 1);
        cons
    }

    pub fn add_genconstr_and(
        &self,
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
                self.inner.model,
                name,
                res_var.index() as std::ffi::c_int,
                len as std::ffi::c_int,
                xvars.as_ptr(),
            )
        };
        self.get_error(error).unwrap();
        let inner: &'static GRBModelInner = unsafe { &*(self.inner.as_ref() as *const GRBModelInner) };
        let cons = GRBConstr {
            index: self.genconstrs_index.get(),
            inner,
        };
        self.genconstrs_index.set(self.genconstrs_index.get() + 1);
        cons
    }

    pub fn add_genconstr_or(
        &self,
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
                self.inner.model,
                name,
                res_var.index() as std::ffi::c_int,
                len as std::ffi::c_int,
                xvars.as_ptr(),
            )
        };
        self.get_error(error).unwrap();
        let inner: &'static GRBModelInner = unsafe { &*(self.inner.as_ref() as *const GRBModelInner) };
        let cons = GRBConstr {
            index: self.genconstrs_index.get(),
            inner,
        };
        self.genconstrs_index.set(self.genconstrs_index.get() + 1);
        cons
    }

    pub fn add_genconstr_norm(
        &self,
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
                self.inner.model,
                name,
                res_var.index() as std::ffi::c_int,
                len as std::ffi::c_int,
                xvars.as_ptr(),
                which as std::ffi::c_double,
            )
        };
        self.get_error(error).unwrap();
        let inner: &'static GRBModelInner = unsafe { &*(self.inner.as_ref() as *const GRBModelInner) };
        let cons = GRBConstr {
            index: self.genconstrs_index.get(),
            inner,
        };
        self.genconstrs_index.set(self.genconstrs_index.get() + 1);
        cons
    }

    pub fn add_genconstr_indicator(
        &self,
        binvar: GRBVar,
        binval: i8,
        mut constr: TempConstr,
    ) -> GRBConstr {
        let name = constr.get_name();
        let name_ptr = match name {
            Some(ref s) => s.as_ptr(),
            None => null_mut(),
        };
        let error = constr.add_as_indicator(self.inner.model, binvar, binval, name_ptr);
        self.get_error(error).unwrap();
        let inner: &'static GRBModelInner = unsafe { &*(self.inner.as_ref() as *const GRBModelInner) };
        let cons = GRBConstr {
            index: self.genconstrs_index.get(),
            inner,
        };
        self.genconstrs_index.set(self.genconstrs_index.get() + 1);
        cons
    }

    pub fn add_genconstr_pwl(
        &self,
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
                self.inner.model,
                name,
                xvar.index() as std::ffi::c_int,
                yvar.index() as std::ffi::c_int,
                npts,
                xpts.as_ptr(),
                ypts.as_ptr(),
            )
        };
        self.get_error(error).unwrap();
        let inner: &'static GRBModelInner = unsafe { &*(self.inner.as_ref() as *const GRBModelInner) };
        let cons = GRBConstr {
            index: self.genconstrs_index.get(),
            inner,
        };
        self.genconstrs_index.set(self.genconstrs_index.get() + 1);
        cons
    }

    pub fn add_genconstr_nl(
        &self,
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
                self.inner.model,
                name,
                res_var.index() as std::ffi::c_int,
                len as std::ffi::c_int,
                opcode.as_ptr(),
                data.as_ptr(),
                parent.as_ptr(),
            )
        };
        self.get_error(error).unwrap();
        let inner: &'static GRBModelInner = unsafe { &*(self.inner.as_ref() as *const GRBModelInner) };
        let cons = GRBConstr {
            index: self.genconstrs_index.get(),
            inner,
        };
        self.genconstrs_index.set(self.genconstrs_index.get() + 1);
        cons
    }

    pub fn set_objective<O: Objective>(&self, obj: O, sense: GRBModelSense) {
        obj.set_as_objective(self, sense);
    }

    pub fn set_objective_n<O: Objective>(
        &self,
        obj: O,
        index: i32,
        priority: i32,
        weight: f64,
        abstol: f64,
        reltol: f64,
    ) {
        obj.set_as_objective_n(self, index, priority, weight, abstol, reltol);
    }

    pub fn optimize(&self) {
        let error = unsafe { ffi::GRBoptimize(self.inner.model) };
        match self.get_error(error) {
            Ok(_) => (),
            Err(e) => {
                panic!("{}", e);
            }
        }
    }

    pub fn reset(&self) {
        let error = unsafe { ffi::GRBreset(self.inner.model, 0) };
        self.get_error(error).unwrap();
    }

    pub fn get_error(&self, error_code: i32) -> Result<(), String> {
        match check_err(error_code) {
            Err(e) => unsafe {
                Err(format!(
                    "ERROR CODE {}: {}",
                    e,
                    CStr::from_ptr(ffi::GRBgetmerrormsg(self.inner.model) as *mut c_char)
                        .to_string_lossy()
                ))
            },
            Ok(_o) => Ok(()),
        }
    }

    pub fn set<S: ModelSetter>(&self, what: S, value: S::Value) {
        let error = what.set(self.inner.model, value);
        self.get_error(error).unwrap();
    }

    pub fn set_list<C, S>(&self, what: S, inds: Vec<C>, values: Vec<S::Value>)
    where
        C: IsModelingObject,
        S: ModelSetterList<C>,
    {
        let error = what.set_list(self.inner.model, inds, values);
        self.get_error(error).unwrap();
    }

    pub fn get_list<C, G>(&self, what: G, inds: &Vec<C>) -> Vec<G::Value>
    where
        C: IsModelingObject,
        G: ModelGetterList<C>,
    {
        what.get_list(self.inner.model, inds)
    }

    pub fn get<G: ModelGetter>(&self, what: G) -> G::Value {
        what.get(self.inner.model)
    }
}

#[repr(i32)]
pub enum GRBModelSense {
    MAXIMIZE = -1,
    MINIMIZE = 1,
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

#[repr(C)]
#[allow(clippy::upper_case_acronyms, non_camel_case_types)]
#[derive(Clone, Copy)]
pub enum GRBStatus {
    LOADED = ffi::GRB_LOADED as isize,
    OPTIMAL = ffi::GRB_OPTIMAL as isize,
    INFEASIBLE = ffi::GRB_INFEASIBLE as isize,
    INF_OR_UNBD = ffi::GRB_INF_OR_UNBD as isize,
    UNBOUNDED = ffi::GRB_UNBOUNDED as isize,
    CUTOFF = ffi::GRB_CUTOFF as isize,
    ITERATION_LIMIT = ffi::GRB_ITERATION_LIMIT as isize,
    NODE_LIMIT = ffi::GRB_NODE_LIMIT as isize,
    TIME_LIMIT = ffi::GRB_TIME_LIMIT as isize,
    SOLUTION_LIMIT = ffi::GRB_SOLUTION_LIMIT as isize,
    INTERRUPTED = ffi::GRB_INTERRUPTED as isize,
    NUMERIC = ffi::GRB_NUMERIC as isize,
    SUBOPTIMAL = ffi::GRB_SUBOPTIMAL as isize,
    INPROGRESS = ffi::GRB_INPROGRESS as isize,
    USER_OBJ_LIMIT = ffi::GRB_USER_OBJ_LIMIT as isize,
    WORK_LIMIT = ffi::GRB_WORK_LIMIT as isize,
    MEM_LIMIT = ffi::GRB_MEM_LIMIT as isize,
    LOCALLY_OPTIMAL = ffi::GRB_LOCALLY_OPTIMAL as isize,
    LOCALLY_INFEASIBLE = ffi::GRB_LOCALLY_INFEASIBLE as isize,
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
    fn get_list(&self, model: *mut ffi::GRBmodel, inds: &Vec<C>) -> Vec<Self::Value>;
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
