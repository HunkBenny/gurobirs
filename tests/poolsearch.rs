use gurobirs::prelude::*;

// We find alternative epsilon-optimal solutions to a given knapsack
// problem by using PoolSearchMode.

#[test]
fn test_poolsearch() {
    let ground_set_size = 10;
    let obj_coef = [32.0, 32.0, 15.0, 15.0, 6.0, 6.0, 1.0, 1.0, 1.0, 1.0];
    let knapsack_coef = [16.0, 16.0, 8.0, 8.0, 4.0, 4.0, 2.0, 2.0, 1.0, 1.0];
    let budget = 33.0;

    let env = GRBEnv::new(false, None).unwrap();
    let model = GRBModel::new(&env);
    model.set(GRBStrAttr::MODELNAME, "poolsearch".to_string());

    // Initialize decision variables for ground set:
    // x[e] == 1 if element e is chosen
    let mut elem = Vec::with_capacity(ground_set_size);
    for e in 0..ground_set_size {
        elem.push(
            model.add_var(
                GRBVar::builder()
                    .vtype(GRBVarType::BINARY)
                    .obj(obj_coef[e])
                    .name(format!("El{}", e)),
            ),
        );
    }

    // Constraint: limit total number of elements to be picked to be at most
    // Budget
    let mut lhs = GRBLinExpr::new();
    for e in 0..ground_set_size {
        lhs += knapsack_coef[e] * &elem[e];
    }
    model.add_constr(lhs.le(budget).name("Budget"));

    // set global sense for ALL objectives
    model.set(GRBIntAttr::MODELSENSE, GRBModelSense::MAXIMIZE as i32);

    // Limit how many solutions to collect
    model.set(GRBIntParam::POOLSOLUTIONS, 1024);

    // Limit the search space by setting a gap for the worst possible solution
    // that will be accepted
    model.set(GRBDblParam::POOLGAP, 0.10);

    // do a systematic search for the k-best solutions
    model.set(GRBIntParam::POOLSEARCHMODE, 2);

    // Optimize
    model.optimize();

    // Status checking
    let status = model.get(GRBIntAttr::STATUS);
    assert!(status == gurobirs_sys::GRB_OPTIMAL);

    // Print best selected set
    println!("Selected elements in best solution:");
    for e in 0..ground_set_size {
        if elem[e].get(GRBDblAttr::X) < 0.9 {
            continue;
        }
        println!("\t El{}", e);
    }

    // Print number of solutions stored
    let n_solutions = model.get(GRBIntAttr::SOLCOUNT);
    println!("Number of solutions found: {}", n_solutions);

    // Print objective values of solutions
    for e in 0..n_solutions {
        model.set(GRBIntParam::SOLUTIONNUMBER, e);
        print!("{} ", model.get(GRBDblAttr::POOLNOBJVAL));
        if e % 15 == 14 {
            println!();
        }
    }
    println!();

    // print fourth best set if available
    if n_solutions >= 4 {
        model.set(GRBIntParam::SOLUTIONNUMBER, 3);

        println!("Selected elements in fourth best solution:");
        for e in 0..ground_set_size {
            if elem[e].get(GRBDblAttr::POOLNX) < 0.9 {
                continue;
            }
            println!("\t El{}", e);
        }
    }
}