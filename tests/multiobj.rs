use gurobirs::prelude::*;
use gurobirs_sys as ffi;

#[test]
fn test_multiobj() {
    let env = GRBEnv::new(false, None).unwrap();
    let model = GRBModel::new(&env);

    let ground_set_size = 20;
    let n_subsets = 4;
    let budget = 12;
    let set: [[f64; 20]; 4] = [
        [1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
        [0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0, 1.0],
        [0.0, 0.0, 0.0, 1.0, 1.0, 0.0, 1.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 0.0, 1.0, 1.0, 0.0, 0.0],
        [0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 0.0, 0.0],
    ];
    let set_obj_priority = [3, 2, 2, 1];
    let set_obj_weight = [1.0, 0.25, 1.25, 1.0];

    model.set(GRBStrAttr::MODELNAME, "multiobj".to_string());

    let mut elem = Vec::with_capacity(ground_set_size);
    for e in 0..ground_set_size {
        let v = model.add_var(GRBVar::builder().vtype(GRBVarType::BINARY).name(format!("El{}", e)));
        elem.push(v);
    }

    // budget constraint
    let mut lhs = GRBLinExpr::new();
    for e in 0..ground_set_size {
        lhs += &elem[e];
    }
    model.add_constr(lhs.le(budget as f64));

    model.set(GRBIntAttr::MODELSENSE, ffi::GRB_MAXIMIZE);

    model.set(GRBIntParam::POOLSOLUTIONS, 100);

    for i in 0..n_subsets {
        let mut objn = GRBLinExpr::new();
        for e in 0..ground_set_size {
            objn += set[i][e] * &elem[e];
        }
        model.set_objective_n(objn, i as i32, set_obj_priority[i], set_obj_weight[i], 1.0 + i as f64, 0.01);
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

    println!("Selected elements in best solution:");
    print!("    ");
    for e in 0..ground_set_size {
        if elem[e].get(GRBDblAttr::X) < 0.9 { continue; }
        print!(" El{}", e);
    }
    println!();

    let n_solutions = model.get(GRBIntAttr::SOLCOUNT);
    println!("Number of solutions found: {}", n_solutions);

    let n_print = n_solutions.min(10);
    println!("Objective values for first {} solutions:", n_print);
    for i in 0..n_subsets {
        model.set(GRBIntParam::OBJNUMBER, i as i32);
        print!("\tSet{}", i);
        for e in 0..n_print {
            model.set(GRBIntParam::SOLUTIONNUMBER, e);
            let val = model.get(GRBDblAttr::OBJNVAL);
            print!(" {:6}", val);
        }
        println!();
    }
}
