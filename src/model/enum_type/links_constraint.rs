use async_graphql::Enum;

#[derive(Enum, Copy, Clone, Eq, PartialEq, Debug)]
#[graphql(name = "links_constraint")]
pub enum LinksConstraint {
    #[graphql(name = "links_pkey")]
    LinksPkey,
}
