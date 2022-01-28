use async_graphql::Enum;

#[derive(Enum, Copy, Clone, Eq, PartialEq, Debug)]
#[graphql(name = "string_constraint")]
pub enum StringConstraint {
    #[graphql(name = "string_pkey")]
    StringPkey,
}
