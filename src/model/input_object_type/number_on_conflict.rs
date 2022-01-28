use crate::model::*;
use async_graphql::{ComplexObject, InputObject, Object, SimpleObject};
use std::string::String;
#[derive(InputObject, Debug)]
#[graphql(name = "number_on_conflict")]
pub struct NumberOnConflict {
    #[graphql(name = "constraint")]
    pub constraint: NumberConstraint,
    #[graphql(name = "update_columns")]
    pub update_columns: Vec<NumberUpdateColumn>,
    #[graphql(name = "where")]
    pub r#where: Option<NumberBoolExp>,
}
