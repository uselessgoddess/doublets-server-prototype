use crate::model::*;
use async_graphql::{InputObject};

#[derive(InputObject, Debug)]
#[graphql(name = "bool_exp_min_order_by")]
pub struct BoolExpMinOrderBy {
    #[graphql(name = "gql")]
    pub gql: Option<OrderBy>,
    #[graphql(name = "id")]
    pub id: Option<OrderBy>,
    #[graphql(name = "link_id")]
    pub link_id: Option<OrderBy>,
    #[graphql(name = "sql")]
    pub sql: Option<OrderBy>,
}
