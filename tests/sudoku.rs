use gurobirs::prelude::*;

// Sudoku example. The Sudoku board is a 9x9 grid, which is further divided
// into a 3x3 grid of 3x3 grids. Each cell in the grid must take a value from
// 0 to 9. No two grid cells in the same row, column, or 3x3 subgrid may take
// the same value.
//
// In the MIP formulation, binary variables x[i,j,v] indicate whether cell
// <i,j> takes value 'v'. The constraints are as follows:
//   1. Each cell must take exactly one value (sum_v x[i,j,v] = 1)
//   2. Each value is used exactly once per row (sum_i x[i,j,v] = 1)
//   3. Each value is used exactly once per column (sum_j x[i,j,v] = 1)
//   4. Each value is used exactly once per 3x3 subgrid

#[test]
fn test_sudoku() {
    const SD: usize = 3;
    const N: usize = SD * SD;

    // Pre-specified cells (from examples/data/sudoku)
    let input = [
        "9.7.8...6",
        "..54....1",
        "2....1...",
        "..3..26..",
        ".........",
        "..19..7..",
        "...7....8",
        "5....43..",
        "7...3.4.2",
    ];

    let env = GRBEnv::new(false, None).unwrap();
    let model = GRBModel::new(&env);

    let mut vars = vec![vec![vec![]; N]; N];
    for i in 0..N {
        for j in 0..N {
            for v in 0..N {
                vars[i][j].push(
                    model.add_var(
                        GRBVar::builder()
                            .vtype(GRBVarType::BINARY)
                            .ub(1.0)
                            .name(format!("G_{}_{}_{}", i, j, v)),
                    ),
                );
            }
        }
    }

    // Each cell must take one value
    for i in 0..N {
        for j in 0..N {
            let mut expr = GRBLinExpr::new();
            for v in 0..N {
                expr += &vars[i][j][v];
            }
            model.add_constr(expr.eq(1.0).name(&format!("V_{}_{}", i, j)));
        }
    }

    // Each value appears once per row
    for i in 0..N {
        for v in 0..N {
            let mut expr = GRBLinExpr::new();
            for j in 0..N {
                expr += &vars[i][j][v];
            }
            model.add_constr(expr.eq(1.0).name(&format!("R_{}_{}", i, v)));
        }
    }

    // Each value appears once per column
    for j in 0..N {
        for v in 0..N {
            let mut expr = GRBLinExpr::new();
            for i in 0..N {
                expr += &vars[i][j][v];
            }
            model.add_constr(expr.eq(1.0).name(&format!("C_{}_{}", j, v)));
        }
    }

    // Each value appears once per sub-grid
    for v in 0..N {
        for i0 in 0..SD {
            for j0 in 0..SD {
                let mut expr = GRBLinExpr::new();
                for i1 in 0..SD {
                    for j1 in 0..SD {
                        expr += &vars[i0 * SD + i1][j0 * SD + j1][v];
                    }
                }
                model
                    .add_constr(expr.eq(1.0).name(&format!("Sub_{}_{}_{}", v, i0, j0)));
            }
        }
    }

    // Fix variables associated with pre-specified cells
    for i in 0..N {
        let row: Vec<char> = input[i].chars().collect();
        for j in 0..N {
            let val = row[j] as i32 - 48 - 1; // 0-based
            if val >= 0 {
                vars[i][j][val as usize].set(GRBDblAttr::LB, 1.0);
            }
        }
    }

    // Optimize model
    model.optimize();

    // Print solution
    println!();
    for i in 0..N {
        for j in 0..N {
            for v in 0..N {
                if vars[i][j][v].get(GRBDblAttr::X) > 0.5 {
                    print!("{}", v + 1);
                }
            }
        }
        println!();
    }
    println!();
}