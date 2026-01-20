use gurobirs::prelude::*;

#[test]
fn test_mip1() {
    let env = GRBenv::new(false, None).unwrap();
    let mut model = GRBModel::new(env);
    let x = model.add_var(
        GRBVar::builder()
            .lb(0.0)
            .ub(1.0)
            .vtype(GRBVarType::BINARY)
            .name("x".to_string()),
    );
    let y = model.add_var(
        GRBVar::builder()
            .lb(0.0)
            .ub(1.0)
            .vtype(GRBVarType::BINARY)
            .name("y".to_string()),
    );
    let z = model.add_var(
        GRBVar::builder()
            .lb(0.0)
            .ub(1.0)
            .vtype(GRBVarType::BINARY)
            .name("z".to_string()),
    );
    model.set_objective(&x + &y + 2.0 * &z, GRBModelSense::MAXIMIZE);
    let cons1 = model.add_constr((&x + 2.0 * &y + 3.0 * &z).le(4.0).name("c0"));
    model.add_constr((&x + &y).ge(1.0).name("c1"));
    model.optimize();

    println!("{}: {}", x.get(GRBStrAttr::VARNAME), x.get(GRBDblAttr::X));
    println!("{}: {}", y.get(GRBStrAttr::VARNAME), y.get(GRBDblAttr::X));
    println!("{}: {}", z.get(GRBStrAttr::VARNAME), z.get(GRBDblAttr::X));
    println!(
        "{}: {}",
        cons1.get(GRBStrAttr::CONSTRNAME),
        cons1.get(GRBDblAttr::SLACK)
    );
    println!("Obj: {}", model.get(GRBDblAttr::OBJVAL));
}
