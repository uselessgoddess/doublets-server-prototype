use crate::model::Bigint;
use async_graphql::SimpleObject;

use std::string::String;
#[derive(Debug, SimpleObject)]
// todo: #[graphql(complex)]
#[graphql(name = "bool_exp_max_fields")]
pub struct BoolExpMaxFields {
    #[graphql(name = "gql")]
    pub gql: Option<String>,
    #[graphql(name = "id")]
    pub id: Option<Bigint>,
    #[graphql(name = "link_id")]
    pub link_id: Option<Bigint>,
    #[graphql(name = "sql")]
    pub sql: Option<String>,
}
