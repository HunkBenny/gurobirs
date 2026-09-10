use gurobirs::prelude::*;
use gurobirs_sys as ffi;

// This test sets up a callback that monitors optimization progress and
// implements a custom termination strategy (matching the C++ callback
// example). The published C++ example reads the model from a file; since
// the Rust wrapper has no model reader, we build a small knapsack MILP
// inline instead.
//
// The termination strategy stops the optimization of a MIP model once at
// least one of the following two conditions has been satisfied:
//   1) The optimality gap is less than 10%
//   2) At least 10000 nodes have been explored, and an integer feasible
//      solution has been found.

struct MyCallback<'a> {
    last_iter: f64,
    last_node: f64,
    num_vars: i32,
    vars: &'a Vec<GRBVar<'a>>,
}

impl CallbackTrait for MyCallback<'_> {
    fn callback(&mut self, cb_ctx: &mut gurobirs::prelude::GRBCallbackContext) {
        match cb_ctx.where_ {
            GRBCallbackCodes::POLLING => {
                // Ignore polling callback
            }
            GRBCallbackCodes::PRESOLVE => {
                // Presolve callback
                let cdels = cb_ctx.get(GRBWhatInt::PRE_COLDEL);
                let rdels = cb_ctx.get(GRBWhatInt::PRE_ROWDEL);
                if let (Ok(cdels), Ok(rdels)) = (cdels, rdels) {
                    if cdels != 0 || rdels != 0 {
                        println!("{} columns and {} rows are removed", cdels, rdels);
                    }
                }
            }
            GRBCallbackCodes::SIMPLEX => {
                // Simplex callback
                let itcnt = cb_ctx.get(GRBWhatDbl::SPX_ITRCNT);
                if let Ok(itcnt) = itcnt {
                    if itcnt - self.last_iter >= 100.0 {
                        self.last_iter = itcnt;
                        let obj = cb_ctx.get(GRBWhatDbl::SPX_OBJVAL);
                        let ispert = cb_ctx.get(GRBWhatInt::SPX_ISPERT);
                        let pinf = cb_ctx.get(GRBWhatDbl::SPX_PRIMINF);
                        let dinf = cb_ctx.get(GRBWhatDbl::SPX_DUALINF);
                        let obj = obj.unwrap_or(-1.0);
                        let ispert = ispert.unwrap_or(0);
                        let pinf = pinf.unwrap_or(-1.0);
                        let dinf = dinf.unwrap_or(-1.0);
                        let ch = if ispert == 0 {
                            ' '
                        } else if ispert == 1 {
                            'S'
                        } else {
                            'P'
                        };
                        println!("{} {} {} {} {}", itcnt, obj, ch, pinf, dinf);
                    }
                }
            }
            GRBCallbackCodes::MIP => {
                // General MIP callback
                let nodecnt = cb_ctx.get(GRBWhatDbl::MIP_NODCNT);
                let objbst = cb_ctx.get(GRBWhatDbl::MIP_OBJBST);
                let objbnd = cb_ctx.get(GRBWhatDbl::MIP_OBJBND);
                let solcnt = cb_ctx.get(GRBWhatInt::MIP_SOLCNT);
                if let (Ok(nodecnt), Ok(objbst), Ok(objbnd), Ok(solcnt)) =
                    (nodecnt, objbst, objbnd, solcnt)
                {
                    if nodecnt - self.last_node >= 100.0 {
                        self.last_node = nodecnt;
                        let actnodes = cb_ctx.get(GRBWhatDbl::MIP_NODLFT).unwrap_or(-1.0);
                        let itcnt = cb_ctx.get(GRBWhatDbl::MIP_ITRCNT).unwrap_or(-1.0);
                        let cutcnt = cb_ctx.get(GRBWhatInt::MIP_CUTCNT).unwrap_or(0);
                        println!(
                            "{} {} {} {} {} {} {}",
                            nodecnt, actnodes, itcnt, objbst, objbnd, solcnt, cutcnt
                        );
                    }
                    if (objbst - objbnd).abs() < 0.1 * (1.0 + objbst.abs()) {
                        println!("Stop early - 10% gap achieved");
                        cb_ctx.abort();
                    }
                    if nodecnt >= 10000.0 && solcnt != 0 {
                        println!("Stop early - 10000 nodes explored");
                        cb_ctx.abort();
                    }
                }
            }
            GRBCallbackCodes::MIPSOL => {
                // MIP solution callback
                let nodecnt = cb_ctx.get(GRBWhatDbl::MIPSOL_NODCNT);
                let obj = cb_ctx.get(GRBWhatDbl::MIPSOL_OBJ);
                let solcnt = cb_ctx.get(GRBWhatInt::MIPSOL_SOLCNT);
                if let (Ok(nodecnt), Ok(obj), Ok(solcnt)) = (nodecnt, obj, solcnt) {
                    let x = cb_ctx.get_solutions(self.vars);
                    println!(
                        "**** New solution at node {}, obj {}, sol {}, x[0] = {} ****",
                        nodecnt, obj, solcnt, x[0]
                    );
                }
            }
            GRBCallbackCodes::MIPNODE => {
                // MIP node callback
                println!("**** New node ****");
                let status = cb_ctx.get(GRBWhatInt::MIPNODE_STATUS);
                if let Ok(status) = status {
                    if status == ffi::GRB_OPTIMAL {
                        let node_vals = cb_ctx.get_noderels(self.vars.clone());
                        cb_ctx.set_solutions(self.vars, node_vals);
                    }
                }
            }
            GRBCallbackCodes::BARRIER => {
                // Barrier callback
                let itcnt = cb_ctx.get(GRBWhatInt::BARRIER_ITRCNT);
                let primobj = cb_ctx.get(GRBWhatDbl::BARRIER_PRIMOBJ);
                let dualobj = cb_ctx.get(GRBWhatDbl::BARRIER_DUALOBJ);
                let priminf = cb_ctx.get(GRBWhatDbl::BARRIER_PRIMINF);
                let dualinf = cb_ctx.get(GRBWhatDbl::BARRIER_DUALINF);
                let cmpl = cb_ctx.get(GRBWhatDbl::BARRIER_COMPL);
                if let (Ok(itcnt), Ok(primobj), Ok(dualobj), Ok(priminf), Ok(dualinf), Ok(cmpl)) =
                    (itcnt, primobj, dualobj, priminf, dualinf, cmpl)
                {
                    println!(
                        "{} {} {} {} {} {}",
                        itcnt, primobj, dualobj, priminf, dualinf, cmpl
                    );
                }
            }
            GRBCallbackCodes::MESSAGE => {
                // Message callback
                if let Ok(msg) = cb_ctx.get(GRBWhatString::MSG_STRING) {
                    print!("{}", msg);
                }
            }
            _ => {}
        }
    }
}

