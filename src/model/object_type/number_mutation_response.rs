use crate::model::Number;

use crate::model::*;
use async_graphql::{ComplexObject, InputObject, Object, SimpleObject};
use std::string::String;
#[derive(Debug, SimpleObject)]
// todo: #[graphql(complex)]
#[graphql(name = "number_mutation_response")]
pub struct NumberMutationResponse {
    #[graphql(name = "affected_rows")]
    pub affected_rows: i32,
    #[graphql(name = "returning")]
    pub returning: Vec<Number>,
}
