use crate::ffi;
use std::ffi::CStr;
pub trait Attribute {
    fn get(attribute: Self) -> &'static CStr;
}
enum GRBIntAttr {
    // number of MIP starts
    Numstart,
    // number of scenarios
    Numscenarios,
    // number of objectives
    Numobj,
    // status for a pass during the multi-objective solve
    Objpassnstatus,
    // optimization pass in which the selected objective function was processed
    Objnpass,
    // number of optimization passes during the multi-objective solve
    Numobjpasses,
    // priority
    Objnpriority,
    // Force general constr to be (1) or to not be (0) in final IIS
    IISGenconstrforce,
    // Force QConstr to be (1) or to not be (0) in final IIS
    IISQconstrforce,
    // Force SOS to be (1) or to not be (0) in final IIS
    IISSosforce,
    // Force constr to be (1) or to not be (0) in final IIS
    IISConstrforce,
    // Force var UB to be (1) or to not be (0) in final IIS
    IISUbforce,
    // Force var LB to be (1) or to not be (0) in final IIS
    IISLbforce,
    // Boolean: Is general constr in IIS?
    IISGenconstr,
    // Boolean: Is QConstr in IIS?
    IISQconstr,
    // Boolean: Is SOS in IIS?
    IISSos,
    // Boolean: Is constr in IIS?
    IISConstr,
    // Boolean: Is var UB in IIS?
    IISUb,
    // Boolean: Is var LB in IIS?
    IISLb,
    // Boolean: Is IIS Minimal?
    IISMinimal,
    // Constraint basis status
    Cbasis,
    // Variable basis status
    Vbasis,
    // method that solved LP using concurrent
    Concurrentwinmethod,
    // 0, no basis,
    // 1, has basis, so can be computed
    // 2, available
    Hasdualnorm,
    // Iters performed (NL barrier)
    Nlbaritercount,
    // Iters performed (barrier)
    Baritercount,
    // Status computed by barrier before crossover
    Barstatus,
    // # of solutions found
    Solcount,
    // Optimization status
    Status,
    // An option for PWL translation
    Funcnonlinear,
    // An option for PWL translation
    Funcpieces,
    // Type of general constraint
    Genconstrtype,
    // Lazy constraint?
    Lazy,
    // Ignore variable for solution identity check in solution pool
    Poolignore,
    // user specified variable partition
    Partition,
    // variable hint priority
    Varhintpri,
    // Convexity of variable PWL obj
    Pwlobjcvx,
    // MIP branch priority
    Branchpriority,
    // fingerprint computed from the model data and attributes influencing the optimization process
    Fingerprint,
    // number of tagged elements in model
    Numtagged,
    // License expiration date
    LicenseExpiration,
    // Model has multiple objectives?
    IsMultiobj,
    // Model has quadratic constr?
    IsQcp,
    // Is model a QP/MIQP (without Q/NL constraints)?
    IsQp,
    // Is model a MIP?
    IsMip,
    // 1=min, -1=max
    Modelsense,
    // # of variables with PWL obj.
    Numpwlobjvars,
    // # of binary vars
    Numbinvars,
    // # of integer vars
    Numintvars,
    // # of nz in q constraints
    Numqcnzs,
    // # of nz in Q
    Numqnzs,
    // # of nz in A
    Numnzs,
    // # of general constraints
    Numgenconstrs,
    // # of quadratic constraints
    Numqconstrs,
    // # of sos constraints
    Numsos,
    // # of vars
    Numvars,
    // # of constraints
    Numconstrs,
}

