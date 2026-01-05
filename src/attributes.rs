use crate::{
    constr::{ConstrGetter, ConstrSetter},
    ffi,
    model::{GRBModelPtr, ModelGetter, ModelGetterList, ModelSetter, ModelSetterList},
    modeling::IsModelingObject,
    var::{VariableGetter, VariableSetter},
};
use std::{
    ffi::{CStr, CString},
    ptr::null_mut,
};
#[allow(clippy::upper_case_acronyms, non_camel_case_types)]
#[derive(Clone, Copy)]
pub enum GRBIntAttr {
    /// number of MIP starts
    NUMSTART,
    /// number of scenarios
    NUMSCENARIOS,
    /// number of objectives
    NUMOBJ,
    /// status for a pass during the multi-objective solve
    OBJPASSNSTATUS,
    /// optimization pass in which the selected objective function was processed
    OBJNPASS,
    /// number of optimization passes during the multi-objective solve
    NUMOBJPASSES,
    /// priority
    OBJNPRIORITY,
    /// Force general constr to be (1) or to not be (0) in final IIS
    IIS_GENCONSTRFORCE,
    /// Force QConstr to be (1) or to not be (0) in final IIS
    IIS_QCONSTRFORCE,
    /// Force SOS to be (1) or to not be (0) in final IIS
    IIS_SOSFORCE,
    /// Force constr to be (1) or to not be (0) in final IIS
    IIS_CONSTRFORCE,
    /// Force var UB to be (1) or to not be (0) in final IIS
    IIS_UBFORCE,
    /// Force var LB to be (1) or to not be (0) in final IIS
    IIS_LBFORCE,
    /// Boolean: Is general constr in IIS?
    IIS_GENCONSTR,
    /// Boolean: Is QConstr in IIS?
    IIS_QCONSTR,
    /// Boolean: Is SOS in IIS?
    IIS_SOS,
    /// Boolean: Is constr in IIS?
    IIS_CONSTR,
    /// Boolean: Is var UB in IIS?
    IIS_UB,
    /// Boolean: Is var LB in IIS?
    IIS_LB,
    /// Boolean: Is IIS Minimal?
    IIS_MINIMAL,
    /// Constraint basis status
    CBASIS,
    /// Variable basis status
    VBASIS,
    /// method that solved LP using concurrent
    CONCURRENTWINMETHOD,
    // 0, no basis,
    // 1, has basis, so can be computed
    // 2, available
    HASDUALNORM,
    /// Iters performed (NL barrier)
    NLBARITERCOUNT,
    /// Iters performed (barrier)
    BARITERCOUNT,
    /// Status computed by barrier before crossover
    BARSTATUS,
    /// # of solutions found
    SOLCOUNT,
    /// Optimization status
    STATUS,
    /// An option for PWL translation
    FUNCNONLINEAR,
    /// An option for PWL translation
    FUNCPIECES,
    /// Type of general constraint
    GENCONSTRTYPE,
    /// Lazy constraint?
    LAZY,
    /// Ignore variable for solution identity check in solution pool
    POOLIGNORE,
    /// user specified variable partition
    PARTITION,
    /// variable hint priority
    VARHINTPRI,
    /// Convexity of variable PWL obj
    PWLOBJCVX,
    /// MIP branch priority
    BRANCHPRIORITY,
    /// fingerprint computed from the model data and attributes influencing the optimization process
    FINGERPRINT,
    /// number of tagged elements in model
    NUMTAGGED,
    /// License expiration date
    LICENSE_EXPIRATION,
    /// Model has multiple objectives?
    IS_MULTIOBJ,
    /// Model has quadratic constr?
    IS_QCP,
    /// Is model a QP/MIQP (without Q/NL constraints)?
    IS_QP,
    /// Is model a MIP?
    IS_MIP,
    /// 1=min, -1=max
    MODELSENSE,
    /// # of variables with PWL obj.
    NUMPWLOBJVARS,
    /// # of binary vars
    NUMBINVARS,
    /// # of integer vars
    NUMINTVARS,
    /// # of nz in q constraints
    NUMQCNZS,
    /// # of nz in Q
    NUMQNZS,
    /// # of nz in A
    NUMNZS,
    /// # of general constraints
    NUMGENCONSTRS,
    /// # of quadratic constraints
    NUMQCONSTRS,
    /// # of sos constraints
    NUMSOS,
    /// # of vars
    NUMVARS,
    /// # of constraints
    NUMCONSTRS,
}

impl ConstrSetter for GRBIntAttr {
    type Value = i32;

    fn set(&self, constr: &crate::constr::GRBConstr, value: Self::Value) -> i32 {
        let attr_name: &CStr = (*self).into();
        let value = value as std::ffi::c_int;
        unsafe {
            ffi::GRBsetintattrelement(
                *constr.inner.0,
                attr_name.as_ptr(),
                constr.index() as std::ffi::c_int,
                value,
            )
        }
    }
}

impl ConstrGetter for GRBIntAttr {
    type Value = i32;

    fn get(&self, constr: &crate::prelude::GRBConstr) -> Self::Value {
        let attr_name: &CStr = (*self).into();
        let return_ptr = 0;
        let error = unsafe {
            ffi::GRBgetintattrelement(
                *constr.inner.0,
                attr_name.as_ptr(),
                constr.index() as std::ffi::c_int,
                return_ptr as *mut std::ffi::c_int,
            )
        };
        constr.get_error(error).unwrap();
        return_ptr
    }
}

impl VariableGetter for GRBIntAttr {
    type Value = i32;

    fn get(&self, var: &crate::prelude::GRBVar) -> Self::Value {
        let attr_name: &CStr = (*self).into();
        let return_ptr = 0;
        let error = unsafe {
            ffi::GRBgetintattrelement(
                *var.inner.0,
                attr_name.as_ptr(),
                var.index() as std::ffi::c_int,
                return_ptr as *mut std::ffi::c_int,
            )
        };
        var.get_error(error).unwrap();
        return_ptr
    }
}

