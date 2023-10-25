pub mod coffee_model;
pub mod failure_model;
pub mod response_model;

use self::{
    coffee_model::CoffeeOutModel, failure_model::FaiureOutModel, response_model::ResponseModel,
};

pub type CoffeeTypeModel = Result<ResponseModel<CoffeeOutModel>, ResponseModel<FaiureOutModel>>;
pub type CoffeesTypeModel =
    Result<ResponseModel<Vec<CoffeeOutModel>>, ResponseModel<FaiureOutModel>>;
