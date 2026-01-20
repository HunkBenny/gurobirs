use gurobirs::prelude::*;
use gurobirs_sys::GRB_METHOD_BARRIER;

#[test]
fn test_facility_location() {
    // number of plants and warehouses
    let num_plants = 5;
    let num_warehouses = 4;

    // warehouse demand in thousands
    let demand = [15, 18, 14, 20];
    // plant capacity in thousands
    let capacity = [20, 22, 17, 19, 18];
    // fixed cost for opening a plants
    let fixed_cost = [12000, 15000, 17000, 13000, 16000];

    let transportation_costs = [
        vec![4000, 2000, 3000, 2500, 4500],
        vec![2500, 2600, 3400, 3000, 4000],
        vec![1200, 1800, 2600, 4100, 3000],
        vec![2200, 2600, 3100, 3700, 3200],
    ];
    // Create a new Gurobi environment
    let env = GRBenv::new(false, None).unwrap();
    let mut model = GRBModel::new(env);
    model.set(GRBStrAttr::MODELNAME, "facility_location".to_string());

    // plant open
    let mut open = Vec::new();
    for p in 0..num_plants {
        open.push(model.add_var(GRBVar::builder().vtype(GRBVarType::BINARY)));
        let vname = format!("open_{}", p);
        open[p].set(GRBStrAttr::VARNAME, vname);
        open[p].set(GRBDblAttr::OBJ, fixed_cost[p] as f64);
    }

    // transportation decision variables, how much goes from a plant to a warehouse
    let mut transport = Vec::new();
    for w in 0..num_warehouses {
        transport.push(Vec::new());
        for p in 0..num_plants {
            let var = model.add_var(GRBVar::builder());
            let vname = format!("trans_{}.{}", p, w);
            var.set(GRBDblAttr::OBJ, transportation_costs[w][p] as f64);
            var.set(GRBStrAttr::VARNAME, vname);
            transport[w].push(var);
        }
    }

    model.set(GRBIntAttr::MODELSENSE, GRBModelSense::MINIMIZE as i32);
    // Production capacity constraints
    // RHS is limited by choice of opening plant
    for p in 0..num_plants {
        let mut expr = GRBLinExpr::new();
        for w in 0..num_warehouses {
            expr += &transport[w][p];
        }
        expr += -(capacity[p] as f64) * &open[p];
        model.add_constr(expr.le(0.0).name(format!("cap_{}", p).as_str()));
    }
    // Demand fulfillment constraints
    for w in 0..num_warehouses {
        let mut expr = GRBLinExpr::new();
        for p in 0..num_plants {
            expr += &transport[w][p];
        }
        let cname = format!("demand_{}", w);
        model.add_constr(expr.eq(demand[w] as f64).name(cname.as_str()));
    }

    // We are going to pass a starting solution to gurobi;

    // First open all plants
    for p in 0..num_plants {
        open[p].set(GRBDblAttr::START, 1.0);
    }

    // Close plant w/ highest cost

    let plant_to_close = (0..num_plants)
        .max_by_key(|x| fixed_cost[*x])
        .expect("No plants");
    println!("Closing plant {}", plant_to_close);
    open[plant_to_close].set(GRBDblAttr::START, 0.0);

    // use barrier in root
    model.set(GRBIntParam::METHOD, GRB_METHOD_BARRIER);

    // solve
    model.optimize();

    // print solution
    println!("TOTAL COSTS: {}", model.get(GRBDblAttr::OBJVAL));
    println!("SOLUTION:");
    for p in 0..num_plants {
        if open[p].get(GRBDblAttr::X) > 0.5 {
            println!("Plant {} is open", p);
            for w in 0..num_warehouses {
                let shipped = transport[w][p].get(GRBDblAttr::X);
                if shipped > 0.0001 {
                    println!("  ships {} units to warehouse {}", shipped, w);
                }
            }
        } else {
            println!("Plant {} is closed", p);
        }
    }
}
