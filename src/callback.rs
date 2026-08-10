// The plan should be as follows:
// 1. user writes a Rust function (`my_callback_fn`).
// 2. user uses `model.set_callback(my_callback_fn)` to register the callback.
// 3. internally, we create a C-compatible function pointer that calls `my_callback_fn`.
//    This way, we can guarantee that we don't continuously call a callback_function.
use crate::error::check_err;
use crate::ffi;
use crate::model::GRBModel;
use crate::modeling::{CanBeAddedToCallback, IsModelingObject};
use crate::var::GRBVar;
use std::ffi::{c_char, CStr};
use std::fmt::Display;
use std::panic::{catch_unwind, AssertUnwindSafe};

/// A struct that represents the context of a Gurobi callback. It contains the model, callback data, and the type of callback being executed.
/// * `model`: pointer to the Gurobi model
/// * `cb_data`: data used by gurobi internally for callback
/// * `where_`: location where callback is called
/// * `solution`: optional vector of solution values for the variables at the current callback context (if not None, then we load solution in)
pub struct GRBCallbackContext {
    pub(crate) model: *mut ffi::GRBmodel,
    pub(crate) cb_data: *mut std::ffi::c_void,
    pub where_: GRBCallbackCodes,
    // TODO: setsolution needs to be added
    pub(crate) solution: Option<Vec<f64>>,
}

pub struct GRBCallback<C: CallbackTrait> {
    callback: C,
}

pub trait CallbackTrait {
    fn callback(&mut self, cb_ctx: &mut GRBCallbackContext);
}

impl<C: CallbackTrait> GRBCallback<C> {
    pub fn new(callback: C) -> Self {
        Self { callback }
    }

    pub fn inner(self) -> C {
        self.callback
    }
}

unsafe extern "C" fn c_shim<C: CallbackTrait>(
    model: *mut ffi::GRBmodel,
    cb_data: *mut std::ffi::c_void,
    where_: std::ffi::c_int,
    user_data: *mut std::ffi::c_void,
) -> i32 {
    let wrapper = user_data as *mut GRBCallback<C>;
    let mut cb_ctx = GRBCallbackContext {
        model,
        cb_data,
        where_: where_.into(),
        solution: None,
    };
    let result = catch_unwind(AssertUnwindSafe(|| unsafe {
        (*wrapper).callback.callback(&mut cb_ctx)
    }));

    if let Some(mut solution) = cb_ctx.solution {
        unsafe {
            let mut _objval_p = 0.0;
            let error = ffi::GRBcbsolution(
                cb_data,
                solution.as_mut_ptr() as *mut std::ffi::c_double,
                &mut _objval_p as *mut std::ffi::c_double,
            );
            check_err(error).unwrap();
        }
    }

    match result {
        Ok(_) => 0,
        Err(_) => ffi::GRB_ERROR_CALLBACK,
    }
}

impl GRBModel {
    pub fn set_callback<C: CallbackTrait>(
        &mut self,
        callback: &mut GRBCallback<C>,
        wheres: Option<u32>,
    ) {
        let error = unsafe {
            if let Some(wheres) = wheres {
                // better for performance
                ffi::GRBsetcallbackfuncadv(
                    *self.inner.0,
                    Some(c_shim::<C>),
                    callback as *mut _ as *mut std::ffi::c_void,
                    wheres as std::ffi::c_uint,
                )
            } else {
                // default
                ffi::GRBsetcallbackfunc(
                    *self.inner.0,
                    Some(c_shim::<C>),
                    callback as *mut _ as *mut std::ffi::c_void,
                )
            }
        };
        check_err(error).unwrap();
    }
}

impl GRBCallbackContext {
    /// Get the raw callback data pointer
    /// # Safety
    /// This function is unsafe because it exposes the raw callback data pointer
    pub unsafe fn get_cbdata(&self) -> *mut std::ffi::c_void {
        self.cb_data
    }

    /// Get the raw model pointer
    ///
    /// # Safety
    /// This function is unsafe because it exposes the raw model pointer, which can lead to undefined behavior
    pub unsafe fn get_model(&self) -> *mut ffi::GRBmodel {
        self.model
    }