impl Attribute for GRBIntAttr {
    fn get(attribute: GRBIntAttr) -> &'static CStr {
        match attribute {
            GRBIntAttr::Numstart => ffi::GRB_INT_ATTR_NUMSTART,
            GRBIntAttr::Numscenarios => ffi::GRB_INT_ATTR_NUMSCENARIOS,
            GRBIntAttr::Numobj => ffi::GRB_INT_ATTR_NUMOBJ,
            GRBIntAttr::Objpassnstatus => ffi::GRB_INT_ATTR_OBJPASSNSTATUS,
            GRBIntAttr::Objnpass => ffi::GRB_INT_ATTR_OBJNPASS,
            GRBIntAttr::Numobjpasses => ffi::GRB_INT_ATTR_NUMOBJPASSES,
            GRBIntAttr::Objnpriority => ffi::GRB_INT_ATTR_OBJNPRIORITY,
            GRBIntAttr::IISGenconstrforce => ffi::GRB_INT_ATTR_IIS_GENCONSTRFORCE,
            GRBIntAttr::IISQconstrforce => ffi::GRB_INT_ATTR_IIS_QCONSTRFORCE,
            GRBIntAttr::IISSosforce => ffi::GRB_INT_ATTR_IIS_SOSFORCE,
            GRBIntAttr::IISConstrforce => ffi::GRB_INT_ATTR_IIS_CONSTRFORCE,
            GRBIntAttr::IISUbforce => ffi::GRB_INT_ATTR_IIS_UBFORCE,
            GRBIntAttr::IISLbforce => ffi::GRB_INT_ATTR_IIS_LBFORCE,
            GRBIntAttr::IISGenconstr => ffi::GRB_INT_ATTR_IIS_GENCONSTR,
            GRBIntAttr::IISQconstr => ffi::GRB_INT_ATTR_IIS_QCONSTR,
            GRBIntAttr::IISSos => ffi::GRB_INT_ATTR_IIS_SOS,
            GRBIntAttr::IISConstr => ffi::GRB_INT_ATTR_IIS_CONSTR,
            GRBIntAttr::IISUb => ffi::GRB_INT_ATTR_IIS_UB,
            GRBIntAttr::IISLb => ffi::GRB_INT_ATTR_IIS_LB,
            GRBIntAttr::IISMinimal => ffi::GRB_INT_ATTR_IIS_MINIMAL,
            GRBIntAttr::Cbasis => ffi::GRB_INT_ATTR_CBASIS,
            GRBIntAttr::Vbasis => ffi::GRB_INT_ATTR_VBASIS,
            GRBIntAttr::Concurrentwinmethod => ffi::GRB_INT_ATTR_CONCURRENTWINMETHOD,
            GRBIntAttr::Hasdualnorm => ffi::GRB_INT_ATTR_HASDUALNORM,
            GRBIntAttr::Nlbaritercount => ffi::GRB_INT_ATTR_NLBARITERCOUNT,
            GRBIntAttr::Baritercount => ffi::GRB_INT_ATTR_BARITERCOUNT,
            GRBIntAttr::Barstatus => ffi::GRB_INT_ATTR_BARSTATUS,
            GRBIntAttr::Solcount => ffi::GRB_INT_ATTR_SOLCOUNT,
            GRBIntAttr::Status => ffi::GRB_INT_ATTR_STATUS,
            GRBIntAttr::Funcnonlinear => ffi::GRB_INT_ATTR_FUNCNONLINEAR,
            GRBIntAttr::Funcpieces => ffi::GRB_INT_ATTR_FUNCPIECES,
            GRBIntAttr::Genconstrtype => ffi::GRB_INT_ATTR_GENCONSTRTYPE,
            GRBIntAttr::Lazy => ffi::GRB_INT_ATTR_LAZY,
            GRBIntAttr::Poolignore => ffi::GRB_INT_ATTR_POOLIGNORE,
            GRBIntAttr::Partition => ffi::GRB_INT_ATTR_PARTITION,
            GRBIntAttr::Varhintpri => ffi::GRB_INT_ATTR_VARHINTPRI,
            GRBIntAttr::Pwlobjcvx => ffi::GRB_INT_ATTR_PWLOBJCVX,
            GRBIntAttr::Branchpriority => ffi::GRB_INT_ATTR_BRANCHPRIORITY,
            GRBIntAttr::Fingerprint => ffi::GRB_INT_ATTR_FINGERPRINT,
            GRBIntAttr::Numtagged => ffi::GRB_INT_ATTR_NUMTAGGED,
            GRBIntAttr::LicenseExpiration => ffi::GRB_INT_ATTR_LICENSE_EXPIRATION,
            GRBIntAttr::IsMultiobj => ffi::GRB_INT_ATTR_IS_MULTIOBJ,
            GRBIntAttr::IsQcp => ffi::GRB_INT_ATTR_IS_QCP,
            GRBIntAttr::IsQp => ffi::GRB_INT_ATTR_IS_QP,
            GRBIntAttr::IsMip => ffi::GRB_INT_ATTR_IS_MIP,
            GRBIntAttr::Modelsense => ffi::GRB_INT_ATTR_MODELSENSE,
            GRBIntAttr::Numpwlobjvars => ffi::GRB_INT_ATTR_NUMPWLOBJVARS,
            GRBIntAttr::Numbinvars => ffi::GRB_INT_ATTR_NUMBINVARS,
            GRBIntAttr::Numintvars => ffi::GRB_INT_ATTR_NUMINTVARS,
            GRBIntAttr::Numqcnzs => ffi::GRB_INT_ATTR_NUMQCNZS,
            GRBIntAttr::Numqnzs => ffi::GRB_INT_ATTR_NUMQNZS,
            GRBIntAttr::Numnzs => ffi::GRB_INT_ATTR_NUMNZS,
            GRBIntAttr::Numgenconstrs => ffi::GRB_INT_ATTR_NUMGENCONSTRS,
            GRBIntAttr::Numqconstrs => ffi::GRB_INT_ATTR_NUMQCONSTRS,
            GRBIntAttr::Numsos => ffi::GRB_INT_ATTR_NUMSOS,
            GRBIntAttr::Numvars => ffi::GRB_INT_ATTR_NUMVARS,
            GRBIntAttr::Numconstrs => ffi::GRB_INT_ATTR_NUMCONSTRS,
        }
    }
}

