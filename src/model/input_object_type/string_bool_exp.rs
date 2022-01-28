use crate::model::*;
use async_graphql::{ComplexObject, InputObject, Object, SimpleObject};
use std::string::String;
#[derive(InputObject, Debug)]
#[graphql(name = "string_bool_exp")]
pub struct StringBoolExp {
    #[graphql(name = "_and")]
    pub _and: Option<Vec<StringBoolExp>>,
    #[graphql(name = "_not")]
    pub _not: Box<Option<StringBoolExp>>,
    #[graphql(name = "_or")]
    pub _or: Option<Vec<StringBoolExp>>,
    #[graphql(name = "id")]
    pub id: Option<BigintComparisonExp>,
    #[graphql(name = "link")]
    pub link: Option<LinksBoolExp>,
    #[graphql(name = "link_id")]
    pub link_id: Option<BigintComparisonExp>,
    #[graphql(name = "value")]
    pub value: Option<StringComparisonExp>,
}