    /// Get model error
    pub fn get_merror(&self, error_code: i32) -> Result<(), String> {
        match check_err(error_code) {
            Err(e) => unsafe {
                Err(format!(
                    "ERROR CODE {}: {}",
                    e,
                    CStr::from_ptr(ffi::GRBgetmerrormsg(self.model) as *mut c_char)
                        .to_string_lossy()
                ))
            },
            Ok(_o) => Ok(()),
        }
    }

    /// Get env error
    pub fn get_error(&self, error_code: i32) -> Result<(), String> {
        match check_err(error_code) {
            Err(e) => unsafe {
                let env = ffi::GRBgetenv(self.model);
                Err(format!(
                    "ERROR CODE {}: {}",
                    e,
                    CStr::from_ptr(ffi::GRBgeterrormsg(env) as *mut c_char).to_string_lossy()
                ))
            },
            Ok(_o) => Ok(()),
        }
    }

    pub fn proceed(&self) {
        let error = unsafe { ffi::GRBcbproceed(self.cb_data) };
        self.get_merror(error).unwrap();
    }

    pub fn abort(&self) {
        unsafe {
            ffi::GRBterminate(self.model);
        }
    }

    pub fn add_cut<E: CanBeAddedToCallback>(&mut self, expr: E) {
        let error = expr.add_cut(self);
        self.get_merror(error).unwrap();
    }

    pub fn add_lazy<E: CanBeAddedToCallback>(&mut self, expr: E) {
        let error = expr.add_lazy(self);
        self.get_error(error).unwrap();
    }

    pub fn get_info<G: CallbackGet>(
        &mut self,
        what: G,
    ) -> Result<G::Output, Box<dyn std::error::Error>> {
        what.get(self)
    }

    /// Get the solution values for the given variables at the current callback context.
    /// This function is rather expensive rn, maybe should add caching?
    /// Caching can be done by checking how many times the callback has been called and only updating
    /// the variable values if the callback is called in a new context.
    pub fn get_solutions<T: GetSolution>(&mut self, variables: T) -> T::Output {
        let num_vars = self.get_nvars();
        let mut values: Vec<f64> = vec![0.0; num_vars as usize];
        let what = match self.where_ {
            GRBCallbackCodes::MIPSOL => ffi::GRB_CB_MIPSOL_SOL,
            GRBCallbackCodes::MULTIOBJ => ffi::GRB_CB_MULTIOBJ_SOL,
            _ => panic!(
                "Cannot get solutions for this callback type (where needs to be MIPSOL or MULTIOBJ), it was {}",
                self.where_
            ),
        };
        let error = unsafe {
            ffi::GRBcbget(
                self.cb_data,
                self.where_ as std::ffi::c_int,
                what,
                values.as_mut_ptr() as *mut std::ffi::c_void,
            )
        };
        self.get_merror(error).unwrap();
        // now extract the values for the requested variables
        variables.get_solution(&values)
    }

    pub fn set_solutions<V: SetSolution>(&mut self, vars: V, val: V::Value) {
        if self.solution.is_none() {
            let num_vars = self.get_nvars();
            self.solution = Some(vec![ffi::GRB_UNDEFINED; num_vars as usize])
        }
        if let Some(solution) = self.solution.as_mut() {
            vars.set_solution(solution, val);
        }
    }

    pub fn get_noderels(&mut self, variables: Vec<GRBVar>) -> Vec<f64> {
        match self.where_.into() {
            GRBCallbackCodes::MIPNODE => {}
            _ => {
                panic!(
                    "Cannot get noderels for this callback type (where needs to be MIPNODE), it was {}",
                    self.where_
                );
            }
        }

        // check if optimally solved
        let optimally_solved = self
            .get_info(GRB_WHAT_INT::MIPNODE_STATUS)
            .expect("Failed to get MIPNODE_STATUS");
        if optimally_solved != ffi::GRB_OPTIMAL {
            panic!(
                "Cannot get noderels if the node is not optimally solved, status was {}",
                optimally_solved
            );
        }

        let num_vars = self.get_nvars();
        let mut values: Vec<f64> = vec![0.0; num_vars as usize];

        let error = unsafe {
            ffi::GRBcbget(
                self.cb_data,
                self.where_ as std::ffi::c_int,
                ffi::GRB_CB_MIPNODE_REL,
                values.as_mut_ptr() as *mut std::ffi::c_void,
            )
        };
        self.get_merror(error).unwrap();
        // now extract the values for the requested variables
        let mut return_values = Vec::with_capacity(variables.len());
        for var in &variables {
            return_values.push(values[var.index()]);
        }
        return_values
    }