impl VariableSetter for GRBIntAttr {
    type Value = i32;

    fn set(&self, var: &crate::prelude::GRBVar, value: Self::Value) -> i32 {
        let attr_name: &CStr = (*self).into();
        unsafe {
            ffi::GRBsetintattrelement(
                *var.inner.0,
                attr_name.as_ptr(),
                var.index() as std::ffi::c_int,
                value,
            )
        }
    }
}

impl ModelGetter for GRBIntAttr {
    type Value = i32;

    fn get(&self, model: *mut ffi::GRBmodel) -> Self::Value {
        let mut value_p = 0;
        let attr_name: &CStr = (*self).into();
        let error = unsafe {
            ffi::GRBgetintattr(model, attr_name.as_ptr(), value_p as *mut std::ffi::c_int)
        };
        value_p
    }
}

impl ModelSetter for GRBIntAttr {
    type Value = i32;

    fn set(&self, model: *mut ffi::GRBmodel, value: Self::Value) -> i32 {
        let attr_name: &CStr = (*self).into();
        unsafe { ffi::GRBsetintattr(model, attr_name.as_ptr(), value) }
    }
}

impl<C> ModelSetterList<C> for GRBIntAttr
where
    C: IsModelingObject,
{
    type Value = i32;

    fn set_list(&self, model: *mut ffi::GRBmodel, inds: Vec<C>, values: Vec<Self::Value>) -> i32 {
        let attr_name: &CStr = (*self).into();
        let len = values.len();
        let mut inds = inds
            .iter()
            .map(|c| c.index() as std::ffi::c_int)
            .collect::<Vec<_>>();
        let mut values = values;
        unsafe {
            ffi::GRBsetintattrlist(
                model,
                attr_name.as_ptr(),
                len as std::ffi::c_int,
                inds.as_mut_ptr(),
                values.as_mut_ptr(),
            )
        }
    }
}

