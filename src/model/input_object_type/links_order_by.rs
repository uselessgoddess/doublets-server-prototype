use crate::model::*;
use async_graphql::{InputObject};

#[derive(InputObject, Debug)]
#[graphql(name = "links_order_by")]
pub struct LinksOrderBy {
    #[graphql(name = "_by_group_aggregate")]
    pub _by_group_aggregate: Option<MpAggregateOrderBy>,
    #[graphql(name = "_by_item_aggregate")]
    pub _by_item_aggregate: Option<MpAggregateOrderBy>,
    #[graphql(name = "_by_path_item_aggregate")]
    pub _by_path_item_aggregate: Option<MpAggregateOrderBy>,
    #[graphql(name = "_by_root_aggregate")]
    pub _by_root_aggregate: Option<MpAggregateOrderBy>,
    #[graphql(name = "bool_exp")]
    pub bool_exp: Option<BoolExpOrderBy>,
    #[graphql(name = "from")]
    pub from: Box<Option<LinksOrderBy>>,
    #[graphql(name = "from_id")]
    pub from_id: Option<OrderBy>,
    #[graphql(name = "id")]
    pub id: Option<OrderBy>,
    #[graphql(name = "in_aggregate")]
    pub in_aggregate: Option<LinksAggregateOrderBy>,
    #[graphql(name = "number")]
    pub number: Option<NumberOrderBy>,
    #[graphql(name = "out_aggregate")]
    pub out_aggregate: Option<LinksAggregateOrderBy>,
    #[graphql(name = "string")]
    pub string: Option<StringOrderBy>,
    #[graphql(name = "to")]
    pub to: Box<Option<LinksOrderBy>>,
    #[graphql(name = "to_id")]
    pub to_id: Option<OrderBy>,
    #[graphql(name = "type")]
    pub r#type: Box<Option<LinksOrderBy>>,
    #[graphql(name = "type_id")]
    pub type_id: Option<OrderBy>,
}