    pub fn get_nvars(&mut self) -> i32 {
        let mut num_vars = 0;
        let error = unsafe {
            ffi::GRBgetintattr(
                self.model,
                ffi::GRB_INT_ATTR_NUMVARS.as_ptr(),
                &mut num_vars as *mut i32,
            )
        };
        self.get_merror(error).unwrap();
        num_vars
    }
}

#[allow(clippy::upper_case_acronyms, non_camel_case_types)]
#[derive(Copy, Clone)]
pub enum GRB_WHAT_DOUBLE {
    RUNTIME,
    WORK,
    MEMUSED,
    MAXMEMUSED,
    SPX_ITRCNT,
    SPX_OBJVAL,
    SPX_PRIMINF,
    SPX_DUALINF,
    MIP_OBJBST,
    MIP_OBJBND,
    MIP_NODCNT,
    MIP_NODLFT,
    MIP_ITRCNT,
    MIPSOL_OBJ,
    MIPSOL_OBJBST,
    MIPSOL_OBJBND,
    MIPSOL_NODCNT,
    MIPNODE_OBJBST,
    MIPNODE_OBJBND,
    MIPNODE_NODCNT,
    BARRIER_PRIMOBJ,
    BARRIER_DUALOBJ,
    BARRIER_PRIMINF,
    BARRIER_DUALINF,
    BARRIER_COMPL,
    MULTIOBJ_OBJBST,
    MULTIOBJ_OBJBND,
    MULTIOBJ_MIPGAP,
    MULTIOBJ_ITRCNT,
    MULTIOBJ_NODCNT,
    MULTIOBJ_NODLFT,
    MULTIOBJ_RUNTIME,
    MULTIOBJ_WORK,
    PDHG_PRIMOBJ,
    PDHG_DUALOBJ,
    PDHG_PRIMINF,
    PDHG_DUALINF,
    PDHG_COMPL,
    NLBAR_PRIMOBJ,
    NLBAR_PRIMINF,
    NLBAR_DUALINF,
    NLBAR_COMPL,
}

#[allow(clippy::upper_case_acronyms, non_camel_case_types)]
#[derive(Copy, Clone)]
pub enum GRB_WHAT_INT {
    PRE_COLDEL,
    PRE_ROWDEL,
    PRE_SENCHG,
    PRE_BNDCHG,
    PRE_COECHG,
    SPX_ISPERT,
    MIP_SOLCNT,
    MIP_CUTCNT,
    MIP_OPENSCENARIOS,
    MIP_PHASE,
    MIPSOL_SOLCNT,
    MIPSOL_OPENSCENARIOS,
    MIPSOL_PHASE,
    MIPNODE_STATUS,
    MIPNODE_SOLCNT,
    MIPNODE_OPENSCENARIOS,
    MIPNODE_PHASE,
    BARRIER_ITRCNT,
    MULTIOBJ_OBJCNT,
    MULTIOBJ_SOLCNT,
    MULTIOBJ_STATUS,
    IIS_CONSTRMIN,
    IIS_CONSTRMAX,
    IIS_CONSTRGUESS,
    IIS_BOUNDMIN,
    IIS_BOUNDMAX,
    IIS_BOUNDGUESS,
    PDHG_ITRCNT,
    NLBAR_ITRCNT,
}

#[allow(clippy::upper_case_acronyms, non_camel_case_types)]
#[derive(Copy, Clone)]
pub enum GRB_WHAT_STRING {
    MSG_STRING,
}

