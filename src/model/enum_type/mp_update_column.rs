use async_graphql::Enum;

#[derive(Enum, Copy, Clone, Eq, PartialEq, Debug)]
#[graphql(name = "mp_update_column")]
pub enum MpUpdateColumn {
    #[graphql(name = "group_id")]
    GroupId,

    #[graphql(name = "id")]
    Id,

    #[graphql(name = "insert_category")]
    InsertCategory,

    #[graphql(name = "item_id")]
    ItemId,

    #[graphql(name = "path_item_depth")]
    PathItemDepth,

    #[graphql(name = "path_item_id")]
    PathItemId,

    #[graphql(name = "position_id")]
    PositionId,

    #[graphql(name = "root_id")]
    RootId,
}