#[test]
fn test_callback() {
    // Build a knapsack MILP inline (substituted for model.read of the C++
    // example)
    let ground_set_size = 20;
    let obj_coef = [
        32.0, 32.0, 15.0, 15.0, 6.0, 6.0, 1.0, 1.0, 1.0, 1.0, 3.0, 3.0, 7.0, 7.0, 4.0, 4.0, 2.0,
        2.0, 5.0, 5.0,
    ];
    let knapsack_coef = [
        16.0, 16.0, 8.0, 8.0, 4.0, 4.0, 2.0, 2.0, 1.0, 1.0, 11.0, 11.0, 9.0, 9.0, 13.0, 13.0,
        6.0, 6.0, 10.0, 10.0,
    ];

    let env = GRBEnv::new(false, None).unwrap();
    let model = GRBModel::new(&env);

    // Turn off display and heuristics
    model.set(GRBIntParam::OUTPUTFLAG, 0);
    model.set(GRBDblParam::HEURISTICS, 0.0);

    let mut vars = Vec::with_capacity(ground_set_size);
    for e in 0..ground_set_size {
        vars.push(
            model.add_var(
                GRBVar::builder()
                    .vtype(GRBVarType::BINARY)
                    .obj(obj_coef[e])
                    .name(format!("El{}", e)),
            ),
        );
    }

    let mut lhs = GRBLinExpr::new();
    for e in 0..ground_set_size {
        lhs += knapsack_coef[e] * &vars[e];
    }
    model.add_constr(lhs.le(33.0));
    model.set(GRBIntAttr::MODELSENSE, GRBModelSense::MAXIMIZE as i32);

    // Create a callback object and associate it with the model
    let num_vars = model.get(GRBIntAttr::NUMVARS);
    let cb = MyCallback {
        last_iter: -f64::INFINITY,
        last_node: -f64::INFINITY,
        num_vars,
        vars: &vars,
    };
    model.set_callback(&mut GRBCallback::new(cb), None);

    // Solve model and capture solution information
    model.optimize();

    println!();
    println!("Optimization complete");
    if model.get(GRBIntAttr::SOLCOUNT) == 0 {
        println!(
            "No solution found, optimization status = {}",
            model.get(GRBIntAttr::STATUS)
        );
    } else {
        println!(
            "Solution found, objective = {}",
            model.get(GRBDblAttr::OBJVAL)
        );
        for j in 0..num_vars {
            let x = vars[j as usize].get(GRBDblAttr::X);
            if x != 0.0 {
                println!("{} {}", vars[j as usize].get(GRBStrAttr::VARNAME), x);
            }
        }
    }
}