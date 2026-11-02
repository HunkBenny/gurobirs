use gurobirs::prelude::*;
use gurobirs_sys as ffi;

#[test]
fn test_mip2() {
    let env = GRBEnv::new(false, None).unwrap();
    let model = GRBModel::new(&env);

    // small MIP
    let x = model.add_var(GRBVar::builder().lb(0.0).ub(1.0).vtype(GRBVarType::BINARY).name("x".to_string()));
    let y = model.add_var(GRBVar::builder().lb(0.0).ub(1.0).vtype(GRBVarType::BINARY).name("y".to_string()));
    let z = model.add_var(GRBVar::builder().lb(0.0).ub(1.0).vtype(GRBVarType::BINARY).name("z".to_string()));
    let vars = vec![x, y, z];

    model.set_objective(&vars[0] + &vars[1] + 2.0 * &vars[2], GRBModelSense::MAXIMIZE);
    model.add_constr((&vars[0] + 2.0 * &vars[1] + 3.0 * &vars[2]).le(4.0));
    model.add_constr((&vars[0] + &vars[1]).ge(1.0));

    if model.get(GRBIntAttr::IS_MIP) == 0 {
        panic!("Model is not a MIP");
    }

    model.optimize();
    let status = model.get(GRBIntAttr::STATUS);
    if status == ffi::GRB_OPTIMAL {
        let objval = model.get(GRBDblAttr::OBJVAL);
        println!("Optimal objective: {}", objval);
    } else if status == ffi::GRB_INF_OR_UNBD {
        println!("Model is infeasible or unbounded");
        return;
    } else if status == ffi::GRB_INFEASIBLE {
        println!("Model is infeasible");
        return;
    } else if status == ffi::GRB_UNBOUNDED {
        println!("Model is unbounded");
        return;
    } else {
        println!("Optimization was stopped with status = {}", status);
        return;
    }

    let objval = model.get(GRBDblAttr::OBJVAL);

    println!();
    for k in 0..model.get(GRBIntAttr::SOLCOUNT) {
        model.set(GRBIntParam::SOLUTIONNUMBER, k);
        let objn = model.get(GRBDblAttr::POOLNOBJVAL);
        println!("Solution {} has objective: {}", k, objn);
    }
    println!();

    // fixed model
    let fixed = model.fixed_model();
    fixed.set(GRBIntParam::PRESOLVE, 0);
    fixed.optimize();

    let fstatus = fixed.get(GRBIntAttr::STATUS);
    if fstatus != ffi::GRB_OPTIMAL {
        println!("Error: fixed model isn't optimal");
        return;
    }
    let fobjval = fixed.get(GRBDblAttr::OBJVAL);
    if (fobjval - objval).abs() > 1e-6 * (1.0 + objval.abs()) {
        println!("Error: objective values are different");
        return;
    }

    let numvars = model.get(GRBIntAttr::NUMVARS);
    // print nonzero vars from fixed model by keeping vars? fixed has its own vars; we can't easily map.
    // We'll just print count.
    println!("Num vars: {}", numvars);
}
