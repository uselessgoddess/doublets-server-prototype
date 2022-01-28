use async_graphql::Enum;

#[derive(Enum, Copy, Clone, Eq, PartialEq, Debug)]
#[graphql(name = "number_constraint")]
pub enum NumberConstraint {
    #[graphql(name = "number_pkey")]
    NumberPkey,
}
