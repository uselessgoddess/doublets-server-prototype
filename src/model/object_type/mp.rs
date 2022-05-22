
use crate::model::{Bigint, Links, MpAggregate};
use async_graphql::{SimpleObject};

use std::string::String;
#[derive(Debug, Default, SimpleObject)]
// todo: #[graphql(complex)]
#[graphql(name = "mp")]
pub struct Mp {
    #[graphql(name = "by_group")]
    pub by_group: Option<Links>,
    #[graphql(name = "by_item")]
    pub by_item: Vec<Mp>,
    #[graphql(name = "by_item_aggregate")]
    pub by_item_aggregate: MpAggregate,
    #[graphql(name = "by_path_item")]
    pub by_path_item: Vec<Mp>,
    #[graphql(name = "by_path_item_aggregate")]
    pub by_path_item_aggregate: MpAggregate,
    #[graphql(name = "by_position")]
    pub by_position: Vec<Mp>,
    #[graphql(name = "by_position_aggregate")]
    pub by_position_aggregate: MpAggregate,
    #[graphql(name = "by_root")]
    pub by_root: Vec<Mp>,
    #[graphql(name = "by_root_aggregate")]
    pub by_root_aggregate: MpAggregate,
    #[graphql(name = "group_id")]
    pub group_id: Option<Bigint>,
    #[graphql(name = "id")]
    pub id: Bigint,
    #[graphql(name = "insert_category")]
    pub insert_category: Option<String>,
    #[graphql(name = "item")]
    pub item: Option<Links>,
    #[graphql(name = "item_id")]
    pub item_id: Option<Bigint>,
    #[graphql(name = "path_item")]
    pub path_item: Option<Links>,
    #[graphql(name = "path_item_depth")]
    pub path_item_depth: Option<Bigint>,
    #[graphql(name = "path_item_id")]
    pub path_item_id: Option<Bigint>,
    #[graphql(name = "position_id")]
    pub position_id: Option<String>,
    #[graphql(name = "root")]
    pub root: Option<Links>,
    #[graphql(name = "root_id")]
    pub root_id: Option<Bigint>,
}
