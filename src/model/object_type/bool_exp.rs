
use crate::model::{Bigint, Links, LinksAggregate};
use async_graphql::{ComplexObject, SimpleObject};

use std::string::String;
#[derive(Debug, Default, SimpleObject)]
// todo: #[graphql(complex)]
#[graphql(name = "bool_exp")]
pub struct BoolExp {
    #[graphql(name = "gql")]
    pub gql: Option<String>,
    #[graphql(name = "id")]
    pub id: Bigint,
    #[graphql(name = "link")]
    pub link: Vec<Links>,
    #[graphql(name = "link_aggregate")]
    pub link_aggregate: LinksAggregate,
    #[graphql(name = "link_id")]
    pub link_id: Option<Bigint>,
    #[graphql(name = "sql")]
    pub sql: Option<String>,
}

#[ComplexObject]
impl BoolExp {}
