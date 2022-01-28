use async_graphql::Enum;

#[derive(Enum, Copy, Clone, Eq, PartialEq, Debug)]
#[graphql(name = "bool_exp_constraint")]
pub enum BoolExpConstraint {
    #[graphql(name = "bool_exp_pkey")]
    BoolExpPkey,
}
