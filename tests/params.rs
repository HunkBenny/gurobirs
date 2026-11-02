use gurobirs::prelude::*;

#[test]
fn test_params() {
    let env = GRBEnv::new(false, None).unwrap();
    let model = GRBModel::new(&env);

    // small MIP
    let x = model.add_var(GRBVar::builder().lb(0.0).ub(1.0).vtype(GRBVarType::BINARY).name("x".to_string()));
    let y = model.add_var(GRBVar::builder().lb(0.0).ub(1.0).vtype(GRBVarType::BINARY).name("y".to_string()));
    let z = model.add_var(GRBVar::builder().lb(0.0).ub(1.0).vtype(GRBVarType::BINARY).name("z".to_string()));
    model.set_objective(&x + &y + 2.0 * &z, GRBModelSense::MAXIMIZE);
    model.add_constr((&x + 2.0 * &y + 3.0 * &z).le(4.0));
    model.add_constr((&x + &y).ge(1.0));

    if model.get(GRBIntAttr::IS_MIP) == 0 {
        println!("The model is not an integer program");
        return;
    }

    model.set(GRBDblParam::TIMELIMIT, 2.0);

    let mut best_gap = f64::INFINITY;
    let mut best_focus = 0i32;

    // baseline (default MIPFocus)
    model.reset();
    model.optimize();
    let gap = model.get(GRBDblAttr::MIPGAP);
    if gap < best_gap {
        best_gap = gap;
        best_focus = 0;
    }

    for i in 1..=3 {
        model.reset();
        model.set(GRBIntParam::MIPFOCUS, i);
        model.optimize();
        let gap = model.get(GRBDblAttr::MIPGAP);
        if gap < best_gap {
            best_gap = gap;
            best_focus = i;
        }
    }

    model.reset();
    if best_focus != 0 {
        model.set(GRBIntParam::MIPFOCUS, best_focus);
    }
    model.set(GRBDblParam::TIMELIMIT, f64::INFINITY);
    model.optimize();
    println!("Solved with MIPFocus: {}", best_focus);
}