impl<C> ModelGetterList<C> for GRBIntAttr
where
    C: IsModelingObject,
{
    type Value = i32;

    fn get_list(&self, model: *mut ffi::GRBmodel, inds: Vec<C>) -> Vec<Self::Value> {
        let len = inds.len();
        let mut inds = inds
            .iter()
            .map(|c| c.index() as std::ffi::c_int)
            .collect::<Vec<_>>();
        let mut values = vec![0 as std::ffi::c_int; len];
        let attr_name: &CStr = (*self).into();
        unsafe {
            ffi::GRBgetintattrlist(
                model,
                attr_name.as_ptr(),
                len as std::ffi::c_int,
                inds.as_mut_ptr(),
                values.as_mut_ptr(),
            )
        };
        values
    }
}
impl From<GRBIntAttr> for &'static CStr {
    fn from(value: GRBIntAttr) -> &'static CStr {
        match value {
            GRBIntAttr::NUMSTART => ffi::GRB_INT_ATTR_NUMSTART,
            GRBIntAttr::NUMSCENARIOS => ffi::GRB_INT_ATTR_NUMSCENARIOS,
            GRBIntAttr::NUMOBJ => ffi::GRB_INT_ATTR_NUMOBJ,
            GRBIntAttr::OBJPASSNSTATUS => ffi::GRB_INT_ATTR_OBJPASSNSTATUS,
            GRBIntAttr::OBJNPASS => ffi::GRB_INT_ATTR_OBJNPASS,
            GRBIntAttr::NUMOBJPASSES => ffi::GRB_INT_ATTR_NUMOBJPASSES,
            GRBIntAttr::OBJNPRIORITY => ffi::GRB_INT_ATTR_OBJNPRIORITY,
            GRBIntAttr::IIS_GENCONSTRFORCE => ffi::GRB_INT_ATTR_IIS_GENCONSTRFORCE,
            GRBIntAttr::IIS_QCONSTRFORCE => ffi::GRB_INT_ATTR_IIS_QCONSTRFORCE,
            GRBIntAttr::IIS_SOSFORCE => ffi::GRB_INT_ATTR_IIS_SOSFORCE,
            GRBIntAttr::IIS_CONSTRFORCE => ffi::GRB_INT_ATTR_IIS_CONSTRFORCE,
            GRBIntAttr::IIS_UBFORCE => ffi::GRB_INT_ATTR_IIS_UBFORCE,
            GRBIntAttr::IIS_LBFORCE => ffi::GRB_INT_ATTR_IIS_LBFORCE,
            GRBIntAttr::IIS_GENCONSTR => ffi::GRB_INT_ATTR_IIS_GENCONSTR,
            GRBIntAttr::IIS_QCONSTR => ffi::GRB_INT_ATTR_IIS_QCONSTR,
            GRBIntAttr::IIS_SOS => ffi::GRB_INT_ATTR_IIS_SOS,
            GRBIntAttr::IIS_CONSTR => ffi::GRB_INT_ATTR_IIS_CONSTR,
            GRBIntAttr::IIS_UB => ffi::GRB_INT_ATTR_IIS_UB,
            GRBIntAttr::IIS_LB => ffi::GRB_INT_ATTR_IIS_LB,
            GRBIntAttr::IIS_MINIMAL => ffi::GRB_INT_ATTR_IIS_MINIMAL,
            GRBIntAttr::CBASIS => ffi::GRB_INT_ATTR_CBASIS,
            GRBIntAttr::VBASIS => ffi::GRB_INT_ATTR_VBASIS,
            GRBIntAttr::CONCURRENTWINMETHOD => ffi::GRB_INT_ATTR_CONCURRENTWINMETHOD,
            GRBIntAttr::HASDUALNORM => ffi::GRB_INT_ATTR_HASDUALNORM,
            GRBIntAttr::NLBARITERCOUNT => ffi::GRB_INT_ATTR_NLBARITERCOUNT,
            GRBIntAttr::BARITERCOUNT => ffi::GRB_INT_ATTR_BARITERCOUNT,
            GRBIntAttr::BARSTATUS => ffi::GRB_INT_ATTR_BARSTATUS,
            GRBIntAttr::SOLCOUNT => ffi::GRB_INT_ATTR_SOLCOUNT,
            GRBIntAttr::STATUS => ffi::GRB_INT_ATTR_STATUS,
            GRBIntAttr::FUNCNONLINEAR => ffi::GRB_INT_ATTR_FUNCNONLINEAR,
            GRBIntAttr::FUNCPIECES => ffi::GRB_INT_ATTR_FUNCPIECES,
            GRBIntAttr::GENCONSTRTYPE => ffi::GRB_INT_ATTR_GENCONSTRTYPE,
            GRBIntAttr::LAZY => ffi::GRB_INT_ATTR_LAZY,
            GRBIntAttr::POOLIGNORE => ffi::GRB_INT_ATTR_POOLIGNORE,
            GRBIntAttr::PARTITION => ffi::GRB_INT_ATTR_PARTITION,
            GRBIntAttr::VARHINTPRI => ffi::GRB_INT_ATTR_VARHINTPRI,
            GRBIntAttr::PWLOBJCVX => ffi::GRB_INT_ATTR_PWLOBJCVX,
            GRBIntAttr::BRANCHPRIORITY => ffi::GRB_INT_ATTR_BRANCHPRIORITY,
            GRBIntAttr::FINGERPRINT => ffi::GRB_INT_ATTR_FINGERPRINT,
            GRBIntAttr::NUMTAGGED => ffi::GRB_INT_ATTR_NUMTAGGED,
            GRBIntAttr::LICENSE_EXPIRATION => ffi::GRB_INT_ATTR_LICENSE_EXPIRATION,
            GRBIntAttr::IS_MULTIOBJ => ffi::GRB_INT_ATTR_IS_MULTIOBJ,
            GRBIntAttr::IS_QCP => ffi::GRB_INT_ATTR_IS_QCP,
            GRBIntAttr::IS_QP => ffi::GRB_INT_ATTR_IS_QP,
            GRBIntAttr::IS_MIP => ffi::GRB_INT_ATTR_IS_MIP,
            GRBIntAttr::MODELSENSE => ffi::GRB_INT_ATTR_MODELSENSE,
            GRBIntAttr::NUMPWLOBJVARS => ffi::GRB_INT_ATTR_NUMPWLOBJVARS,
            GRBIntAttr::NUMBINVARS => ffi::GRB_INT_ATTR_NUMBINVARS,
            GRBIntAttr::NUMINTVARS => ffi::GRB_INT_ATTR_NUMINTVARS,
            GRBIntAttr::NUMQCNZS => ffi::GRB_INT_ATTR_NUMQCNZS,
            GRBIntAttr::NUMQNZS => ffi::GRB_INT_ATTR_NUMQNZS,
            GRBIntAttr::NUMNZS => ffi::GRB_INT_ATTR_NUMNZS,
            GRBIntAttr::NUMGENCONSTRS => ffi::GRB_INT_ATTR_NUMGENCONSTRS,
            GRBIntAttr::NUMQCONSTRS => ffi::GRB_INT_ATTR_NUMQCONSTRS,
            GRBIntAttr::NUMSOS => ffi::GRB_INT_ATTR_NUMSOS,
            GRBIntAttr::NUMVARS => ffi::GRB_INT_ATTR_NUMVARS,
            GRBIntAttr::NUMCONSTRS => ffi::GRB_INT_ATTR_NUMCONSTRS,
        }
    }
}

