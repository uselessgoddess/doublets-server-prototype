use crate::model::BoolExp;

use async_graphql::{SimpleObject};


#[derive(Debug, Default, SimpleObject)]
// todo: #[graphql(complex)]
#[graphql(name = "bool_exp_mutation_response")]
pub struct BoolExpMutationResponse {
    #[graphql(name = "affected_rows")]
    pub affected_rows: i32,
    #[graphql(name = "returning")]
    pub returning: Vec<BoolExp>,
}
