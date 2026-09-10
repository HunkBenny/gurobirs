use gurobirs::prelude::*;

fn f(u: f64) -> f64 {
    u.exp()
}

fn g(u: f64) -> f64 {
    u.sqrt()
}

fn print_sol(model: &GRBModel, x: &GRBVar, y: &GRBVar, u: &GRBVar, v: &GRBVar) {
    println!("x = {}, u = {}", x.get(GRBDblAttr::X), u.get(GRBDblAttr::X));
    println!("y = {}, v = {}", y.get(GRBDblAttr::X), v.get(GRBDblAttr::X));
    println!("Obj = {}", model.get(GRBDblAttr::OBJVAL));

    // Calculate violation of exp(x) + 4 sqrt(y) <= 9
    let mut vio = f(x.get(GRBDblAttr::X)) + 4.0 * g(y.get(GRBDblAttr::X)) - 9.0;
    if vio < 0.0 {
        vio = 0.0;
    }
    println!("Vio = {}", vio);
}

// This example considers the following nonconvex nonlinear problem
//
//  maximize    2 x    + y
//  subject to  exp(x) + 4 sqrt(y) <= 9
//              x, y >= 0
//
// We show you the piecewise-linear approach to handle general function
// constraints (such as exp and sqrt):
//    a) Add two variables
//       u = exp(x)
//       v = sqrt(y)
//    b) Compute points (x, u) of u = exp(x) for some step length (e.g.,
//       x = 0, 1e-3, 2e-3, ..., xmax) and points (y, v) of v = sqrt(y) for
//       some step length.
//    c) Use the points to add two general constraints of type
//       piecewise-linear.

#[test]
fn test_gc_pwl_func() {
    let env = GRBEnv::new(false, None).unwrap();
    let model = GRBModel::new(&env);

    let lb = 0.0;
    let ub = f64::INFINITY;

    let x = model.add_var(GRBVar::builder().lb(lb).ub(ub).name("x".to_string()));
    let y = model.add_var(GRBVar::builder().lb(lb).ub(ub).name("y".to_string()));
    let u = model.add_var(GRBVar::builder().lb(lb).ub(ub).name("u".to_string()));
    let v = model.add_var(GRBVar::builder().lb(lb).ub(ub).name("v".to_string()));

    // Set objective
    model.set_objective(2 * &x + &y, GRBModelSense::MAXIMIZE);

    // Add linear constraint
    model.add_constr((&u + 4.0 * &v).le(9.0).name("l1"));

    // PWL constraint approach
    let intv = 1e-3;
    let xmax = 9.0f64.ln();
    let len = (xmax / intv).ceil() as usize + 1;
    let mut xpts = Vec::with_capacity(len);
    let mut upts = Vec::with_capacity(len);
    for i in 0..len {
        xpts.push(i as f64 * intv);
        upts.push(f(i as f64 * intv));
    }
    model.add_genconstr_pwl(x, u, len as i32, xpts, upts, "gc1");

    let ymax = (9.0 / 4.0) * (9.0 / 4.0);
    let len = (ymax / intv).ceil() as usize + 1;
    let mut ypts = Vec::with_capacity(len);
    let mut vpts = Vec::with_capacity(len);
    for i in 0..len {
        ypts.push(i as f64 * intv);
        vpts.push(g(i as f64 * intv));
    }
    model.add_genconstr_pwl(y, v, len as i32, ypts, vpts, "gc2");

    model.optimize();
    print_sol(&model, &x, &y, &u, &v);
}