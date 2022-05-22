use crate::model::*;
use async_graphql::{InputObject};

#[derive(InputObject, Debug)]
#[graphql(name = "float8_comparison_exp")]
pub struct Float8ComparisonExp {
    #[graphql(name = "_eq")]
    pub _eq: Option<Float8>,
    #[graphql(name = "_gt")]
    pub _gt: Option<Float8>,
    #[graphql(name = "_gte")]
    pub _gte: Option<Float8>,
    #[graphql(name = "_in")]
    pub _in: Option<Vec<Float8>>,
    #[graphql(name = "_is_null")]
    pub _is_null: Option<bool>,
    #[graphql(name = "_lt")]
    pub _lt: Option<Float8>,
    #[graphql(name = "_lte")]
    pub _lte: Option<Float8>,
    #[graphql(name = "_neq")]
    pub _neq: Option<Float8>,
    #[graphql(name = "_nin")]
    pub _nin: Option<Vec<Float8>>,
}
