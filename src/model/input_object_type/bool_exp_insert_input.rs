use crate::model::*;
use async_graphql::{InputObject};
use std::string::String;
#[derive(InputObject, Debug)]
#[graphql(name = "bool_exp_insert_input")]
pub struct BoolExpInsertInput {
    #[graphql(name = "gql")]
    pub gql: Option<String>,
    #[graphql(name = "id")]
    pub id: Option<Bigint>,
    #[graphql(name = "link")]
    pub link: Option<LinksArrRelInsertInput>,
    #[graphql(name = "link_id")]
    pub link_id: Option<Bigint>,
    #[graphql(name = "sql")]
    pub sql: Option<String>,
}
