use crate::model::*;
use async_graphql::{InputObject};

#[derive(InputObject, Debug)]
#[graphql(name = "bool_exp_order_by")]
pub struct BoolExpOrderBy {
    #[graphql(name = "gql")]
    pub gql: Option<OrderBy>,
    #[graphql(name = "id")]
    pub id: Option<OrderBy>,
    #[graphql(name = "link_aggregate")]
    pub link_aggregate: Option<LinksAggregateOrderBy>,
    #[graphql(name = "link_id")]
    pub link_id: Option<OrderBy>,
    #[graphql(name = "sql")]
    pub sql: Option<OrderBy>,
}
