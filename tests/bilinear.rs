use gurobirs::prelude::*;

// This example formulates and solves the following simple bilinear model:
//
//   maximize    x
//   subject to  x + y + z <= 10
//               x * y <= 2          (bilinear inequality)
//               x * z + y * z == 1  (bilinear equality)
//               x, y, z non-negative (x integral in second version)

#[test]
fn test_bilinear() {
    let env = GRBEnv::new(false, None).unwrap();
    let model = GRBModel::new(&env);

    // Create variables
    let x = model.add_var(GRBVar::builder().vtype(GRBVarType::CONTINUOUS).name("x".to_string()));
    let y = model.add_var(GRBVar::builder().vtype(GRBVarType::CONTINUOUS).name("y".to_string()));
    let z = model.add_var(GRBVar::builder().vtype(GRBVarType::CONTINUOUS).name("z".to_string()));

    // Set objective
    model.set_objective(GRBLinExpr::from(&x), GRBModelSense::MAXIMIZE);

    // Add linear constraint: x + y + z <= 10
    model.add_constr((&x + &y + &z).le(10.0).name("c0"));

    // Add bilinear inequality constraint: x * y <= 2
    model.add_qconstr((&x * &y).le(2.0).name("bilinear0"));

    // Add bilinear equality constraint: x * z + y * z == 1
    model.add_qconstr((&x * &z + &y * &z).eq(1.0).name("bilinear1"));

    // Optimize model
    model.optimize();

    println!("{} {}", x.get(GRBStrAttr::VARNAME), x.get(GRBDblAttr::X));
    println!("{} {}", y.get(GRBStrAttr::VARNAME), y.get(GRBDblAttr::X));
    println!("{} {}", z.get(GRBStrAttr::VARNAME), z.get(GRBDblAttr::X));

    // Constrain x to be integral and solve again
    let vt: i8 = GRBVarType::INTEGER.into();
    x.set(GRBCharAttr::VTYPE, vt as u8 as char);
    model.optimize();

    println!("{} {}", x.get(GRBStrAttr::VARNAME), x.get(GRBDblAttr::X));
    println!("{} {}", y.get(GRBStrAttr::VARNAME), y.get(GRBDblAttr::X));
    println!("{} {}", z.get(GRBStrAttr::VARNAME), z.get(GRBDblAttr::X));

    println!("Obj: {}", model.get(GRBDblAttr::OBJVAL));
}