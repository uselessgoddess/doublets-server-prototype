use crate::model::Bigint;


use async_graphql::{SimpleObject};

#[derive(Debug, Default, SimpleObject)]
// todo: #[graphql(complex)]
#[graphql(name = "mp_sum_fields")]
pub struct MpSumFields {
    #[graphql(name = "group_id")]
    pub group_id: Option<Bigint>,
    #[graphql(name = "id")]
    pub id: Option<Bigint>,
    #[graphql(name = "item_id")]
    pub item_id: Option<Bigint>,
    #[graphql(name = "path_item_depth")]
    pub path_item_depth: Option<Bigint>,
    #[graphql(name = "path_item_id")]
    pub path_item_id: Option<Bigint>,
    #[graphql(name = "root_id")]
    pub root_id: Option<Bigint>,
}
