
use async_graphql::{InputObject};
use std::string::String;
#[derive(InputObject, Debug)]
#[graphql(name = "String_comparison_exp")]
pub struct StringComparisonExp {
    #[graphql(name = "_eq")]
    pub _eq: Option<String>,
    #[graphql(name = "_gt")]
    pub _gt: Option<String>,
    #[graphql(name = "_gte")]
    pub _gte: Option<String>,
    #[graphql(name = "_ilike")]
    pub _ilike: Option<String>,
    #[graphql(name = "_in")]
    pub _in: Option<Vec<String>>,
    #[graphql(name = "_is_null")]
    pub _is_null: Option<bool>,
    #[graphql(name = "_like")]
    pub _like: Option<String>,
    #[graphql(name = "_lt")]
    pub _lt: Option<String>,
    #[graphql(name = "_lte")]
    pub _lte: Option<String>,
    #[graphql(name = "_neq")]
    pub _neq: Option<String>,
    #[graphql(name = "_nilike")]
    pub _nilike: Option<String>,
    #[graphql(name = "_nin")]
    pub _nin: Option<Vec<String>>,
    #[graphql(name = "_nlike")]
    pub _nlike: Option<String>,
    #[graphql(name = "_nsimilar")]
    pub _nsimilar: Option<String>,
    #[graphql(name = "_similar")]
    pub _similar: Option<String>,
}
