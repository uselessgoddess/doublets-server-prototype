use crate::model::*;
use async_graphql::{ComplexObject, InputObject, Object, SimpleObject};
use std::string::String;
#[derive(InputObject, Debug)]
#[graphql(name = "bool_exp_max_order_by")]
pub struct BoolExpMaxOrderBy {
    #[graphql(name = "gql")]
    pub gql: Option<OrderBy>,
    #[graphql(name = "id")]
    pub id: Option<OrderBy>,
    #[graphql(name = "link_id")]
    pub link_id: Option<OrderBy>,
    #[graphql(name = "sql")]
    pub sql: Option<OrderBy>,
}
