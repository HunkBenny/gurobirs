use gurobirs::prelude::*;
use gurobirs_sys as ffi;

#[test]
fn test_fixanddive() {
    let env = GRBEnv::new(false, None).unwrap();
    let model = GRBModel::new(&env);

    // small MIP
    let x = model.add_var(GRBVar::builder().lb(0.0).ub(1.0).vtype(GRBVarType::BINARY).name("x".to_string()));
    let y = model.add_var(GRBVar::builder().lb(0.0).ub(1.0).vtype(GRBVarType::BINARY).name("y".to_string()));
    let z = model.add_var(GRBVar::builder().lb(0.0).ub(1.0).vtype(GRBVarType::BINARY).name("z".to_string()));
    let vars = vec![x, y, z];

    model.set_objective(&vars[0] + &vars[1] + 2.0 * &vars[2], GRBModelSense::MAXIMIZE);
    model.add_constr((&vars[0] + 2.0 * &vars[1] + 3.0 * &vars[2]).le(4.0));
    model.add_constr((&vars[0] + &vars[1]).ge(1.0));

    // collect integer vars and relax them
    let mut intvars: Vec<GRBVar> = Vec::new();
    for v in &vars {
        if v.get(GRBCharAttr::VTYPE) != ffi::GRB_CONTINUOUS as u8 as char {
            intvars.push(*v);
            v.set(GRBCharAttr::VTYPE, ffi::GRB_CONTINUOUS as u8 as char);
        }
    }

    model.set(GRBIntParam::OUTPUTFLAG, 0);
    model.optimize();

    for iter in 0..1000 {
        // fractional vars
        let mut fractional: Vec<GRBVar> = Vec::new();
        for v in &intvars {
            let sol = v.get(GRBDblAttr::X).abs();
            if (sol - (sol + 0.5).floor()).abs() > 1e-5 {
                fractional.push(*v);
            }
        }

        println!("Iteration {}, obj {}, fractional {}", iter, model.get(GRBDblAttr::OBJVAL), fractional.len());

        if fractional.is_empty() {
            println!("Found feasible solution - objective {}", model.get(GRBDblAttr::OBJVAL));
            break;
        }

        fractional.sort_by(|a, b| {
            let sol_a = a.get(GRBDblAttr::X).abs();
            let frac_a = (sol_a - (sol_a + 0.5).floor()).abs();
            let sol_b = b.get(GRBDblAttr::X).abs();
            let frac_b = (sol_b - (sol_b + 0.5).floor()).abs();
            frac_a.partial_cmp(&frac_b).unwrap()
        });

        let nfix = (fractional.len() / 4).max(1);
        for i in 0..nfix {
            let v = &fractional[i];
            let fixval = (v.get(GRBDblAttr::X) + 0.5).floor();
            println!("  Fix {} to {} ( rel {} )", v.get(GRBStrAttr::VARNAME), fixval, v.get(GRBDblAttr::X));
            v.set(GRBDblAttr::LB, fixval);
            v.set(GRBDblAttr::UB, fixval);
        }

        model.optimize();

        if model.get(GRBIntAttr::STATUS) != ffi::GRB_OPTIMAL {
            println!("Relaxation is infeasible");
            break;
        }
    }
}
