use crate::model::Bigint;


use async_graphql::{SimpleObject};
use std::string::String;
#[derive(Debug, Default, SimpleObject)]
// todo: #[graphql(complex)]
#[graphql(name = "bool_exp_min_fields")]
pub struct BoolExpMinFields {
    #[graphql(name = "gql")]
    pub gql: Option<String>,
    #[graphql(name = "id")]
    pub id: Option<Bigint>,
    #[graphql(name = "link_id")]
    pub link_id: Option<Bigint>,
    #[graphql(name = "sql")]
    pub sql: Option<String>,
}