#[allow(clippy::upper_case_acronyms, non_camel_case_types)]
#[derive(Clone, Copy)]
pub enum GRBDblAttr {
    /// Deprecated since v13 - use POOLNX instead
    Xn,
    /// maximum amount of allocated memory (in GB) in master environment
    MAXMEMUSED,
    /// current amount of allocated memory (in GB) in master environment
    MEMUSED,
    /// objective value for scenario i
    SCENNOBJVAL,
    /// objective bound for scenario i
    SCENNOBJBOUND,
    /// solution value in scenario i
    SCENNX,
    /// right hand side in scenario i
    SCENNRHS,
    /// objective in scenario i
    SCENNOBJ,
    /// upper bound in scenario i
    SCENNUB,
    /// lower bound in scenario i
    SCENNLB,
    /// work done for a pass during the multi-objective solve
    OBJPASSNWORK,
    /// runtime for a pass during the multi-objective solve
    OBJPASSNRUNTIME,
    /// number of unexplored nodes for a pass during the multi-objective solve
    OBJPASSNOPENNODECOUNT,
    /// objective value for a pass during the multi-objective solve
    OBJPASSNOBJVAL,
    /// objective bound for a pass during the multi-objective solve
    OBJPASSNOBJBOUND,
    /// number of explored nodes for a pass during the multi-objective solve
    OBJPASSNNODECOUNT,
    /// MIP gap for a pass during the multi-objective solve
    OBJPASSNMIPGAP,
    /// simplex iteration count for a pass during the multi-objective solve
    OBJPASSNITERCOUNT,
    /// absolute tolerance
    OBJNABSTOL,
    /// relative tolerance
    OBJNRELTOL,
    /// weight
    OBJNWEIGHT,
    /// constant term
    OBJNCON,
    /// Solution objective for Multi-objectives, also depends on solutionnumber
    OBJNVAL,
    /// ith objective
    OBJN,
    /// Dual norm square
    CDUALNORM,
    /// QC Constraint slack
    QCSLACK,
    /// Constraint slack
    SLACK,
    /// Dual value for QC
    QCPI,
    /// Dual value
    PI,
    /// Dual norm square
    VDUALNORM,
    /// Reduced costs
    RC,
    /// Best barrier dual iterate
    BARPI,
    /// Best barrier primal iterate
    BARX,
    /// Deprecated since v13 - use POOLNX instead
    XN,
    /// Alternate MIP solution, depends on solutionnumber
    POOLNX,
    /// Solution value
    X,
    /// Unexplored nodes (B&C)
    OPENNODECOUNT,
    /// Nodes explored (B&C)
    NODECOUNT,
    /// Iters performed (PDHG)
    PDHGITERCOUNT,
    /// Iters performed (simplex)
    ITERCOUNT,
    /// MIP optimality gap
    MIPGAP,
    /// Deprecated since v13 - use POOLNOBJVAL instead
    POOLOBJVAL,
    /// Solution objective, depends on solutionnumber
    POOLNOBJVAL,
    /// Best bound on pool solution
    POOLOBJBOUND,
    /// Continuous bound
    OBJBOUNDC,
    /// Best bound on solution
    OBJBOUND,
    /// Solution objective
    OBJVAL,
    /// Work for optimization
    WORK,
    /// Run time for optimization
    RUNTIME,
    /// Min (abs) rhs of Q
    MIN_QCRHS,
    /// Max (abs) rhs of Q
    MAX_QCRHS,
    /// Min (abs) nz coeff in linear part of Q
    MIN_QCLCOEFF,
    /// Max (abs) nz coeff in linear part of Q
    MAX_QCLCOEFF,
    /// Min (abs) obj coeff of quadratic part
    MIN_QOBJ_COEFF,
    /// Max (abs) obj coeff of quadratic part
    MAX_QOBJ_COEFF,
    /// Min (abs) nz coeff in Q
    MIN_QCCOEFF,
    /// Max (abs) nz coeff in Q
    MAX_QCCOEFF,
    /// Min (abs) rhs coeff
    MIN_RHS,
    /// Max (abs) rhs coeff
    MAX_RHS,
    /// Min (abs) obj coeff
    MIN_OBJ_COEFF,
    /// Max (abs) obj coeff
    MAX_OBJ_COEFF,
    /// Min (abs) var bd
    MIN_BOUND,
    /// Max (abs) finite var bd
    MAX_BOUND,
    /// Min (abs) nz coeff in A
    MIN_COEFF,
    /// Max (abs) nz coeff in A
    MAX_COEFF,
    /// An option for PWL translation
    FUNCPIECERATIO,
    /// An option for PWL translation
    FUNCPIECELENGTH,
    /// An option for PWL translation
    FUNCPIECEERROR,
    /// QC RHS
    QCRHS,
    /// LP dual solution warm start
    DSTART,
    /// RHS
    RHS,
    /// variable hint value
    VARHINTVAL,
    /// LP primal solution warm start
    PSTART,
    /// MIP start value, depends on startnumber
    START,
    /// Objective coeff
    OBJ,
    /// Upper bound
    UB,
    /// Lower bound
    LB,
    /// Objective constant
    OBJCON,
    /// # of nz in A
    DNUMNZS,
}
impl ConstrSetter for GRBDblAttr {
    type Value = f64;

    fn set(&self, constr: &crate::constr::GRBConstr, value: Self::Value) -> i32 {
        let attr_name: &CStr = (*self).into();
        let value = value as std::ffi::c_double;
        unsafe {
            ffi::GRBsetdblattrelement(
                *constr.inner.0,
                attr_name.as_ptr(),
                constr.index() as std::ffi::c_int,
                value,
            )
        }
    }
}

impl ConstrGetter for GRBDblAttr {
    type Value = f64;

    fn get(&self, constr: &crate::prelude::GRBConstr) -> Self::Value {
        let attr_name: &CStr = (*self).into();
        let mut return_ptr = 0.0;
        let error = unsafe {
            ffi::GRBgetdblattrelement(
                *constr.inner.0,
                attr_name.as_ptr(),
                constr.index() as std::ffi::c_int,
                &mut return_ptr as *mut std::ffi::c_double,
            )
        };
        constr.get_error(error).unwrap();
        return_ptr
    }
}

impl VariableGetter for GRBDblAttr {
    type Value = f64;

    fn get(&self, var: &crate::prelude::GRBVar) -> Self::Value {
        let attr_name: &CStr = (*self).into();
        let mut return_ptr = 0.0;
        let error = unsafe {
            ffi::GRBgetdblattrelement(
                *var.inner.0,
                attr_name.as_ptr(),
                var.index() as std::ffi::c_int,
                &mut return_ptr as *mut std::ffi::c_double,
            )
        };
        var.get_error(error).unwrap();
        return_ptr
    }
}

impl VariableSetter for GRBDblAttr {
    type Value = f64;

    fn set(&self, var: &crate::prelude::GRBVar, value: Self::Value) -> i32 {
        let attr_name: &CStr = (*self).into();
        unsafe {
            ffi::GRBsetdblattrelement(
                *var.inner.0,
                attr_name.as_ptr(),
                var.index() as std::ffi::c_int,
                value,
            )
        }
    }
}

impl ModelGetter for GRBDblAttr {
    type Value = f64;

    fn get(&self, model: *mut ffi::GRBmodel) -> Self::Value {
        let mut value_p = 0.0;
        let attr_name: &CStr = (*self).into();
        let error = unsafe {
            ffi::GRBgetdblattr(
                model,
                attr_name.as_ptr(),
                &mut value_p as *mut std::ffi::c_double,
            )
        };
        value_p
    }
}

impl ModelSetter for GRBDblAttr {
    type Value = f64;

