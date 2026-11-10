use gurobirs::prelude::*;
use gurobirs_sys as ffi;

#[test]
fn test_feasopt() {
    let env = GRBEnv::new(false, None).unwrap();
    let mut model = GRBModel::new(&env);

    // infeasible model: x <= 1 and x >= 2
    let x = model.add_var(GRBVar::builder().lb(0.0).ub(10.0).name("x".to_string()));
    let c1 = model.add_constr((0.0 + &x).le(1.0));
    let c2 = model.add_constr((0.0 + &x).ge(2.0));
    let constrs = vec![c1, c2];
    // objective defaults to 0

    model.optimize();
    let status = model.get(GRBIntAttr::STATUS);
    if status == ffi::GRB_OPTIMAL {
        println!("Original model is feasible");
    } else {
        println!("Original model is infeasible, relaxing constraints");
    }

    let orignumvars = model.get(GRBIntAttr::NUMVARS) as usize;

    // feasRelax: linear, minrelax true, vrelax false, crelax true
    let vars: Vec<GRBVar> = Vec::new();
    let lbpen: Vec<f64> = Vec::new();
    let ubpen: Vec<f64> = Vec::new();
    let mut rhspen = vec![1.0; constrs.len()];
    model.feas_relax(ffi::GRB_FEASRELAX_LINEAR, true, &vars, &lbpen, &ubpen, &constrs, &mut rhspen);
    model.optimize();
    let status = model.get(GRBIntAttr::STATUS);
    if status == ffi::GRB_OPTIMAL {
        println!("Relaxed model optimal, obj = {}", model.get(GRBDblAttr::OBJVAL));
    } else {
        println!("Relaxed model status {}", status);
        return;
    }

    println!("\nSlack values:");
    let numvars = model.get(GRBIntAttr::NUMVARS) as usize;
    for i in orignumvars..numvars {
        let v = model.get_var(i);
        if v.get(GRBDblAttr::X) > 1e-6 {
            println!("{} = {}", v.get(GRBStrAttr::VARNAME), v.get(GRBDblAttr::X));
        }
    }
}
