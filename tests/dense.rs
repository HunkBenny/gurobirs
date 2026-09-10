use gurobirs::prelude::*;

// This example formulates and solves the following simple QP model:
//
//   minimize    x + y + x^2 + x*y + y^2 + y*z + z^2
//   subject to  x + 2 y + 3 z >= 4
//               x +   y       >= 1
//               x, y, z non-negative
//
// The example illustrates the use of dense matrices to store A and Q
// (and dense vectors for the other relevant data).

#[test]
fn test_dense() {
    let rows = 2;
    let cols = 3;

    let c = [1.0, 1.0, 0.0];
    let q = [[1.0, 1.0, 0.0], [0.0, 1.0, 1.0], [0.0, 0.0, 1.0]];
    let a = [[1.0, 2.0, 3.0], [1.0, 1.0, 0.0]];
    let sense = ['>', '>'];
    let rhs = [4.0, 1.0];
    let lb = [0.0, 0.0, 0.0];

    let env = GRBEnv::new(false, None).unwrap();
    let model = GRBModel::new(&env);

    // Add variables to the model
    let mut vars = Vec::with_capacity(cols);
    for j in 0..cols {
        vars.push(
            model.add_var(
                GRBVar::builder()
                    .lb(lb[j])
                    .name(format!("x{}", j)),
            ),
        );
    }

    // Populate A matrix and add constraints
    for i in 0..rows {
        let mut lhs = GRBLinExpr::new();
        for j in 0..cols {
            if a[i][j] != 0.0 {
                lhs += a[i][j] * &vars[j];
            }
        }
        match sense[i] {
            '>' => model.add_constr(lhs.ge(rhs[i])),
            '<' => model.add_constr(lhs.le(rhs[i])),
            _ => model.add_constr(lhs.eq(rhs[i])),
        };
    }

    // Build the quadratic objective
    let mut obj_lin = GRBLinExpr::new();
    for j in 0..cols {
        obj_lin += c[j] * &vars[j];
    }
    let mut obj = GRBQuadExpr::new();
    obj = obj + obj_lin;
    for i in 0..cols {
        for j in i..cols {
            if q[i][j] != 0.0 {
                obj += q[i][j] * (vars[i] * vars[j]);
            }
        }
    }
    model.set_objective(obj, GRBModelSense::MINIMIZE);

    model.optimize();

    let mut success = false;
    let mut objval = 0.0;
    let mut solution = vec![0.0; cols];
    if model.get(GRBIntAttr::STATUS) == gurobirs_sys::GRB_OPTIMAL {
        objval = model.get(GRBDblAttr::OBJVAL);
        for i in 0..cols {
            solution[i] = vars[i].get(GRBDblAttr::X);
        }
        success = true;
    }

    println!(
        "optimal={} x: {} y: {} z: {}  obj: {}",
        success, solution[0], solution[1], solution[2], objval
    );
}