use gurobirs::prelude::*;

// In this example we show the use of general constraints for modeling
// some common expressions. We use as an example a SAT-problem where we
// want to see if it is possible to satisfy at least four (or all) clauses
// of the logical form
//
// L = (x0 or ~x1 or x2)  and (x1 or ~x2 or x3)  and
//     (x2 or ~x3 or x0)  and (x3 or ~x0 or x1)  and
//     (~x0 or ~x1 or x2) and (~x1 or ~x2 or x3) and
//     (~x2 or ~x3 or x0) and (~x3 or ~x0 or x1)
//
// with objective maximize Obj0 + Obj1, where
//   Obj0 = MIN(Clause1, ..., Clause8)
//   Obj1 = 1 -> Clause1 + ... + Clause8 >= 4

#[test]
fn test_genconstr() {
    const N: usize = 4;
    const NLITERALS: usize = 4;
    const NCLAUSES: usize = 8;
    const NOBJ: usize = 2;

    // Example data: {0, n+1, 2} means clause (x0 or ~x1 or x2)
    let clauses: [[i32; 3]; NCLAUSES] = [
        [0, N as i32 + 1, 2],
        [1, N as i32 + 2, 3],
        [2, N as i32 + 3, 0],
        [3, N as i32, 1],
        [N as i32, N as i32 + 1, 2],
        [N as i32 + 1, N as i32 + 2, 3],
        [N as i32 + 2, N as i32 + 3, 0],
        [N as i32 + 3, N as i32, 1],
    ];

    let env = GRBEnv::new(false, None).unwrap();
    let model = GRBModel::new(&env);
    model.set(GRBStrAttr::MODELNAME, "genconstr".to_string());

    // Initialize decision variables and objective
    let mut lit = Vec::with_capacity(NLITERALS);
    let mut not_lit = Vec::with_capacity(NLITERALS);
    for i in 0..NLITERALS {
        lit.push(
            model.add_var(
                GRBVar::builder()
                    .vtype(GRBVarType::BINARY)
                    .name(format!("X{}", i)),
            ),
        );
        not_lit.push(
            model.add_var(
                GRBVar::builder()
                    .vtype(GRBVarType::BINARY)
                    .name(format!("notX{}", i)),
            ),
        );
    }

    let mut cla = Vec::with_capacity(NCLAUSES);
    for i in 0..NCLAUSES {
        cla.push(
            model.add_var(
                GRBVar::builder()
                    .vtype(GRBVarType::BINARY)
                    .name(format!("Clause{}", i)),
            ),
        );
    }

    let mut obj = Vec::with_capacity(NOBJ);
    for i in 0..NOBJ {
        obj.push(
            model.add_var(
                GRBVar::builder()
                    .ub(1.0)
                    .obj(1.0)
                    .vtype(GRBVarType::BINARY)
                    .name(format!("Obj{}", i)),
            ),
        );
    }

    // Link Xi and notXi
    for i in 0..NLITERALS {
        let lhs = &lit[i] + &not_lit[i];
        model.add_constr(lhs.eq(1.0).name(&format!("CNSTR_X{}", i)));
    }

    // Link clauses and literals
    for i in 0..NCLAUSES {
        let mut clause = Vec::with_capacity(3);
        for j in 0..3 {
            if clauses[i][j] >= N as i32 {
                clause.push(not_lit[(clauses[i][j] - N as i32) as usize]);
            } else {
                clause.push(lit[clauses[i][j] as usize]);
            }
        }
        model.add_genconstr_or(cla[i], clause, &format!("CNSTR_Clause{}", i));
    }

    // Link objs with clauses
    model.add_genconstr_min(obj[0], cla.clone(), f64::INFINITY, "CNSTR_Obj0");

    let mut lhs = GRBLinExpr::new();
    for i in 0..NCLAUSES {
        lhs += &cla[i];
    }
    model.add_genconstr_indicator(obj[1], 1, lhs.ge(4.0).name("CNSTR_Obj1"));

    // Set global objective sense
    model.set_objective(
        GRBLinExpr::from(&obj[0]) + GRBLinExpr::from(&obj[1]),
        GRBModelSense::MAXIMIZE,
    );

    // Optimize
    model.optimize();

    // Status checking
    let status = model.get(GRBIntAttr::STATUS);

    if status == gurobirs_sys::GRB_INF_OR_UNBD
        || status == gurobirs_sys::GRB_INFEASIBLE
        || status == gurobirs_sys::GRB_UNBOUNDED
    {
        println!("The model cannot be solved because it is infeasible or unbounded");
        return;
    }
    assert_eq!(status, gurobirs_sys::GRB_OPTIMAL);

    // Print result
    let objval = model.get(GRBDblAttr::OBJVAL);

    if objval > 1.9 {
        println!("Logical expression is satisfiable");
    } else if objval > 0.9 {
        println!("At least four clauses can be satisfied");
    } else {
        println!("Not even three clauses can be satisfied");
    }
}