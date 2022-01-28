use async_graphql::Enum;

#[derive(Enum, Copy, Clone, Eq, PartialEq, Debug)]
#[graphql(name = "bool_exp_update_column")]
pub enum BoolExpUpdateColumn {
    #[graphql(name = "gql")]
    Gql,

    #[graphql(name = "id")]
    Id,

    #[graphql(name = "link_id")]
    LinkId,

    #[graphql(name = "sql")]
    Sql,
}
