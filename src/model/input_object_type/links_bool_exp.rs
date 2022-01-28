use crate::model::*;
use async_graphql::{ComplexObject, InputObject, Object, SimpleObject};
use std::string::String;
#[derive(InputObject, Debug)]
#[graphql(name = "links_bool_exp")]
pub struct LinksBoolExp {
    #[graphql(name = "_and")]
    pub _and: Option<Vec<LinksBoolExp>>,
    #[graphql(name = "_by_group")]
    pub _by_group: Box<Option<MpBoolExp>>,
    #[graphql(name = "_by_item")]
    pub _by_item: Box<Option<MpBoolExp>>,
    #[graphql(name = "_by_path_item")]
    pub _by_path_item: Box<Option<MpBoolExp>>,
    #[graphql(name = "_by_root")]
    pub _by_root: Box<Option<MpBoolExp>>,
    #[graphql(name = "_not")]
    pub _not: Box<Option<LinksBoolExp>>,
    #[graphql(name = "_or")]
    pub _or: Box<Option<Vec<LinksBoolExp>>>,
    #[graphql(name = "bool_exp")]
    pub bool_exp: Box<Option<BoolExpBoolExp>>,
    #[graphql(name = "from")]
    pub from: Box<Option<LinksBoolExp>>,
    #[graphql(name = "from_id")]
    pub from_id: Box<Option<BigintComparisonExp>>,
    #[graphql(name = "id")]
    pub id: Box<Option<BigintComparisonExp>>,
    #[graphql(name = "in")]
    pub r#in: Box<Option<LinksBoolExp>>,
    #[graphql(name = "number")]
    pub number: Box<Option<NumberBoolExp>>,
    #[graphql(name = "out")]
    pub out: Box<Option<LinksBoolExp>>,
    #[graphql(name = "string")]
    pub string: Box<Option<StringBoolExp>>,
    #[graphql(name = "to")]
    pub to: Box<Option<LinksBoolExp>>,
    #[graphql(name = "to_id")]
    pub to_id: Box<Option<BigintComparisonExp>>,
    #[graphql(name = "type")]
    pub r#type: Box<Option<LinksBoolExp>>,
    #[graphql(name = "type_id")]
    pub type_id: Box<Option<BigintComparisonExp>>,
}
