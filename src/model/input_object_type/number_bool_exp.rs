use crate::model::*;
use async_graphql::{InputObject};

#[derive(InputObject, Debug)]
#[graphql(name = "number_bool_exp")]
pub struct NumberBoolExp {
    #[graphql(name = "_and")]
    pub _and: Option<Vec<NumberBoolExp>>,
    #[graphql(name = "_not")]
    pub _not: Box<Option<NumberBoolExp>>,
    #[graphql(name = "_or")]
    pub _or: Option<Vec<NumberBoolExp>>,
    #[graphql(name = "id")]
    pub id: Option<BigintComparisonExp>,
    #[graphql(name = "link")]
    pub link: Option<LinksBoolExp>,
    #[graphql(name = "link_id")]
    pub link_id: Option<BigintComparisonExp>,
    #[graphql(name = "value")]
    pub value: Option<Float8ComparisonExp>,
}