    fn set(&self, model: *mut ffi::GRBmodel, value: Self::Value) -> i32 {
        let attr_name: &CStr = (*self).into();
        unsafe { ffi::GRBsetdblattr(model, attr_name.as_ptr(), value) }
    }
}

impl<C> ModelSetterList<C> for GRBDblAttr
where
    C: IsModelingObject,
{
    type Value = f64;

    fn set_list(&self, model: *mut ffi::GRBmodel, inds: Vec<C>, values: Vec<Self::Value>) -> i32 {
        let attr_name: &CStr = (*self).into();
        let len = values.len();
        let mut inds = inds
            .iter()
            .map(|c| c.index() as std::ffi::c_int)
            .collect::<Vec<_>>();
        let mut values = values;
        unsafe {
            ffi::GRBsetdblattrlist(
                model,
                attr_name.as_ptr(),
                len as std::ffi::c_int,
                inds.as_mut_ptr(),
                values.as_mut_ptr(),
            )
        }
    }
}

impl<C> ModelGetterList<C> for GRBDblAttr
where
    C: IsModelingObject,
{
    type Value = f64;

    fn get_list(&self, model: *mut ffi::GRBmodel, inds: Vec<C>) -> Vec<Self::Value> {
        let len = inds.len();
        let mut inds = inds
            .iter()
            .map(|c| c.index() as std::ffi::c_int)
            .collect::<Vec<_>>();
        let mut values = vec![0.0 as std::ffi::c_double; len];
        let attr_name: &CStr = (*self).into();
        unsafe {
            ffi::GRBgetdblattrlist(
                model,
                attr_name.as_ptr(),
                len as std::ffi::c_int,
                inds.as_mut_ptr(),
                values.as_mut_ptr(),
            )
        };
        values
    }
}
impl From<GRBDblAttr> for &'static CStr {
    fn from(value: GRBDblAttr) -> &'static CStr {
        match value {
            GRBDblAttr::Xn => ffi::GRB_DBL_ATTR_Xn,
            GRBDblAttr::MAXMEMUSED => ffi::GRB_DBL_ATTR_MAXMEMUSED,
            GRBDblAttr::MEMUSED => ffi::GRB_DBL_ATTR_MEMUSED,
            GRBDblAttr::SCENNOBJVAL => ffi::GRB_DBL_ATTR_SCENNOBJVAL,
            GRBDblAttr::SCENNOBJBOUND => ffi::GRB_DBL_ATTR_SCENNOBJBOUND,
            GRBDblAttr::SCENNX => ffi::GRB_DBL_ATTR_SCENNX,
            GRBDblAttr::SCENNRHS => ffi::GRB_DBL_ATTR_SCENNRHS,
            GRBDblAttr::SCENNOBJ => ffi::GRB_DBL_ATTR_SCENNOBJ,
            GRBDblAttr::SCENNUB => ffi::GRB_DBL_ATTR_SCENNUB,
            GRBDblAttr::SCENNLB => ffi::GRB_DBL_ATTR_SCENNLB,
            GRBDblAttr::OBJPASSNWORK => ffi::GRB_DBL_ATTR_OBJPASSNWORK,
            GRBDblAttr::OBJPASSNRUNTIME => ffi::GRB_DBL_ATTR_OBJPASSNRUNTIME,
            GRBDblAttr::OBJPASSNOPENNODECOUNT => ffi::GRB_DBL_ATTR_OBJPASSNOPENNODECOUNT,
            GRBDblAttr::OBJPASSNOBJVAL => ffi::GRB_DBL_ATTR_OBJPASSNOBJVAL,
            GRBDblAttr::OBJPASSNOBJBOUND => ffi::GRB_DBL_ATTR_OBJPASSNOBJBOUND,
            GRBDblAttr::OBJPASSNNODECOUNT => ffi::GRB_DBL_ATTR_OBJPASSNNODECOUNT,
            GRBDblAttr::OBJPASSNMIPGAP => ffi::GRB_DBL_ATTR_OBJPASSNMIPGAP,
            GRBDblAttr::OBJPASSNITERCOUNT => ffi::GRB_DBL_ATTR_OBJPASSNITERCOUNT,
            GRBDblAttr::OBJNABSTOL => ffi::GRB_DBL_ATTR_OBJNABSTOL,
            GRBDblAttr::OBJNRELTOL => ffi::GRB_DBL_ATTR_OBJNRELTOL,
            GRBDblAttr::OBJNWEIGHT => ffi::GRB_DBL_ATTR_OBJNWEIGHT,
            GRBDblAttr::OBJNCON => ffi::GRB_DBL_ATTR_OBJNCON,
            GRBDblAttr::OBJNVAL => ffi::GRB_DBL_ATTR_OBJNVAL,
            GRBDblAttr::OBJN => ffi::GRB_DBL_ATTR_OBJN,
            GRBDblAttr::CDUALNORM => ffi::GRB_DBL_ATTR_CDUALNORM,
            GRBDblAttr::QCSLACK => ffi::GRB_DBL_ATTR_QCSLACK,
            GRBDblAttr::SLACK => ffi::GRB_DBL_ATTR_SLACK,
            GRBDblAttr::QCPI => ffi::GRB_DBL_ATTR_QCPI,
            GRBDblAttr::PI => ffi::GRB_DBL_ATTR_PI,
            GRBDblAttr::VDUALNORM => ffi::GRB_DBL_ATTR_VDUALNORM,
            GRBDblAttr::RC => ffi::GRB_DBL_ATTR_RC,
            GRBDblAttr::BARPI => ffi::GRB_DBL_ATTR_BARPI,
            GRBDblAttr::BARX => ffi::GRB_DBL_ATTR_BARX,
            GRBDblAttr::XN => ffi::GRB_DBL_ATTR_XN,
            GRBDblAttr::POOLNX => ffi::GRB_DBL_ATTR_POOLNX,
            GRBDblAttr::X => ffi::GRB_DBL_ATTR_X,
            GRBDblAttr::OPENNODECOUNT => ffi::GRB_DBL_ATTR_OPENNODECOUNT,
            GRBDblAttr::NODECOUNT => ffi::GRB_DBL_ATTR_NODECOUNT,
            GRBDblAttr::PDHGITERCOUNT => ffi::GRB_DBL_ATTR_PDHGITERCOUNT,
            GRBDblAttr::ITERCOUNT => ffi::GRB_DBL_ATTR_ITERCOUNT,
            GRBDblAttr::MIPGAP => ffi::GRB_DBL_ATTR_MIPGAP,
            GRBDblAttr::POOLOBJVAL => ffi::GRB_DBL_ATTR_POOLOBJVAL,
            GRBDblAttr::POOLNOBJVAL => ffi::GRB_DBL_ATTR_POOLNOBJVAL,
            GRBDblAttr::POOLOBJBOUND => ffi::GRB_DBL_ATTR_POOLOBJBOUND,
            GRBDblAttr::OBJBOUNDC => ffi::GRB_DBL_ATTR_OBJBOUNDC,
            GRBDblAttr::OBJBOUND => ffi::GRB_DBL_ATTR_OBJBOUND,
            GRBDblAttr::OBJVAL => ffi::GRB_DBL_ATTR_OBJVAL,
            GRBDblAttr::WORK => ffi::GRB_DBL_ATTR_WORK,
            GRBDblAttr::RUNTIME => ffi::GRB_DBL_ATTR_RUNTIME,
            GRBDblAttr::MIN_QCRHS => ffi::GRB_DBL_ATTR_MIN_QCRHS,
            GRBDblAttr::MAX_QCRHS => ffi::GRB_DBL_ATTR_MAX_QCRHS,
            GRBDblAttr::MIN_QCLCOEFF => ffi::GRB_DBL_ATTR_MIN_QCLCOEFF,
            GRBDblAttr::MAX_QCLCOEFF => ffi::GRB_DBL_ATTR_MAX_QCLCOEFF,
            GRBDblAttr::MIN_QOBJ_COEFF => ffi::GRB_DBL_ATTR_MIN_QOBJ_COEFF,
            GRBDblAttr::MAX_QOBJ_COEFF => ffi::GRB_DBL_ATTR_MAX_QOBJ_COEFF,
            GRBDblAttr::MIN_QCCOEFF => ffi::GRB_DBL_ATTR_MIN_QCCOEFF,
            GRBDblAttr::MAX_QCCOEFF => ffi::GRB_DBL_ATTR_MAX_QCCOEFF,
            GRBDblAttr::MIN_RHS => ffi::GRB_DBL_ATTR_MIN_RHS,
            GRBDblAttr::MAX_RHS => ffi::GRB_DBL_ATTR_MAX_RHS,
            GRBDblAttr::MIN_OBJ_COEFF => ffi::GRB_DBL_ATTR_MIN_OBJ_COEFF,
            GRBDblAttr::MAX_OBJ_COEFF => ffi::GRB_DBL_ATTR_MAX_OBJ_COEFF,
            GRBDblAttr::MIN_BOUND => ffi::GRB_DBL_ATTR_MIN_BOUND,
            GRBDblAttr::MAX_BOUND => ffi::GRB_DBL_ATTR_MAX_BOUND,
            GRBDblAttr::MIN_COEFF => ffi::GRB_DBL_ATTR_MIN_COEFF,
            GRBDblAttr::MAX_COEFF => ffi::GRB_DBL_ATTR_MAX_COEFF,
            GRBDblAttr::FUNCPIECERATIO => ffi::GRB_DBL_ATTR_FUNCPIECERATIO,
            GRBDblAttr::FUNCPIECELENGTH => ffi::GRB_DBL_ATTR_FUNCPIECELENGTH,
            GRBDblAttr::FUNCPIECEERROR => ffi::GRB_DBL_ATTR_FUNCPIECEERROR,
            GRBDblAttr::QCRHS => ffi::GRB_DBL_ATTR_QCRHS,
            GRBDblAttr::DSTART => ffi::GRB_DBL_ATTR_DSTART,
            GRBDblAttr::RHS => ffi::GRB_DBL_ATTR_RHS,
            GRBDblAttr::VARHINTVAL => ffi::GRB_DBL_ATTR_VARHINTVAL,
            GRBDblAttr::PSTART => ffi::GRB_DBL_ATTR_PSTART,
            GRBDblAttr::START => ffi::GRB_DBL_ATTR_START,
            GRBDblAttr::OBJ => ffi::GRB_DBL_ATTR_OBJ,
            GRBDblAttr::UB => ffi::GRB_DBL_ATTR_UB,
            GRBDblAttr::LB => ffi::GRB_DBL_ATTR_LB,
            GRBDblAttr::OBJCON => ffi::GRB_DBL_ATTR_OBJCON,
            GRBDblAttr::DNUMNZS => ffi::GRB_DBL_ATTR_DNUMNZS,
        }
    }
}