enum GRBDblAttr {
    // Deprecated since v13 - use GRB_DBL_ATTR_POOLNX instead
    XN,
    // maximum amount of allocated memory (in GB) in master environment
    Maxmemused,
    // current amount of allocated memory (in GB) in master environment
    Memused,
    // objective value for scenario i
    Scennobjval,
    // objective bound for scenario i
    Scennobjbound,
    // solution value in scenario i
    Scennx,
    // right hand side in scenario i
    Scennrhs,
    // objective in scenario i
    Scennobj,
    // upper bound in scenario i
    Scennub,
    // lower bound in scenario i
    Scennlb,
    // work done for a pass during the multi-objective solve
    Objpassnwork,
    // runtime for a pass during the multi-objective solve
    Objpassnruntime,
    // number of unexplored nodes for a pass during the multi-objective solve
    Objpassnopennodecount,
    // objective value for a pass during the multi-objective solve
    Objpassnobjval,
    // objective bound for a pass during the multi-objective solve
    Objpassnobjbound,
    // number of explored nodes for a pass during the multi-objective solve
    Objpassnnodecount,
    // MIP gap for a pass during the multi-objective solve
    Objpassnmipgap,
    // simplex iteration count for a pass during the multi-objective solve
    Objpassnitercount,
    // absolute tolerance
    Objnabstol,
    // relative tolerance
    Objnreltol,
    // weight
    Objnweight,
    // constant term
    Objncon,
    // Solution objective for Multi-objectives, also depends on solutionnumber
    Objnval,
    // ith objective
    Objn,
    // Dual norm square
    Cdualnorm,
    // QC Constraint slack
    Qcslack,
    // Constraint slack
    Slack,
    // Dual value for QC
    Qcpi,
    // Dual value
    Pi,
    // Dual norm square
    Vdualnorm,
    // Reduced costs
    Rc,
    // Best barrier dual iterate
    Barpi,
    // Best barrier primal iterate
    Barx,
    // Deprecated since v13 - use GRB_DBL_ATTR_POOLNX instead
    Xn,
    // Alternate MIP solution, depends on solutionnumber
    Poolnx,
    // Solution value
    X,
    // Unexplored nodes (B&C)
    Opennodecount,
    // Nodes explored (B&C)
    Nodecount,
    // Iters performed (PDHG)
    Pdhgitercount,
    // Iters performed (simplex)
    Itercount,
    // MIP optimality gap
    Mipgap,
    // Deprecated since v13 - use GRB_DBL_ATTR_POOLNOBJVAL instead
    Poolobjval,
    // Solution objective, depends on solutionnumber
    Poolnobjval,
    // Best bound on pool solution
    Poolobjbound,
    // Continuous bound
    Objboundc,
    // Best bound on solution
    Objbound,
    // Solution objective
    Objval,
    // Work for optimization
    Work,
    // Run time for optimization
    Runtime,
    // Min (abs) rhs of Q
    MinQcrhs,
    // Max (abs) rhs of Q
    MaxQcrhs,
    // Min (abs) nz coeff in linear part of Q
    MinQclcoeff,
    // Max (abs) nz coeff in linear part of Q
    MaxQclcoeff,
    // Min (abs) obj coeff of quadratic part
    MinQobjCoeff,
    // Max (abs) obj coeff of quadratic part
    MaxQobjCoeff,
    // Min (abs) nz coeff in Q
    MinQccoeff,
    // Max (abs) nz coeff in Q
    MaxQccoeff,
    // Min (abs) rhs coeff
    MinRhs,
    // Max (abs) rhs coeff
    MaxRhs,
    // Min (abs) obj coeff
    MinObjCoeff,
    // Max (abs) obj coeff
    MaxObjCoeff,
    // Min (abs) var bd
    MinBound,
    // Max (abs) finite var bd
    MaxBound,
    // Min (abs) nz coeff in A
    MinCoeff,
    // Max (abs) nz coeff in A
    MaxCoeff,
    // An option for PWL translation
    Funcpieceratio,
    // An option for PWL translation
    Funcpiecelength,
    // An option for PWL translation
    Funcpieceerror,
    // QC RHS
    Qcrhs,
    // LP dual solution warm start
    Dstart,
    // RHS
    Rhs,
    // variable hint value
    Varhintval,
    // LP primal solution warm start
    Pstart,
    // MIP start value, depends on startnumber
    Start,
    // Objective coeff
    Obj,
    // Upper bound
    Ub,
    // Lower bound
    Lb,
    // Objective constant
    Objcon,
    // # of nz in A
    Dnumnzs,
}

