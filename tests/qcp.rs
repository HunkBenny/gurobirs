use gurobirs::prelude::{
    Expr, GRBDblAttr, GRBEnv, GRBLinExpr, GRBModel, GRBModelSense, GRBStrAttr, GRBStrParam, GRBVar,
    GRBVarBuilder,
};

#[test]
fn test_qcp() {
    let env = GRBEnv::new(false, None).unwrap();
    let mut model = GRBModel::new(&env);
    let x = model.add_var(GRBVar::builder().name("x".to_owned()));
    let y = model.add_var(GRBVar::builder().name("y".to_owned()));
    let z = model.add_var(GRBVar::builder().name("z".to_owned()));
    let obj = GRBLinExpr::from(&x);
    model.set_objective(obj, GRBModelSense::MAXIMIZE);

    model.add_constr((&x + &y + &z).eq(1.0).name("c0"));

    model.add_qconstr((&x * &x + &y * &y).le(&z * &z).name("qc0"));
    model.add_qconstr((&x * &x).le(&y * &z).name("qc1"));

    model.optimize();
    println!("{} {}", x.get(GRBStrAttr::VARNAME), x.get(GRBDblAttr::X));
    println!("{} {}", y.get(GRBStrAttr::VARNAME), y.get(GRBDblAttr::X));
    println!("{} {}", z.get(GRBStrAttr::VARNAME), z.get(GRBDblAttr::X));

    println!("Obj: {}", model.get(GRBDblAttr::OBJVAL));
}