#[allow(clippy::upper_case_acronyms, non_camel_case_types)]
#[derive(Clone, Copy)]
pub enum GRBStrAttr {
    /// name of scenario i
    SCENNNAME,
    /// name
    OBJNNAME,
    /// Name of general constraint
    GENCONSTRNAME,
    /// QC name
    QCNAME,
    /// quadratic constraint tags
    QCTAG,
    /// Constraint name
    CONSTRNAME,
    /// linear constraint tags
    CTAG,
    /// variable tags
    VTAG,
    /// Variable name
    VARNAME,
    /// model name
    MODELNAME,
}

impl ConstrSetter for GRBStrAttr {
    type Value = String;

    fn set(&self, constr: &crate::constr::GRBConstr, value: Self::Value) -> i32 {
        let attr_name: &CStr = (*self).into();
        let value =
            CString::new(value).expect("Failed to convert String to CString in `ModelSetter::set`");
        unsafe {
            ffi::GRBsetstrattrelement(
                *constr.inner.0,
                attr_name.as_ptr(),
                constr.index() as std::ffi::c_int,
                value.as_ptr(),
            )
        }
    }
}

impl ConstrGetter for GRBStrAttr {
    type Value = String;

    fn get(&self, constr: &crate::prelude::GRBConstr) -> Self::Value {
        let attr_name: &CStr = (*self).into();
        // We cannot use null_mut() here directly because we need a pointer to a pointer, the
        // pointer we point to is allowed to be null, but the pointer itself must be valid
        let return_ptr = (&mut null_mut()) as *mut *mut std::ffi::c_char;
        let error = unsafe {
            ffi::GRBgetstrattrelement(
                *constr.inner.0,
                attr_name.as_ptr(),
                constr.index() as std::ffi::c_int,
                return_ptr,
            )
        };
        constr.get_error(error).unwrap();
        let c_str: &CStr = unsafe { CStr::from_ptr(*return_ptr) };
        c_str.to_string_lossy().to_string()
    }
}
impl VariableGetter for GRBStrAttr {
    type Value = String;

