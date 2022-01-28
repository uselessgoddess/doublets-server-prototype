use crate::model::BoolExp;
use crate::model::*;
use async_graphql::{ComplexObject, InputObject, Object, SimpleObject};

use std::string::String;
#[derive(Debug, SimpleObject)]
// todo: #[graphql(complex)]
#[graphql(name = "bool_exp_mutation_response")]
pub struct BoolExpMutationResponse {
    #[graphql(name = "affected_rows")]
    pub affected_rows: i32,
    #[graphql(name = "returning")]
    pub returning: Vec<BoolExp>,
}