#[allow(clippy::upper_case_acronyms, non_camel_case_types)]
#[derive(Copy, Clone)]
#[repr(i32)]
pub enum GRBCallbackCodes {
    POLLING = ffi::GRB_CB_POLLING,
    NLBAR = ffi::GRB_CB_NLBAR,
    PDHG = ffi::GRB_CB_PDHG,
    IIS = ffi::GRB_CB_IIS,
    MULTIOBJ = ffi::GRB_CB_MULTIOBJ,
    BARRIER = ffi::GRB_CB_BARRIER,
    MESSAGE = ffi::GRB_CB_MESSAGE,
    MIPNODE = ffi::GRB_CB_MIPNODE,
    MIPSOL = ffi::GRB_CB_MIPSOL,
    MIP = ffi::GRB_CB_MIP,
    SIMPLEX = ffi::GRB_CB_SIMPLEX,
    PRESOLVE = ffi::GRB_CB_PRESOLVE,
}

impl Display for GRBCallbackCodes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                GRBCallbackCodes::POLLING => "POLLING",
                GRBCallbackCodes::NLBAR => "NLBAR",
                GRBCallbackCodes::PDHG => "PDHG",
                GRBCallbackCodes::IIS => "IIS",
                GRBCallbackCodes::MULTIOBJ => "MULTIOBJ",
                GRBCallbackCodes::BARRIER => "BARRIER",
                GRBCallbackCodes::MESSAGE => "MESSAGE",
                GRBCallbackCodes::MIPNODE => "MIPNODE",
                GRBCallbackCodes::MIPSOL => "MIPSOL",
                GRBCallbackCodes::MIP => "MIP",
                GRBCallbackCodes::SIMPLEX => "SIMPLEX",
                GRBCallbackCodes::PRESOLVE => "PRESOLVE",
            }
        )
    }
}

impl From<std::ffi::c_int> for GRBCallbackCodes {
    fn from(where_: std::ffi::c_int) -> Self {
        match where_ {
            ffi::GRB_CB_POLLING => GRBCallbackCodes::POLLING,
            ffi::GRB_CB_NLBAR => GRBCallbackCodes::NLBAR,
            ffi::GRB_CB_PDHG => GRBCallbackCodes::PDHG,
            ffi::GRB_CB_IIS => GRBCallbackCodes::IIS,
            ffi::GRB_CB_MULTIOBJ => GRBCallbackCodes::MULTIOBJ,
            ffi::GRB_CB_BARRIER => GRBCallbackCodes::BARRIER,
            ffi::GRB_CB_MESSAGE => GRBCallbackCodes::MESSAGE,
            ffi::GRB_CB_MIPNODE => GRBCallbackCodes::MIPNODE,
            ffi::GRB_CB_MIPSOL => GRBCallbackCodes::MIPSOL,
            ffi::GRB_CB_MIP => GRBCallbackCodes::MIP,
            ffi::GRB_CB_SIMPLEX => GRBCallbackCodes::SIMPLEX,
            ffi::GRB_CB_PRESOLVE => GRBCallbackCodes::PRESOLVE,
            _ => panic!("Unknown callback code: {}", where_),
        }
    }
}

