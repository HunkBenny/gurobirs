use gurobirs::prelude::*;

// This example formulates and solves the following simple model
// with PWL constraints:
//
//  maximize
//        sum c[j] * x[j]
//  subject to
//        sum A[i,j] * x[j] <= 0,  for i = 0, ..., m-1
//        sum y[j] <= 3
//        y[j] = pwl(x[j]),        for j = 0, ..., n-1
//        x[j] free, y[j] >= 0,    for j = 0, ..., n-1
//  where pwl(x) = 0,     if x  = 0
//               = 1+|x|, if x != 0

#[test]
fn test_gc_pwl() {
    let n = 5;
    let m = 5;
    let c = [0.5, 0.8, 0.5, 0.1, -1.0];
    let a = [
        [0.0, 0.0, 0.0, 1.0, -1.0],
        [0.0, 0.0, 1.0, 1.0, -1.0],
        [1.0, 1.0, 0.0, 0.0, -1.0],
        [1.0, 0.0, 1.0, 0.0, -1.0],
        [1.0, 0.0, 0.0, 1.0, -1.0],
    ];
    let npts = 5;
    let xpts = vec![-1.0, 0.0, 0.0, 0.0, 1.0];
    let ypts = vec![2.0, 1.0, 0.0, 1.0, 2.0];

    let env = GRBEnv::new(false, None).unwrap();
    let model = GRBModel::new(&env);
    model.set(GRBStrAttr::MODELNAME, "gc_pwl".to_string());

    // Add variables, set bounds and obj coefficients
    let mut x = Vec::with_capacity(n);
    for i in 0..n {
        x.push(
            model.add_var(
                GRBVar::builder()
                    .lb(-f64::INFINITY)
                    .obj(c[i])
                    .name(format!("x{}", i)),
            ),
        );
    }

    let mut y = Vec::with_capacity(n);
    for i in 0..n {
        y.push(model.add_var(GRBVar::builder().name(format!("y{}", i))));
    }

    // Set objective to maximize
    let mut obj = GRBLinExpr::new();
    for j in 0..n {
        obj += c[j] * &x[j];
    }
    model.set_objective(obj, GRBModelSense::MAXIMIZE);

    // Add linear constraints
    for i in 0..m {
        let mut le = GRBLinExpr::new();
        for j in 0..n {
            le += a[i][j] * &x[j];
        }
        model.add_constr(le.le(0.0));
    }

    let mut le1 = GRBLinExpr::new();
    for j in 0..n {
        le1 += &y[j];
    }
    model.add_constr(le1.le(3.0));

    // Add piecewise constraints
    for j in 0..n {
        model.add_genconstr_pwl(x[j], y[j], npts, xpts.clone(), ypts.clone(), "");
    }

    // Optimize model
    model.optimize();

    for j in 0..n {
        println!("x[{}] = {}", j, x[j].get(GRBDblAttr::X));
    }

    println!("Obj: {}", model.get(GRBDblAttr::OBJVAL));
}