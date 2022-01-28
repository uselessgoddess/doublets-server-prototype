use async_graphql::Enum;

#[derive(Enum, Copy, Clone, Eq, PartialEq, Debug)]
#[graphql(name = "string_update_column")]
pub enum StringUpdateColumn {
    #[graphql(name = "id")]
    Id,

    #[graphql(name = "link_id")]
    LinkId,

    #[graphql(name = "value")]
    Value,
}
