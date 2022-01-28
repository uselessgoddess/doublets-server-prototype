use crate::model::String;

use crate::model::*;
use async_graphql::{ComplexObject, InputObject, Object, SimpleObject};
#[derive(Debug, Default, SimpleObject)]
// todo: #[graphql(complex)]
#[graphql(name = "string_mutation_response")]
pub struct StringMutationResponse {
    #[graphql(name = "affected_rows")]
    pub affected_rows: i32,
    #[graphql(name = "returning")]
    pub returning: Vec<String>,
}
