use crate::model::Links;
use crate::model::*;
use async_graphql::{ComplexObject, InputObject, Object, SimpleObject};

use std::string::String;
#[derive(Debug, Default, SimpleObject)]
// todo: #[graphql(complex)]
#[graphql(name = "links_mutation_response")]
pub struct LinksMutationResponse {
    #[graphql(name = "affected_rows")]
    pub affected_rows: i32,
    #[graphql(name = "returning")]
    pub returning: Vec<Links>,
}
