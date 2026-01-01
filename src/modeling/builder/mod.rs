use crate::model::GRBModel;

pub(crate) trait CanBeAddedToModel {
    fn add_to_model(self, model: &mut GRBModel);
}

pub mod var;
