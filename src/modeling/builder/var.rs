use std::ffi::CString;
use std::ptr::null_mut;

use crate::ffi;
use crate::modeling::CanBeAddedToModel;
use crate::var::GRBVar;
use crate::var::GRBVarType;

pub struct GRBVarBuilder {
    lb: Option<f64>,
    ub: Option<f64>,
    obj: Option<f64>,
    vtype: Option<GRBVarType>,
    name: Option<CString>,
}

impl GRBVarBuilder {
    pub fn lb(mut self, lb: f64) -> Self {
        self.lb = Some(lb);
        self
    }
    pub fn ub(mut self, ub: f64) -> Self {
        self.ub = Some(ub);
        self
    }
    pub fn obj(mut self, obj: f64) -> Self {
        self.obj = Some(obj);
        self
    }
    pub fn vtype(mut self, vtype: GRBVarType) -> Self {
        self.vtype = Some(vtype);
        self
    }
    pub fn name(mut self, name: String) -> Self {
        self.name = Some(CString::new(name).expect("Creating variable name failed."));
        self
    }
}

impl GRBVar {
    pub fn builder() -> GRBVarBuilder {
        GRBVarBuilder {
            lb: None,
            ub: None,
            obj: None,
            vtype: None,
            name: None,
        }
    }
}

impl CanBeAddedToModel for GRBVarBuilder {
    fn add_to_model(self, model: &mut crate::model::GRBModel) {
        let name_ptr = match self.name {
            Some(cname) => cname.as_ptr(),
            None => null_mut(),
        };

        let error = unsafe {
            ffi::GRBaddvar(
                model.inner(),
                0,
                null_mut(),
                null_mut(),
                self.obj.unwrap_or(0.0),
                self.lb.unwrap_or(0.0),
                self.ub.unwrap_or(f64::INFINITY),
                self.vtype.unwrap_or(GRBVarType::CONTINUOUS).into(),
                name_ptr,
            )
        };
        model.get_error(error).unwrap();
    }
}
