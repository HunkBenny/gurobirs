use gurobi_rust::prelude::{Expr, GRBModel, GRBModelSense, GRBVar, GRBVarType, GRBenv};

#[test]
fn test_build_model() {
    // first create env
    let env =
        GRBenv::new(false, None).expect("err nerrr sth happened when creating the environment");
    let mut model = GRBModel::new(env);

    let x = model.add_var(
        GRBVar::builder()
            .ub(10.0)
            .vtype(GRBVarType::INTEGER)
            .name("x".to_owned()),
    );
    let y = model.add_var(
        GRBVar::builder()
            .ub(5.0)
            .vtype(GRBVarType::INTEGER)
            .name("y".to_owned()),
    );

    let cons = x + y;
    model.add_constr(cons.le(6.0));
    let obj = 5.0 * x + 100.0 * y;
    model.set_objective(obj, GRBModelSense::MAXIMIZE);
    model.optimize();
}
