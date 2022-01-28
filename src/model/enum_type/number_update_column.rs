use async_graphql::Enum;

#[derive(Enum, Copy, Clone, Eq, PartialEq, Debug)]
#[graphql(name = "number_update_column")]
pub enum NumberUpdateColumn {
    #[graphql(name = "id")]
    Id,

    #[graphql(name = "link_id")]
    LinkId,

    #[graphql(name = "value")]
    Value,
}
