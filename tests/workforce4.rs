use gurobirs::prelude::*;
use gurobirs_sys as ffi;

fn solve_and_print(
    model: &GRBModel,
    tot_slack: &GRBVar,
    n_workers: usize,
    workers: &[&str],
    tot_shifts: &[GRBVar],
) -> i32 {
    model.optimize();
    let status = model.get(GRBIntAttr::STATUS);

    if status == ffi::GRB_INF_OR_UNBD || status == ffi::GRB_INFEASIBLE || status == ffi::GRB_UNBOUNDED {
        println!("The model cannot be solved because it is infeasible or unbounded");
        return status;
    }
    if status != ffi::GRB_OPTIMAL {
        println!("Optimization was stopped with status {}", status);
        return status;
    }

    println!("\nTotal slack required: {}", tot_slack.get(GRBDblAttr::X));
    for w in 0..n_workers {
        println!("{} worked {} shifts", workers[w], tot_shifts[w].get(GRBDblAttr::X));
    }
    println!();

    status
}

#[test]
fn test_workforce4() {
    const N_SHIFTS: usize = 14;
    const N_WORKERS: usize = 7;

    let shifts = [
        "Mon1", "Tue2", "Wed3", "Thu4", "Fri5", "Sat6", "Sun7", "Mon8", "Tue9",
        "Wed10", "Thu11", "Fri12", "Sat13", "Sun14",
    ];
    let workers = ["Amy", "Bob", "Cathy", "Dan", "Ed", "Fred", "Gu"];

    let shift_requirements = [3, 2, 4, 4, 5, 6, 5, 2, 2, 3, 4, 6, 7, 5];

    let availability = [
        [0, 1, 1, 0, 1, 0, 1, 0, 1, 1, 1, 1, 1, 1],
        [1, 1, 0, 0, 1, 1, 0, 1, 0, 0, 1, 0, 1, 0],
        [0, 0, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1],
        [0, 1, 1, 0, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1],
        [1, 1, 1, 1, 1, 0, 1, 1, 1, 0, 1, 0, 1, 1],
        [1, 1, 1, 0, 0, 1, 0, 1, 1, 0, 0, 1, 1, 1],
        [1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
    ];

    let env = GRBEnv::new(false, None).unwrap();
    let model = GRBModel::new(&env);
    model.set(GRBStrAttr::MODELNAME, "assignment".to_string());

    // Assignment variables: x[w][s] == 1 if worker w is assigned to shift s.
    let mut x = Vec::with_capacity(N_WORKERS);
    for w in 0..N_WORKERS {
        x.push(Vec::with_capacity(N_SHIFTS));
        for s in 0..N_SHIFTS {
            let var = model.add_var(
                GRBVar::builder()
                    .ub(availability[w][s] as f64)
                    .vtype(GRBVarType::BINARY)
                    .name(format!("{}.{}", workers[w], shifts[s])),
            );
            x[w].push(var);
        }
    }

    // Slack variables for each shift constraint.
    let mut slacks = Vec::with_capacity(N_SHIFTS);
    for s in 0..N_SHIFTS {
        slacks.push(model.add_var(
            GRBVar::builder().name(format!("{}Slack", shifts[s])),
        ));
    }

    // Variable to represent the total slack.
    let tot_slack = model.add_var(GRBVar::builder().lb(0.0).name("totSlack".to_string()));

    // Variables to count the total shifts worked by each worker.
    let mut tot_shifts = Vec::with_capacity(N_WORKERS);
    for w in 0..N_WORKERS {
        tot_shifts.push(model.add_var(
            GRBVar::builder().name(format!("{}TotShifts", workers[w])),
        ));
    }

    // Constraint: assign exactly shiftRequirements[s] workers to each shift s.
    for s in 0..N_SHIFTS {
        let mut lhs = GRBLinExpr::new();
        lhs += &slacks[s];
        for w in 0..N_WORKERS {
            lhs += &x[w][s];
        }
        model.add_constr(lhs.eq(shift_requirements[s] as f64).name(shifts[s]));
    }

    // Constraint: set totSlack equal to the total slack.
    let mut lhs = GRBLinExpr::new();
    for s in 0..N_SHIFTS {
        lhs += &slacks[s];
    }
    model.add_constr(lhs.eq(&tot_slack).name("totSlack"));

    // Constraint: compute the total number of shifts for each worker.
    for w in 0..N_WORKERS {
        let mut lhs = GRBLinExpr::new();
        for s in 0..N_SHIFTS {
            lhs += &x[w][s];
        }
        model.add_constr(lhs.eq(&tot_shifts[w]).name(&format!("totShifts{}", workers[w])));
    }

    // Objective: minimize the total slack.
    model.set_objective(GRBLinExpr::from(&tot_slack), GRBModelSense::MINIMIZE);

    let status = solve_and_print(&model, &tot_slack, N_WORKERS, &workers, &tot_shifts);
    assert_eq!(status, ffi::GRB_OPTIMAL);

    // Constrain the slack by fixing its value to the found optimum.
    let slack_val = tot_slack.get(GRBDblAttr::X);
    tot_slack.set(GRBDblAttr::UB, slack_val);
    tot_slack.set(GRBDblAttr::LB, slack_val);

    // Variable to count the average number of shifts worked.
    let avg_shifts = model.add_var(GRBVar::builder().lb(0.0).name("avgShifts".to_string()));

    // Variables to count the difference from the average for each worker.
    let mut diff_shifts = Vec::with_capacity(N_WORKERS);
    for w in 0..N_WORKERS {
        diff_shifts.push(model.add_var(
            GRBVar::builder()
                .lb(-f64::INFINITY)
                .name(format!("{}Diff", workers[w])),
        ));
    }

    // Constraint: compute the average number of shifts worked.
    let mut lhs = GRBLinExpr::new();
    for w in 0..N_WORKERS {
        lhs += &tot_shifts[w];
    }
    model.add_constr(lhs.eq(N_WORKERS as f64 * &avg_shifts).name("avgShifts"));

    // Constraint: compute the difference from the average number of shifts.
    for w in 0..N_WORKERS {
        let mut lhs = GRBLinExpr::new();
        lhs += &tot_shifts[w];
        lhs -= &avg_shifts;
        model.add_constr(lhs.eq(&diff_shifts[w]).name(&format!("{}Diff", workers[w])));
    }

    // Objective: minimize the sum of the square of the difference from the
    // average number of shifts worked.
    let mut qobj = GRBQuadExpr::new();
    for w in 0..N_WORKERS {
        qobj = qobj + diff_shifts[w] * diff_shifts[w];
    }
    model.set_objective(qobj, GRBModelSense::MINIMIZE);

    let status = solve_and_print(&model, &tot_slack, N_WORKERS, &workers, &tot_shifts);
    assert_eq!(status, ffi::GRB_OPTIMAL);
}