impl From<GRB_WHAT_DOUBLE> for std::ffi::c_int {
    fn from(value: GRB_WHAT_DOUBLE) -> Self {
        match value {
            GRB_WHAT_DOUBLE::RUNTIME => ffi::GRB_CB_RUNTIME,
            GRB_WHAT_DOUBLE::WORK => ffi::GRB_CB_WORK,
            GRB_WHAT_DOUBLE::MEMUSED => ffi::GRB_CB_MEMUSED,
            GRB_WHAT_DOUBLE::MAXMEMUSED => ffi::GRB_CB_MAXMEMUSED,
            GRB_WHAT_DOUBLE::SPX_ITRCNT => ffi::GRB_CB_SPX_ITRCNT,
            GRB_WHAT_DOUBLE::SPX_OBJVAL => ffi::GRB_CB_SPX_OBJVAL,
            GRB_WHAT_DOUBLE::SPX_PRIMINF => ffi::GRB_CB_SPX_PRIMINF,
            GRB_WHAT_DOUBLE::SPX_DUALINF => ffi::GRB_CB_SPX_DUALINF,
            GRB_WHAT_DOUBLE::MIP_OBJBST => ffi::GRB_CB_MIP_OBJBST,
            GRB_WHAT_DOUBLE::MIP_OBJBND => ffi::GRB_CB_MIP_OBJBND,
            GRB_WHAT_DOUBLE::MIP_NODCNT => ffi::GRB_CB_MIP_NODCNT,
            GRB_WHAT_DOUBLE::MIP_NODLFT => ffi::GRB_CB_MIP_NODLFT,
            GRB_WHAT_DOUBLE::MIP_ITRCNT => ffi::GRB_CB_MIP_ITRCNT,
            GRB_WHAT_DOUBLE::MIPSOL_OBJ => ffi::GRB_CB_MIPSOL_OBJ,
            GRB_WHAT_DOUBLE::MIPSOL_OBJBST => ffi::GRB_CB_MIPSOL_OBJBST,
            GRB_WHAT_DOUBLE::MIPSOL_OBJBND => ffi::GRB_CB_MIPSOL_OBJBND,
            GRB_WHAT_DOUBLE::MIPSOL_NODCNT => ffi::GRB_CB_MIPSOL_NODCNT,
            GRB_WHAT_DOUBLE::MIPNODE_OBJBST => ffi::GRB_CB_MIPNODE_OBJBST,
            GRB_WHAT_DOUBLE::MIPNODE_OBJBND => ffi::GRB_CB_MIPNODE_OBJBND,
            GRB_WHAT_DOUBLE::MIPNODE_NODCNT => ffi::GRB_CB_MIPNODE_NODCNT,
            GRB_WHAT_DOUBLE::BARRIER_PRIMOBJ => ffi::GRB_CB_BARRIER_PRIMOBJ,
            GRB_WHAT_DOUBLE::BARRIER_DUALOBJ => ffi::GRB_CB_BARRIER_DUALOBJ,
            GRB_WHAT_DOUBLE::BARRIER_PRIMINF => ffi::GRB_CB_BARRIER_PRIMINF,
            GRB_WHAT_DOUBLE::BARRIER_DUALINF => ffi::GRB_CB_BARRIER_DUALINF,
            GRB_WHAT_DOUBLE::BARRIER_COMPL => ffi::GRB_CB_BARRIER_COMPL,
            GRB_WHAT_DOUBLE::MULTIOBJ_OBJBST => ffi::GRB_CB_MULTIOBJ_OBJBST,
            GRB_WHAT_DOUBLE::MULTIOBJ_OBJBND => ffi::GRB_CB_MULTIOBJ_OBJBND,
            GRB_WHAT_DOUBLE::MULTIOBJ_MIPGAP => ffi::GRB_CB_MULTIOBJ_MIPGAP,
            GRB_WHAT_DOUBLE::MULTIOBJ_ITRCNT => ffi::GRB_CB_MULTIOBJ_ITRCNT,
            GRB_WHAT_DOUBLE::MULTIOBJ_NODCNT => ffi::GRB_CB_MULTIOBJ_NODCNT,
            GRB_WHAT_DOUBLE::MULTIOBJ_NODLFT => ffi::GRB_CB_MULTIOBJ_NODLFT,
            GRB_WHAT_DOUBLE::MULTIOBJ_RUNTIME => ffi::GRB_CB_MULTIOBJ_RUNTIME,
            GRB_WHAT_DOUBLE::MULTIOBJ_WORK => ffi::GRB_CB_MULTIOBJ_WORK,
            GRB_WHAT_DOUBLE::PDHG_PRIMOBJ => ffi::GRB_CB_PDHG_PRIMOBJ,
            GRB_WHAT_DOUBLE::PDHG_DUALOBJ => ffi::GRB_CB_PDHG_DUALOBJ,
            GRB_WHAT_DOUBLE::PDHG_PRIMINF => ffi::GRB_CB_PDHG_PRIMINF,
            GRB_WHAT_DOUBLE::PDHG_DUALINF => ffi::GRB_CB_PDHG_DUALINF,
            GRB_WHAT_DOUBLE::PDHG_COMPL => ffi::GRB_CB_PDHG_COMPL,
            GRB_WHAT_DOUBLE::NLBAR_PRIMOBJ => ffi::GRB_CB_NLBAR_PRIMOBJ,
            GRB_WHAT_DOUBLE::NLBAR_PRIMINF => ffi::GRB_CB_NLBAR_PRIMINF,
            GRB_WHAT_DOUBLE::NLBAR_DUALINF => ffi::GRB_CB_NLBAR_DUALINF,
            GRB_WHAT_DOUBLE::NLBAR_COMPL => ffi::GRB_CB_NLBAR_COMPL,
        }
    }
}

