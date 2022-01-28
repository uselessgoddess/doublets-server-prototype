use async_graphql::Enum;

#[derive(Enum, Copy, Clone, Eq, PartialEq, Debug)]
#[graphql(name = "links_update_column")]
pub enum LinksUpdateColumn {
    #[graphql(name = "id")]
    Id,

    #[graphql(name = "from_id")]
    FromId,

    #[graphql(name = "to_id")]
    ToId,

    #[graphql(name = "type_id")]
    TypeId,
}
