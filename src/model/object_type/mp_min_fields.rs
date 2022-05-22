use crate::model::Bigint;


use async_graphql::{SimpleObject};
use std::string::String;
#[derive(Debug, Default, SimpleObject)]
// todo: #[graphql(complex)]
#[graphql(name = "mp_min_fields")]
pub struct MpMinFields {
    #[graphql(name = "group_id")]
    pub group_id: Option<Bigint>,
    #[graphql(name = "id")]
    pub id: Option<Bigint>,
    #[graphql(name = "insert_category")]
    pub insert_category: Option<String>,
    #[graphql(name = "item_id")]
    pub item_id: Option<Bigint>,
    #[graphql(name = "path_item_depth")]
    pub path_item_depth: Option<Bigint>,
    #[graphql(name = "path_item_id")]
    pub path_item_id: Option<Bigint>,
    #[graphql(name = "position_id")]
    pub position_id: Option<String>,
    #[graphql(name = "root_id")]
    pub root_id: Option<Bigint>,
}