impl From<GRB_WHAT_INT> for std::ffi::c_int {
    fn from(value: GRB_WHAT_INT) -> Self {
        match value {
            GRB_WHAT_INT::PRE_COLDEL => ffi::GRB_CB_PRE_COLDEL,
            GRB_WHAT_INT::PRE_ROWDEL => ffi::GRB_CB_PRE_ROWDEL,
            GRB_WHAT_INT::PRE_SENCHG => ffi::GRB_CB_PRE_SENCHG,
            GRB_WHAT_INT::PRE_BNDCHG => ffi::GRB_CB_PRE_BNDCHG,
            GRB_WHAT_INT::PRE_COECHG => ffi::GRB_CB_PRE_COECHG,
            GRB_WHAT_INT::SPX_ISPERT => ffi::GRB_CB_SPX_ISPERT,
            GRB_WHAT_INT::MIP_SOLCNT => ffi::GRB_CB_MIP_SOLCNT,
            GRB_WHAT_INT::MIP_CUTCNT => ffi::GRB_CB_MIP_CUTCNT,
            GRB_WHAT_INT::MIP_OPENSCENARIOS => ffi::GRB_CB_MIP_OPENSCENARIOS,
            GRB_WHAT_INT::MIP_PHASE => ffi::GRB_CB_MIP_PHASE,
            GRB_WHAT_INT::MIPSOL_SOLCNT => ffi::GRB_CB_MIPSOL_SOLCNT,
            GRB_WHAT_INT::MIPSOL_OPENSCENARIOS => ffi::GRB_CB_MIPSOL_OPENSCENARIOS,
            GRB_WHAT_INT::MIPSOL_PHASE => ffi::GRB_CB_MIPSOL_PHASE,
            GRB_WHAT_INT::MIPNODE_STATUS => ffi::GRB_CB_MIPNODE_STATUS,
            GRB_WHAT_INT::MIPNODE_SOLCNT => ffi::GRB_CB_MIPNODE_SOLCNT,
            GRB_WHAT_INT::MIPNODE_OPENSCENARIOS => ffi::GRB_CB_MIPNODE_OPENSCENARIOS,
            GRB_WHAT_INT::MIPNODE_PHASE => ffi::GRB_CB_MIPNODE_PHASE,
            GRB_WHAT_INT::BARRIER_ITRCNT => ffi::GRB_CB_BARRIER_ITRCNT,
            GRB_WHAT_INT::MULTIOBJ_OBJCNT => ffi::GRB_CB_MULTIOBJ_OBJCNT,
            GRB_WHAT_INT::MULTIOBJ_SOLCNT => ffi::GRB_CB_MULTIOBJ_SOLCNT,
            GRB_WHAT_INT::MULTIOBJ_STATUS => ffi::GRB_CB_MULTIOBJ_STATUS,
            GRB_WHAT_INT::IIS_CONSTRMIN => ffi::GRB_CB_IIS_CONSTRMIN,
            GRB_WHAT_INT::IIS_CONSTRMAX => ffi::GRB_CB_IIS_CONSTRMAX,
            GRB_WHAT_INT::IIS_CONSTRGUESS => ffi::GRB_CB_IIS_CONSTRGUESS,
            GRB_WHAT_INT::IIS_BOUNDMIN => ffi::GRB_CB_IIS_BOUNDMIN,
            GRB_WHAT_INT::IIS_BOUNDMAX => ffi::GRB_CB_IIS_BOUNDMAX,
            GRB_WHAT_INT::IIS_BOUNDGUESS => ffi::GRB_CB_IIS_BOUNDGUESS,
            GRB_WHAT_INT::PDHG_ITRCNT => ffi::GRB_CB_PDHG_ITRCNT,
            GRB_WHAT_INT::NLBAR_ITRCNT => ffi::GRB_CB_NLBAR_ITRCNT,
        }
    }
}

impl From<GRB_WHAT_STRING> for std::ffi::c_int {
    fn from(value: GRB_WHAT_STRING) -> Self {
        match value {
            GRB_WHAT_STRING::MSG_STRING => ffi::GRB_CB_MSG_STRING,
        }
    }
}
pub trait CallbackGet {
    type Output;

