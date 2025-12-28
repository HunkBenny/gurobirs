pub(crate) trait CanBeAddedToModel {
    fn add_to_model(self, model: &mut crate::model::GRBModel);
}

pub mod constr;
pub mod var;
