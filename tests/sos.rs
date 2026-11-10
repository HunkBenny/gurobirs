use gurobirs::prelude::*;
use gurobirs_sys as ffi;

#[test]
fn test_sos() {
    let env = GRBEnv::new(false, None).unwrap();
    let mut model = GRBModel::new(&env);

    let ub = [1.0, 1.0, 2.0];
    let obj = [-2.0, -1.0, -1.0];
    let names = ["x0", "x1", "x2"];

    let mut x = Vec::new();
    for i in 0..3 {
        let v = model.add_var(
            GRBVar::builder()
                .ub(ub[i])
                .obj(obj[i])
                .name(names[i].to_string()),
        );
        x.push(v);
    }

    // first SOS1: x0=0 or x1=0
    let sosv1 = vec![x[0], x[1]];
    let soswt1 = vec![1.0, 2.0];
    model.add_sos(&sosv1, &soswt1, 2, ffi::GRB_SOS_TYPE1);

    // second SOS1: x0=0 or x2=0
    let sosv2 = vec![x[0], x[2]];
    let soswt2 = vec![1.0, 2.0];
    model.add_sos(&sosv2, &soswt2, 2, ffi::GRB_SOS_TYPE1);

    model.optimize();

    for i in 0..3 {
        println!("{} {}", x[i].get(GRBStrAttr::VARNAME), x[i].get(GRBDblAttr::X));
    }
    println!("Obj: {}", model.get(GRBDblAttr::OBJVAL));
}