impl Attribute for GRBDblAttr {
    fn get(attribute: GRBDblAttr) -> &'static CStr {
        match attribute {
            GRBDblAttr::Xn => ffi::GRB_DBL_ATTR_Xn,
            GRBDblAttr::Maxmemused => ffi::GRB_DBL_ATTR_MAXMEMUSED,
            GRBDblAttr::Memused => ffi::GRB_DBL_ATTR_MEMUSED,
            GRBDblAttr::Scennobjval => ffi::GRB_DBL_ATTR_SCENNOBJVAL,
            GRBDblAttr::Scennobjbound => ffi::GRB_DBL_ATTR_SCENNOBJBOUND,
            GRBDblAttr::Scennx => ffi::GRB_DBL_ATTR_SCENNX,
            GRBDblAttr::Scennrhs => ffi::GRB_DBL_ATTR_SCENNRHS,
            GRBDblAttr::Scennobj => ffi::GRB_DBL_ATTR_SCENNOBJ,
            GRBDblAttr::Scennub => ffi::GRB_DBL_ATTR_SCENNUB,
            GRBDblAttr::Scennlb => ffi::GRB_DBL_ATTR_SCENNLB,
            GRBDblAttr::Objpassnwork => ffi::GRB_DBL_ATTR_OBJPASSNWORK,
            GRBDblAttr::Objpassnruntime => ffi::GRB_DBL_ATTR_OBJPASSNRUNTIME,
            GRBDblAttr::Objpassnopennodecount => ffi::GRB_DBL_ATTR_OBJPASSNOPENNODECOUNT,
            GRBDblAttr::Objpassnobjval => ffi::GRB_DBL_ATTR_OBJPASSNOBJVAL,
            GRBDblAttr::Objpassnobjbound => ffi::GRB_DBL_ATTR_OBJPASSNOBJBOUND,
            GRBDblAttr::Objpassnnodecount => ffi::GRB_DBL_ATTR_OBJPASSNNODECOUNT,
            GRBDblAttr::Objpassnmipgap => ffi::GRB_DBL_ATTR_OBJPASSNMIPGAP,
            GRBDblAttr::Objpassnitercount => ffi::GRB_DBL_ATTR_OBJPASSNITERCOUNT,
            GRBDblAttr::Objnabstol => ffi::GRB_DBL_ATTR_OBJNABSTOL,
            GRBDblAttr::Objnreltol => ffi::GRB_DBL_ATTR_OBJNRELTOL,
            GRBDblAttr::Objnweight => ffi::GRB_DBL_ATTR_OBJNWEIGHT,
            GRBDblAttr::Objncon => ffi::GRB_DBL_ATTR_OBJNCON,
            GRBDblAttr::Objnval => ffi::GRB_DBL_ATTR_OBJNVAL,
            GRBDblAttr::Objn => ffi::GRB_DBL_ATTR_OBJN,
            GRBDblAttr::Cdualnorm => ffi::GRB_DBL_ATTR_CDUALNORM,
            GRBDblAttr::Qcslack => ffi::GRB_DBL_ATTR_QCSLACK,
            GRBDblAttr::Slack => ffi::GRB_DBL_ATTR_SLACK,
            GRBDblAttr::Qcpi => ffi::GRB_DBL_ATTR_QCPI,
            GRBDblAttr::Pi => ffi::GRB_DBL_ATTR_PI,
            GRBDblAttr::Vdualnorm => ffi::GRB_DBL_ATTR_VDUALNORM,
            GRBDblAttr::Rc => ffi::GRB_DBL_ATTR_RC,
            GRBDblAttr::Barpi => ffi::GRB_DBL_ATTR_BARPI,
            GRBDblAttr::Barx => ffi::GRB_DBL_ATTR_BARX,
            GRBDblAttr::XN => ffi::GRB_DBL_ATTR_XN,
            GRBDblAttr::Poolnx => ffi::GRB_DBL_ATTR_POOLNX,
            GRBDblAttr::X => ffi::GRB_DBL_ATTR_X,
            GRBDblAttr::Opennodecount => ffi::GRB_DBL_ATTR_OPENNODECOUNT,
            GRBDblAttr::Nodecount => ffi::GRB_DBL_ATTR_NODECOUNT,
            GRBDblAttr::Pdhgitercount => ffi::GRB_DBL_ATTR_PDHGITERCOUNT,
            GRBDblAttr::Itercount => ffi::GRB_DBL_ATTR_ITERCOUNT,
            GRBDblAttr::Mipgap => ffi::GRB_DBL_ATTR_MIPGAP,
            GRBDblAttr::Poolobjval => ffi::GRB_DBL_ATTR_POOLOBJVAL,
            GRBDblAttr::Poolnobjval => ffi::GRB_DBL_ATTR_POOLNOBJVAL,
            GRBDblAttr::Poolobjbound => ffi::GRB_DBL_ATTR_POOLOBJBOUND,
            GRBDblAttr::Objboundc => ffi::GRB_DBL_ATTR_OBJBOUNDC,
            GRBDblAttr::Objbound => ffi::GRB_DBL_ATTR_OBJBOUND,
            GRBDblAttr::Objval => ffi::GRB_DBL_ATTR_OBJVAL,
            GRBDblAttr::Work => ffi::GRB_DBL_ATTR_WORK,
            GRBDblAttr::Runtime => ffi::GRB_DBL_ATTR_RUNTIME,
            GRBDblAttr::MinQcrhs => ffi::GRB_DBL_ATTR_MIN_QCRHS,
            GRBDblAttr::MaxQcrhs => ffi::GRB_DBL_ATTR_MAX_QCRHS,
            GRBDblAttr::MinQclcoeff => ffi::GRB_DBL_ATTR_MIN_QCLCOEFF,
            GRBDblAttr::MaxQclcoeff => ffi::GRB_DBL_ATTR_MAX_QCLCOEFF,
            GRBDblAttr::MinQobjCoeff => ffi::GRB_DBL_ATTR_MIN_QOBJ_COEFF,
            GRBDblAttr::MaxQobjCoeff => ffi::GRB_DBL_ATTR_MAX_QOBJ_COEFF,
            GRBDblAttr::MinQccoeff => ffi::GRB_DBL_ATTR_MIN_QCCOEFF,
            GRBDblAttr::MaxQccoeff => ffi::GRB_DBL_ATTR_MAX_QCCOEFF,
            GRBDblAttr::MinRhs => ffi::GRB_DBL_ATTR_MIN_RHS,
            GRBDblAttr::MaxRhs => ffi::GRB_DBL_ATTR_MAX_RHS,
            GRBDblAttr::MinObjCoeff => ffi::GRB_DBL_ATTR_MIN_OBJ_COEFF,
            GRBDblAttr::MaxObjCoeff => ffi::GRB_DBL_ATTR_MAX_OBJ_COEFF,
            GRBDblAttr::MinBound => ffi::GRB_DBL_ATTR_MIN_BOUND,
            GRBDblAttr::MaxBound => ffi::GRB_DBL_ATTR_MAX_BOUND,
            GRBDblAttr::MinCoeff => ffi::GRB_DBL_ATTR_MIN_COEFF,
            GRBDblAttr::MaxCoeff => ffi::GRB_DBL_ATTR_MAX_COEFF,
            GRBDblAttr::Funcpieceratio => ffi::GRB_DBL_ATTR_FUNCPIECERATIO,
            GRBDblAttr::Funcpiecelength => ffi::GRB_DBL_ATTR_FUNCPIECELENGTH,
            GRBDblAttr::Funcpieceerror => ffi::GRB_DBL_ATTR_FUNCPIECEERROR,
            GRBDblAttr::Qcrhs => ffi::GRB_DBL_ATTR_QCRHS,
            GRBDblAttr::Dstart => ffi::GRB_DBL_ATTR_DSTART,
            GRBDblAttr::Rhs => ffi::GRB_DBL_ATTR_RHS,
            GRBDblAttr::Varhintval => ffi::GRB_DBL_ATTR_VARHINTVAL,
            GRBDblAttr::Pstart => ffi::GRB_DBL_ATTR_PSTART,
            GRBDblAttr::Start => ffi::GRB_DBL_ATTR_START,
            GRBDblAttr::Obj => ffi::GRB_DBL_ATTR_OBJ,
            GRBDblAttr::Ub => ffi::GRB_DBL_ATTR_UB,
            GRBDblAttr::Lb => ffi::GRB_DBL_ATTR_LB,
            GRBDblAttr::Objcon => ffi::GRB_DBL_ATTR_OBJCON,
            GRBDblAttr::Dnumnzs => ffi::GRB_DBL_ATTR_DNUMNZS,
        }
    }
}