    fn get(&self, var: &crate::prelude::GRBVar) -> Self::Value {
        let attr_name: &CStr = (*self).into();
        // We cannot use null_mut() here directly because we need a pointer to a pointer, the
        // pointer we point to is allowed to be null, but the pointer itself must be valid
        let value_p = (&mut null_mut()) as *mut *mut std::ffi::c_char;
        let error = unsafe {
            ffi::GRBgetstrattrelement(
                *var.inner.0,
                attr_name.as_ptr(),
                var.index() as std::ffi::c_int,
                value_p,
            )
        };
        var.get_error(error).unwrap();
        let c_str: &CStr = unsafe { CStr::from_ptr(*value_p) };
        c_str.to_string_lossy().to_string()
    }
}

impl VariableSetter for GRBStrAttr {
    type Value = String;

    fn set(&self, var: &crate::prelude::GRBVar, value: Self::Value) -> i32 {
        let attr_name: &CStr = (*self).into();
        let value =
            CString::new(value).expect("Failed to convert String to CString in `ModelSetter::set`");
        unsafe {
            ffi::GRBsetstrattrelement(
                *var.inner.0,
                attr_name.as_ptr(),
                var.index() as std::ffi::c_int,
                value.as_ptr(),
            )
        }
    }
}

impl ModelGetter for GRBStrAttr {
    type Value = String;

    fn get(&self, model: *mut ffi::GRBmodel) -> Self::Value {
        // We cannot use null_mut() here directly because we need a pointer to a pointer, the
        // pointer we point to is allowed to be null, but the pointer itself must be valid
        let value_p = (&mut null_mut()) as *mut *mut std::ffi::c_char;
        let attr_name: &CStr = (*self).into();
        let error = unsafe { ffi::GRBgetstrattr(model, attr_name.as_ptr(), value_p) };
        let c_str: &CStr = unsafe { CStr::from_ptr(*value_p) };
        c_str.to_string_lossy().to_string()
    }
}

impl ModelSetter for GRBStrAttr {
    type Value = String;

    fn set(&self, model: *mut ffi::GRBmodel, value: Self::Value) -> i32 {
        let attr_name: &CStr = (*self).into();
        let value =
            CString::new(value).expect("Failed to convert String to CString in `ModelSetter::set`");
        unsafe { ffi::GRBsetstrattr(model, attr_name.as_ptr(), value.as_ptr()) }
    }
}

impl<C> ModelGetterList<C> for GRBStrAttr
where
    C: IsModelingObject,
{
    type Value = String;

    fn get_list(&self, model: *mut ffi::GRBmodel, inds: Vec<C>) -> Vec<Self::Value> {
        let len = inds.len();
        let mut inds = inds
            .iter()
            .map(|c| c.index() as std::ffi::c_int)
            .collect::<Vec<_>>();
        let mut values = vec![null_mut(); len];
        let attr_name: &CStr = (*self).into();
        unsafe {
            ffi::GRBgetstrattrlist(
                model,
                attr_name.as_ptr(),
                len as std::ffi::c_int,
                inds.as_mut_ptr(),
                values.as_mut_ptr(),
            )
        };
        values
            .iter()
            .map(|&c| {
                let c_str: &CStr = unsafe { CStr::from_ptr(c) };
                c_str.to_string_lossy().to_string()
            })
            .collect::<Vec<_>>()
    }
}

