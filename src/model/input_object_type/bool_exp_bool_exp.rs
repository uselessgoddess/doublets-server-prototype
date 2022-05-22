use crate::model::*;
use async_graphql::{InputObject};

#[derive(InputObject, Debug)]
#[graphql(name = "bool_exp_bool_exp")]
pub struct BoolExpBoolExp {
    #[graphql(name = "_and")]
    pub _and: Option<Vec<BoolExpBoolExp>>,
    #[graphql(name = "_not")]
    pub _not: Box<Option<BoolExpBoolExp>>,
    #[graphql(name = "_or")]
    pub _or: Option<Vec<BoolExpBoolExp>>,
    #[graphql(name = "gql")]
    pub gql: Option<StringComparisonExp>,
    #[graphql(name = "id")]
    pub id: Option<BigintComparisonExp>,
    #[graphql(name = "link")]
    pub link: Option<LinksBoolExp>,
    #[graphql(name = "link_id")]
    pub link_id: Option<BigintComparisonExp>,
    #[graphql(name = "sql")]
    pub sql: Option<StringComparisonExp>,
}
