use gurobirs::prelude::{
    CallbackTrait, Expr, GRBCallback, GRBCallbackCodes, GRBCharAttr, GRBDblAttr, GRBEnv,
    GRBIntAttr, GRBIntParam, GRBLinExpr, GRBModel, GRBStrAttr, GRBVar, GRBVarType,
};

#[test]
fn test_tsp() {
    let n = 30;

    let (x, y) = {
        let mut x = Vec::with_capacity(n);
        let mut y = Vec::with_capacity(n);
        for i in 0..n {
            x.push(rand::random::<f64>());
            y.push(rand::random::<f64>());
        }
        (x, y)
    };

    let env = GRBEnv::new(false, None).unwrap();
    let mut model = GRBModel::new(&env);
    model.set(GRBIntParam::LAZYCONSTRAINTS, 1);

    // add vars
    let mut vars = vec![vec![None; n]; n];
    for i in 0..n {
        for j in 0..=i {
            let var = model.add_var(
                GRBVar::builder()
                    .vtype(GRBVarType::BINARY)
                    .obj(distance(&x, &y, i, j) / 2.0)
                    .name(format!("x_{}_{}", i, j)),
            );
            vars[i][j] = Some(var.clone());
            vars[j][i] = Some(var);
        }
    }
    let vars = vars
        .into_iter()
        .map(|row| row.into_iter().map(|v| v.unwrap()).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    // degree-2 constraints
    let mut ind = vec![0.0; n];
    let mut val = vec![1.0; n];
    for i in 0..n {
        let mut expr = GRBLinExpr::new();
        for j in 0..n {
            expr += &vars[i][j];
        }
        model.add_constr(expr.eq(2.0).name(format!("deg2_{}", i).as_str()));
    }

    // no edges back to node
    for i in 0..n {
        vars[i][i].set(GRBDblAttr::UB, 0.0);
    }

    let cb = Callback {
        vars: &vars,
        n: n as i32,
    };
    let wheres = 0 << GRBCallbackCodes::MIPSOL as u32;
    model.set_callback(&mut GRBCallback::new(cb), None);
    model.optimize();

    let sol_count = model.get(GRBIntAttr::SOLCOUNT);
    println!("Number of solutions: {}", sol_count);
    if sol_count > 0 {
        let solution = vars
            .iter()
            .map(|x_vec| {
                x_vec
                    .iter()
                    .map(|x| {
                        let val = x.get(GRBDblAttr::X);
                        val
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        let (tour, len) = find_subtour(n as i32, &solution);
        println!("Tour: ");
        println!("{:#?}", tour);
    }
}

struct Callback<'a> {
    vars: &'a Vec<Vec<GRBVar>>,
    n: i32,
}
impl CallbackTrait for Callback<'_> {
    fn callback(&mut self, mut cb_ctx: gurobirs::prelude::GRBCallbackContext) {
        if let GRBCallbackCodes::POLLING = cb_ctx.where_ {
            return;
        }
        match cb_ctx.where_ {
            GRBCallbackCodes::MIPSOL => {
                println!("Callback called with where = {}", cb_ctx.where_);
            }
            _ => {
                return;
            }
        }
        let x = cb_ctx.get_solutions(self.vars);
        let (tour, len) = find_subtour(self.n, &x);
        if len < self.n {
            let mut expr = GRBLinExpr::new();
            for i in 0..len {
                for j in i + 1..len {
                    let var = &self.vars[tour[i as usize] as usize][tour[j as usize] as usize];
                    let varname = var.get(GRBStrAttr::VARNAME);
                    expr += var;
                }
            }
            cb_ctx.add_lazy(expr.le((len - 1) as f64));
        }
    }
}

#[inline]
fn find_subtour(n: i32, sol: &[Vec<f64>]) -> (Vec<i32>, i32) {
    let mut tour = Vec::with_capacity(n as usize);
    for _ in 0..n {
        tour.push(-1);
    }
    let mut seen = {
        let mut seen = Vec::with_capacity(n as usize);
        for _ in 0..n {
            seen.push(false);
        }
        seen
    };
    let mut start = 0;
    let mut bestlen = n + 1;
    let mut bestind = -1;
    while start < n {
        let mut node = 0;
        while node < n as usize {
            if !seen[node] {
                break;
            }
            node += 1;
        }
        if node == n as usize {
            break;
        }
        let mut len = 0;
        while len < n {
            tour[start as usize + len as usize] = node as i32;
            seen[node] = true;
            let mut i = 0;
            while i < n {
                // node -> i
                if sol[node][i as usize] > 0.5 && !seen[i as usize] {}
                if sol[node][i as usize] > 0.5 && !seen[i as usize] {
                    node = i as usize;
                    break;
                }
                i += 1;
            }
            if i == n {
                len += 1;
                if len < bestlen {
                    bestlen = len;
                    bestind = start;
                }
                start += len;
                break;
            }
            len += 1;
        }
    }
    let mut output_tour = Vec::with_capacity(bestlen as usize);
    for i in 0..bestlen {
        output_tour.push(tour[bestind as usize + i as usize]);
    }
    (output_tour, bestlen)
}

#[inline]
fn distance(x: &[f64], y: &[f64], i: usize, j: usize) -> f64 {
    let dx = x[i] - x[j];
    let dy = y[i] - y[j];
    (dx * dx + dy * dy).sqrt()
}