    fn get(&self, context: &GRBCallbackContext)
        -> Result<Self::Output, Box<dyn std::error::Error>>;
}

impl CallbackGet for GRB_WHAT_DOUBLE {
    type Output = f64;

    fn get(
        &self,
        context: &GRBCallbackContext,
    ) -> Result<Self::Output, Box<dyn std::error::Error>> {
        let mut result_p: f64 = 0.0;
        let error = unsafe {
            ffi::GRBcbget(
                context.cb_data,
                context.where_ as std::ffi::c_int,
                (*self).into(),
                &mut result_p as *mut f64 as *mut std::ffi::c_void,
            )
        };
        context.get_merror(error)?;
        Ok(result_p)
    }
}

impl CallbackGet for GRB_WHAT_INT {
    type Output = i32;

    fn get(
        &self,
        context: &GRBCallbackContext,
    ) -> Result<Self::Output, Box<dyn std::error::Error>> {
        let mut result_p: i32 = 0;
        let error = unsafe {
            ffi::GRBcbget(
                context.cb_data,
                context.where_ as std::ffi::c_int,
                (*self).into(),
                &mut result_p as *mut i32 as *mut std::ffi::c_void,
            )
        };
        context.get_merror(error)?;
        Ok(result_p)
    }
}

impl CallbackGet for GRB_WHAT_STRING {
    type Output = String;

    fn get(
        &self,
        context: &GRBCallbackContext,
    ) -> Result<Self::Output, Box<dyn std::error::Error>> {
        let result_p: *mut std::ffi::c_char = std::ptr::null_mut();
        let error = unsafe {
            ffi::GRBcbget(
                context.cb_data,
                context.where_ as std::ffi::c_int,
                (*self).into(),
                result_p as *mut std::ffi::c_void,
            )
        };
        context.get_merror(error)?;
        let result_p = unsafe { CStr::from_ptr(result_p) }
            .to_string_lossy()
            .into_owned();
        Ok(result_p)
    }
}

pub trait GetSolution {
    type Output;
    fn get_solution(&self, values: &Vec<f64>) -> Self::Output;
}

impl GetSolution for GRBVar {
    type Output = f64;

    fn get_solution(&self, values: &Vec<f64>) -> Self::Output {
        values[self.index()]
    }
}

impl<T: GetSolution> GetSolution for &Vec<T> {
    type Output = Vec<T::Output>;

    fn get_solution(&self, values: &Vec<f64>) -> Self::Output {
        self.iter().map(|item| item.get_solution(values)).collect()
    }
}
impl<T: GetSolution> GetSolution for Vec<T> {
    type Output = Vec<T::Output>;

    fn get_solution(&self, values: &Vec<f64>) -> Self::Output {
        self.iter().map(|item| item.get_solution(values)).collect()
    }
}

pub trait SetSolution {
    type Value;
    fn set_solution(&self, solution: &mut Vec<f64>, val: Self::Value);
}

impl SetSolution for GRBVar {
    type Value = f64;

    fn set_solution(&self, solution: &mut Vec<f64>, val: Self::Value) {
        solution[self.index()] = val;
    }
}

impl SetSolution for &GRBVar {
    type Value = f64;

    fn set_solution(&self, solution: &mut Vec<f64>, val: Self::Value) {
        solution[self.index()] = val;
    }
}

impl<T: SetSolution> SetSolution for Vec<T> {
    type Value = Vec<T::Value>;

    fn set_solution(&self, solution: &mut Vec<f64>, val: Self::Value) {
        assert_eq!(
            self.len(),
            val.len(),
            "Number of variables ({}) does not match number of values ({})",
            self.len(),
            val.len()
        );
        for (var, v) in self.iter().zip(val) {
            var.set_solution(solution, v);
        }
    }
}

impl<T: SetSolution> SetSolution for &Vec<T> {
    type Value = Vec<T::Value>;

    fn set_solution(&self, solution: &mut Vec<f64>, val: Self::Value) {
        assert_eq!(
            self.len(),
            val.len(),
            "Number of variables ({}) does not match number of values ({})",
            self.len(),
            val.len()
        );
        for (var, v) in self.iter().zip(val) {
            var.set_solution(solution, v);
        }
    }
}
