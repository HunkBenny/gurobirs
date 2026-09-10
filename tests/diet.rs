use gurobirs::prelude::*;
use gurobirs_sys::GRB_OPTIMAL;

fn print_solution(model: &GRBModel, n_foods: usize, nutrition: &[GRBVar], buy: &[GRBVar]) {
    if model.get(GRBIntAttr::STATUS) == GRB_OPTIMAL {
        println!("\nCost: {}", model.get(GRBDblAttr::OBJVAL));
        println!("\nBuy:");
        for j in 0..n_foods {
            if buy[j].get(GRBDblAttr::X) > 0.0001 {
                println!("{} {}", buy[j].get(GRBStrAttr::VARNAME), buy[j].get(GRBDblAttr::X));
            }
        }
        println!("\nNutrition:");
        for i in 0..nutrition.len() {
            println!(
                "{} {}",
                nutrition[i].get(GRBStrAttr::VARNAME),
                nutrition[i].get(GRBDblAttr::X)
            );
        }
    } else {
        println!("No solution");
    }
}

#[test]
fn test_diet() {
    const N_CATEGORIES: usize = 4;
    let categories = ["calories", "protein", "fat", "sodium"];
    let min_nutrition = [1800.0, 91.0, 0.0, 0.0];
    let max_nutrition = [2200.0, f64::INFINITY, 65.0, 1779.0];

    const N_FOODS: usize = 9;
    let foods = [
        "hamburger", "chicken", "hot dog", "fries", "macaroni", "pizza", "salad", "milk",
        "ice cream",
    ];
    let cost = [2.49, 2.89, 1.50, 1.89, 2.09, 1.99, 2.49, 0.89, 1.59];

    let nutrition_values = [
        [410.0, 24.0, 26.0, 730.0],
        [420.0, 32.0, 10.0, 1190.0],
        [560.0, 20.0, 32.0, 1800.0],
        [380.0, 4.0, 19.0, 270.0],
        [320.0, 12.0, 10.0, 930.0],
        [320.0, 15.0, 12.0, 820.0],
        [320.0, 31.0, 12.0, 1230.0],
        [100.0, 8.0, 2.5, 125.0],
        [330.0, 8.0, 10.0, 180.0],
    ];

    let env = GRBEnv::new(false, None).unwrap();
    let model = GRBModel::new(&env);
    model.set(GRBStrAttr::MODELNAME, "diet".to_string());

    // Decision variables for the nutrition information, limited via bounds.
    let mut nutrition = Vec::with_capacity(N_CATEGORIES);
    for i in 0..N_CATEGORIES {
        nutrition.push(model.add_var(
            GRBVar::builder()
                .lb(min_nutrition[i])
                .ub(max_nutrition[i])
                .name(categories[i].to_string()),
        ));
    }

    // Decision variables for the foods to buy, with the objective coefficient
    // set during creation.
    let mut buy = Vec::with_capacity(N_FOODS);
    for j in 0..N_FOODS {
        buy.push(model.add_var(
            GRBVar::builder()
                .lb(0.0)
                .obj(cost[j])
                .name(foods[j].to_string()),
        ));
    }

    model.set(GRBIntAttr::MODELSENSE, GRBModelSense::MINIMIZE as i32);

    // Nutrition constraints.
    for i in 0..N_CATEGORIES {
        let mut ntot = GRBLinExpr::new();
        for j in 0..N_FOODS {
            ntot += nutrition_values[j][i] * &buy[j];
        }
        model.add_constr(ntot.eq(&nutrition[i]).name(categories[i]));
    }

    model.optimize();
    print_solution(&model, N_FOODS, &nutrition, &buy);

    println!("\nAdding constraint: at most 6 servings of dairy");
    model.add_constr((&buy[7] + &buy[8]).le(6.0).name("limit_dairy"));

    model.optimize();
    print_solution(&model, N_FOODS, &nutrition, &buy);
}