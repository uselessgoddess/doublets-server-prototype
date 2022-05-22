use crate::model::Bigint;

use async_graphql::{InputObject};


#[derive(InputObject, Debug)]
#[graphql(name = "bigint_comparison_exp")]
pub struct BigintComparisonExp {
    #[graphql(name = "_eq")]
    pub _eq: Option<Bigint>,
    #[graphql(name = "_gt")]
    pub _gt: Option<Bigint>,
    #[graphql(name = "_gte")]
    pub _gte: Option<Bigint>,
    #[graphql(name = "_in")]
    pub _in: Option<Vec<Bigint>>,
    #[graphql(name = "_is_null")]
    pub _is_null: Option<bool>,
    #[graphql(name = "_lt")]
    pub _lt: Option<Bigint>,
    #[graphql(name = "_lte")]
    pub _lte: Option<Bigint>,
    #[graphql(name = "_neq")]
    pub _neq: Option<Bigint>,
    #[graphql(name = "_nin")]
    pub _nin: Option<Vec<Bigint>>,
}
