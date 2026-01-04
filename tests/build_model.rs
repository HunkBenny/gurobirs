use gurobi_rust::prelude::{
    CallbackTrait, Expr, GRBCallback, GRBCallbackContext, GRBModel, GRBModelSense, GRBVar,
    GRBVarType, GRBenv,
};

struct MyCallback;
impl CallbackTrait for MyCallback {
    fn callback(&self, cb_ctx: GRBCallbackContext) {
        println!("Callback called from Rust! where = {}", cb_ctx.where_);
    }
}

#[test]
fn test_build_model() {
    // first create env
    let env =
        GRBenv::new(false, None).expect("err nerrr sth happened when creating the environment");
    let mut model = GRBModel::new(env);

    let x = model.add_var(
        GRBVar::builder()
            .ub(10.0)
            .vtype(GRBVarType::INTEGER)
            .name("x".to_owned()),
    );
    let y = model.add_var(
        GRBVar::builder()
            .ub(5.0)
            .vtype(GRBVarType::INTEGER)
            .name("y".to_owned()),
    );

    let obj = 5.0 * x + 100.0 * y;
    model.add_constr((x + y).le(6.0));
    model.set_objective(obj, GRBModelSense::MAXIMIZE);
    let mut callback = GRBCallback::new(MyCallback);
    model.set_callback(&mut callback);
    model.optimize();
}
