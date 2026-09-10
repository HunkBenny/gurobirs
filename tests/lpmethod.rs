use gurobirs::prelude::*;

// Solve a model with different values of the Method parameter; show which
// value gives the shortest solve time. The published C++ example reads the
// model from a file; since the Rust wrapper has no model reader, we build a
// small LP inline instead.

#[test]
fn test_lpmethod() {
    let env = GRBEnv::new(false, None).unwrap();
    let model = GRBModel::new(&env);

    let x = model.add_var(GRBVar::builder().name("x".to_string()));
    let y = model.add_var(GRBVar::builder().name("y".to_string()));
    let z = model.add_var(GRBVar::builder().name("z".to_string()));

    model.set_objective(
        &x + &y + 2 * &z,
        GRBModelSense::MAXIMIZE,
    );

    model.add_constr((&x + 2.0 * &y + 3.0 * &z).le(4.0));
    model.add_constr((&x + &y).ge(1.0));

    // Solve the model with different values of Method
    let mut best_method = -1;
    let mut best_time = f64::INFINITY;
    for i in 0..=2 {
        model.set(GRBIntParam::METHOD, i);
        model.optimize();
        if model.get(GRBIntAttr::STATUS) == gurobirs_sys::GRB_OPTIMAL {
            best_time = model.get(GRBDblAttr::RUNTIME);
            best_method = i;
            // Reduce the TimeLimit parameter to save time with other methods
            model.set(GRBDblParam::TIMELIMIT, best_time);
        }
    }

    // Report which method was fastest
    if best_method == -1 {
        println!("Unable to solve this model");
    } else {
        println!(
            "Solved in {} seconds with Method: {}",
            best_time, best_method
        );
    }
}