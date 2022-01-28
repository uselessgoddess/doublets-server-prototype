use crate::model::*;
use async_graphql::{ComplexObject, InputObject, Object, SimpleObject};
use std::string::String;
#[derive(InputObject, Debug)]
#[graphql(name = "number_order_by")]
pub struct NumberOrderBy {
    #[graphql(name = "id")]
    pub id: Option<OrderBy>,
    #[graphql(name = "link_aggregate")]
    pub link_aggregate: Option<LinksAggregateOrderBy>,
    #[graphql(name = "link_id")]
    pub link_id: Option<OrderBy>,
    #[graphql(name = "value")]
    pub value: Option<OrderBy>,
}