// PERF:Cloning here might be avoidable
// FIX: Memory leak due to CString::into_raw() - need to convert back to CString and let it drop
impl<C> ModelSetterList<C> for GRBStrAttr
where
    C: IsModelingObject,
{
    type Value = String;

    fn set_list(&self, model: *mut ffi::GRBmodel, inds: Vec<C>, values: Vec<Self::Value>) -> i32 {
        let attr_name: &CStr = (*self).into();
        let len = values.len();
        let mut inds = inds
            .iter()
            .map(|c| c.index() as std::ffi::c_int)
            .collect::<Vec<_>>();
        let mut values = values
            .iter()
            .map(|s| {
                CString::new(s.clone())
                    .expect("Failed to convert String to CString in `ModelSetterList::set_list`")
                    .into_raw()
            })
            .collect::<Vec<_>>();
        unsafe {
            ffi::GRBsetstrattrlist(
                model,
                attr_name.as_ptr(),
                len as std::ffi::c_int,
                inds.as_mut_ptr(),
                values.as_mut_ptr(),
            )
        }
    }
}
impl From<GRBStrAttr> for &'static CStr {
    fn from(value: GRBStrAttr) -> &'static CStr {
        match value {
            GRBStrAttr::SCENNNAME => ffi::GRB_STR_ATTR_SCENNNAME,
            GRBStrAttr::OBJNNAME => ffi::GRB_STR_ATTR_OBJNNAME,
            GRBStrAttr::GENCONSTRNAME => ffi::GRB_STR_ATTR_GENCONSTRNAME,
            GRBStrAttr::QCNAME => ffi::GRB_STR_ATTR_QCNAME,
            GRBStrAttr::QCTAG => ffi::GRB_STR_ATTR_QCTAG,
            GRBStrAttr::CONSTRNAME => ffi::GRB_STR_ATTR_CONSTRNAME,
            GRBStrAttr::CTAG => ffi::GRB_STR_ATTR_CTAG,
            GRBStrAttr::VTAG => ffi::GRB_STR_ATTR_VTAG,
            GRBStrAttr::VARNAME => ffi::GRB_STR_ATTR_VARNAME,
            GRBStrAttr::MODELNAME => ffi::GRB_STR_ATTR_MODELNAME,
        }
    }
}

#[allow(clippy::upper_case_acronyms, non_camel_case_types)]
#[derive(Clone, Copy)]
pub enum GRBCharAttr {
    /// QC sense ('<', '>', or '=')
    QCSENSE,
    /// Sense ('<', '>', or '=')
    SENSE,
    /// Integrality type
    VTYPE,
}

impl From<GRBCharAttr> for &'static CStr {
    fn from(value: GRBCharAttr) -> &'static CStr {
        match value {
            GRBCharAttr::QCSENSE => ffi::GRB_CHAR_ATTR_QCSENSE,
            GRBCharAttr::SENSE => ffi::GRB_CHAR_ATTR_SENSE,
            GRBCharAttr::VTYPE => ffi::GRB_CHAR_ATTR_VTYPE,
        }
    }
}

impl ConstrSetter for GRBCharAttr {
    type Value = char;

    fn set(&self, constr: &crate::constr::GRBConstr, value: Self::Value) -> i32 {
        let attr_name: &CStr = (*self).into();
        let value = value as std::ffi::c_char;
        unsafe {
            ffi::GRBsetcharattrelement(
                *constr.inner.0,
                attr_name.as_ptr(),
                constr.index() as std::ffi::c_int,
                value,
            )
        }
    }
}

impl ConstrGetter for GRBCharAttr {
    type Value = char;

    fn get(&self, constr: &crate::prelude::GRBConstr) -> Self::Value {
        let attr_name: &CStr = (*self).into();
        let return_ptr = '\0' as std::ffi::c_char;
        let error = unsafe {
            ffi::GRBgetcharattrelement(
                *constr.inner.0,
                attr_name.as_ptr(),
                constr.index() as std::ffi::c_int,
                return_ptr as *mut std::ffi::c_char,
            )
        };
        constr.get_error(error).unwrap();
        return_ptr as u8 as char
    }
}
impl VariableGetter for GRBCharAttr {
    type Value = char;

    fn get(&self, var: &crate::prelude::GRBVar) -> Self::Value {
        let attr_name: &CStr = (*self).into();
        let return_ptr = '\0' as std::ffi::c_char;
        let error = unsafe {
            ffi::GRBgetcharattrelement(
                *var.inner.0,
                attr_name.as_ptr(),
                var.index() as std::ffi::c_int,
                return_ptr as *mut std::ffi::c_char,
            )
        };
        var.get_error(error).unwrap();
        return_ptr as u8 as char
    }
}

impl<C> ModelGetterList<C> for GRBCharAttr
where
    C: IsModelingObject,
{
    type Value = char;

    fn get_list(&self, model: *mut ffi::GRBmodel, inds: Vec<C>) -> Vec<Self::Value> {
        let len = inds.len();
        let mut inds = inds
            .iter()
            .map(|c| c.index() as std::ffi::c_int)
            .collect::<Vec<_>>();
        let mut values = vec!['\0' as std::ffi::c_char; len];
        let attr_name: &CStr = (*self).into();
        unsafe {
            ffi::GRBgetcharattrlist(
                model,
                attr_name.as_ptr(),
                len as std::ffi::c_int,
                inds.as_mut_ptr(),
                values.as_mut_ptr(),
            )
        };
        values.iter().map(|&c| c as u8 as char).collect()
    }
}

impl VariableSetter for GRBCharAttr {
    type Value = char;

    fn set(&self, var: &crate::prelude::GRBVar, value: Self::Value) -> i32 {
        let attr_name: &CStr = (*self).into();
        let value = value as std::ffi::c_char;
        unsafe {
            ffi::GRBsetcharattrelement(
                *var.inner.0,
                attr_name.as_ptr(),
                var.index() as std::ffi::c_int,
                value,
            )
        }
    }
}

impl<C> ModelSetterList<C> for GRBCharAttr
where
    C: IsModelingObject,
{
    type Value = char;

    fn set_list(&self, model: *mut ffi::GRBmodel, inds: Vec<C>, values: Vec<Self::Value>) -> i32 {
        let attr_name: &CStr = (*self).into();
        let len = values.len();
        let mut inds = inds
            .iter()
            .map(|c| c.index() as std::ffi::c_int)
            .collect::<Vec<_>>();
        let mut values = values
            .iter()
            .map(|c| *c as std::ffi::c_char)
            .collect::<Vec<_>>();
        unsafe {
            ffi::GRBsetcharattrlist(
                model,
                attr_name.as_ptr(),
                len as std::ffi::c_int,
                inds.as_mut_ptr(),
                values.as_mut_ptr(),
            )
        }
    }
}
// TODO: Add tests!