enum GRBStrAttr {
    // name of scenario i
    Scennname,
    // name
    Objnname,
    // Name of general constraint
    Genconstrname,
    // QC name
    Qcname,
    // quadratic constraint tags
    Qctag,
    // Constraint name
    Constrname,
    // linear constraint tags
    Ctag,
    // variable tags
    Vtag,
    // Variable name
    Varname,
    // model name
    Modelname,
}

impl Attribute for GRBStrAttr {
    fn get(attribute: GRBStrAttr) -> &'static CStr {
        match attribute {
            GRBStrAttr::Scennname => ffi::GRB_STR_ATTR_SCENNNAME,
            GRBStrAttr::Objnname => ffi::GRB_STR_ATTR_OBJNNAME,
            GRBStrAttr::Genconstrname => ffi::GRB_STR_ATTR_GENCONSTRNAME,
            GRBStrAttr::Qcname => ffi::GRB_STR_ATTR_QCNAME,
            GRBStrAttr::Qctag => ffi::GRB_STR_ATTR_QCTAG,
            GRBStrAttr::Constrname => ffi::GRB_STR_ATTR_CONSTRNAME,
            GRBStrAttr::Ctag => ffi::GRB_STR_ATTR_CTAG,
            GRBStrAttr::Vtag => ffi::GRB_STR_ATTR_VTAG,
            GRBStrAttr::Varname => ffi::GRB_STR_ATTR_VARNAME,
            GRBStrAttr::Modelname => ffi::GRB_STR_ATTR_MODELNAME,
        }
    }
}

enum GRBCharAttr {
    // QC sense ('<', '>', or '=')
    Qcsense,
    // Sense ('<', '>', or '=')
    Sense,
    // Integrality type
    Vtype,
}

impl Attribute for GRBCharAttr {
    fn get(attribute: GRBCharAttr) -> &'static CStr {
        match attribute {
            GRBCharAttr::Qcsense => ffi::GRB_CHAR_ATTR_QCSENSE,
            GRBCharAttr::Sense => ffi::GRB_CHAR_ATTR_SENSE,
            GRBCharAttr::Vtype => ffi::GRB_CHAR_ATTR_VTYPE,
        }
    }
}

// TODO: Add tests!
