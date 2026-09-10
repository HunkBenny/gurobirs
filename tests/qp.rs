use gurobirs::prelude::*;

fn vtype_char(vt: GRBVarType) -> char {
    let c: i8 = vt.into();
    c as u8 as char
}

// This example formulates and solves the following simple QP model:
//
//   minimize    x^2 + x*y + y^2 + y*z + z^2 + 2 x
//   subject to  x + 2 y + 3 z >= 4
//               x +   y       >= 1
//               x, y, z non-negative
//
// It solves it once as a continuous model, and once as an integer model.

#[test]
fn test_qp() {
    let env = GRBEnv::new(false, None).unwrap();
    let model = GRBModel::new(&env);

    // Create variables
    let x = model.add_var(GRBVar::builder().ub(1.0).vtype(GRBVarType::CONTINUOUS).name("x".to_string()));
    let y = model.add_var(GRBVar::builder().ub(1.0).vtype(GRBVarType::CONTINUOUS).name("y".to_string()));
    let z = model.add_var(GRBVar::builder().ub(1.0).vtype(GRBVarType::CONTINUOUS).name("z".to_string()));

    // Set objective
    let mut obj = GRBQuadExpr::new();
    obj = obj + &x * &x;
    obj = obj + &x * &y;
    obj = obj + &y * &y;
    obj = obj + &y * &z;
    obj = obj + &z * &z;
    obj = obj + 2 * &x;
    model.set_objective(obj, GRBModelSense::MINIMIZE);

    // Add constraint: x + 2 y + 3 z >= 4
    model.add_constr((&x + 2.0 * &y + 3.0 * &z).ge(4.0).name("c0"));

    // Add constraint: x + y >= 1
    model.add_constr((&x + &y).ge(1.0).name("c1"));

    // Optimize model
    model.optimize();

    println!("{} {}", x.get(GRBStrAttr::VARNAME), x.get(GRBDblAttr::X));
    println!("{} {}", y.get(GRBStrAttr::VARNAME), y.get(GRBDblAttr::X));
    println!("{} {}", z.get(GRBStrAttr::VARNAME), z.get(GRBDblAttr::X));

    println!("Obj: {}", model.get(GRBDblAttr::OBJVAL));

    // Change variable types to integer
    x.set(GRBCharAttr::VTYPE, vtype_char(GRBVarType::INTEGER));
    y.set(GRBCharAttr::VTYPE, vtype_char(GRBVarType::INTEGER));
    z.set(GRBCharAttr::VTYPE, vtype_char(GRBVarType::INTEGER));

    // Optimize model
    model.optimize();

    println!("{} {}", x.get(GRBStrAttr::VARNAME), x.get(GRBDblAttr::X));
    println!("{} {}", y.get(GRBStrAttr::VARNAME), y.get(GRBDblAttr::X));
    println!("{} {}", z.get(GRBStrAttr::VARNAME), z.get(GRBDblAttr::X));

    println!("Obj: {}", model.get(GRBDblAttr::OBJVAL));
}