use async_graphql::Enum;

#[derive(Enum, Copy, Clone, Eq, PartialEq, Debug)]
#[graphql(name = "mp_constraint")]
pub enum MpConstraint {
    #[graphql(name = "mp_pkey")]
    MpPkey,
}
