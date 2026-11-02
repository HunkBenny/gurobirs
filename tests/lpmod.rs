use gurobirs::prelude::*;
use gurobirs_sys as ffi;

#[test]
fn test_lpmod() {
    let env = GRBEnv::new(false, None).unwrap();
    let model = GRBModel::new(&env);

    // simple LP
    let x = model.add_var(GRBVar::builder().lb(0.0).ub(1.0).vtype(GRBVarType::CONTINUOUS).name("x".to_string()));
    let y = model.add_var(GRBVar::builder().lb(0.0).ub(1.0).vtype(GRBVarType::CONTINUOUS).name("y".to_string()));
    let z = model.add_var(GRBVar::builder().lb(0.0).ub(1.0).vtype(GRBVarType::CONTINUOUS).name("z".to_string()));
    let vars = vec![x, y, z];

    model.set_objective(&vars[0] + &vars[1] + 2.0 * &vars[2], GRBModelSense::MAXIMIZE);
    model.add_constr((&vars[0] + 2.0 * &vars[1] + 3.0 * &vars[2]).le(4.0));
    model.add_constr((&vars[0] + &vars[1]).ge(1.0));

    if model.get(GRBIntAttr::IS_MIP) != 0 {
        println!("The model is not a linear program");
        return;
    }

    model.optimize();
    let status = model.get(GRBIntAttr::STATUS);
    if status == ffi::GRB_INF_OR_UNBD || status == ffi::GRB_INFEASIBLE || status == ffi::GRB_UNBOUNDED {
        println!("The model cannot be solved because it is infeasible or unbounded");
        return;
    }
    if status != ffi::GRB_OPTIMAL {
        println!("Optimization was stopped with status {}", status);
        return;
    }

    // find smallest positive variable
    let mut min_val = f64::INFINITY;
    let mut min_var = 0usize;
    for (j, v) in vars.iter().enumerate() {
        let sol = v.get(GRBDblAttr::X);
        if sol > 0.0001 && sol < min_val && v.get(GRBDblAttr::LB) == 0.0 {
            min_val = sol;
            min_var = j;
        }
    }

    println!("\n*** Setting {} from {} to zero ***\n", vars[min_var].get(GRBStrAttr::VARNAME), min_val);
    vars[min_var].set(GRBDblAttr::UB, 0.0);

    // warm start
    model.optimize();
    let warm_count = model.get(GRBDblAttr::ITERCOUNT);
    let warm_time = model.get(GRBDblAttr::RUNTIME);

    // reset and cold solve
    println!("\n*** Resetting and solving without an advanced start ***\n");
    model.reset();
    model.optimize();
    let cold_count = model.get(GRBDblAttr::ITERCOUNT);
    let cold_time = model.get(GRBDblAttr::RUNTIME);

    println!("\n*** Warm start: {} iterations, {} seconds", warm_count, warm_time);
    println!("*** Cold start: {} iterations, {} seconds", cold_count, cold_time);
}
