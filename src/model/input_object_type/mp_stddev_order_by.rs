use crate::model::*;
use async_graphql::{ComplexObject, InputObject, Object, SimpleObject};
use std::string::String;
#[derive(InputObject, Debug)]
#[graphql(name = "mp_stddev_order_by")]
pub struct MpStddevOrderBy {
    #[graphql(name = "group_id")]
    pub group_id: Option<OrderBy>,
    #[graphql(name = "id")]
    pub id: Option<OrderBy>,
    #[graphql(name = "item_id")]
    pub item_id: Option<OrderBy>,
    #[graphql(name = "path_item_depth")]
    pub path_item_depth: Option<OrderBy>,
    #[graphql(name = "path_item_id")]
    pub path_item_id: Option<OrderBy>,
    #[graphql(name = "root_id")]
    pub root_id: Option<OrderBy>,
}
