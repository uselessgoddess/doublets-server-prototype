use crate::model::*;
use async_graphql::{InputObject};

#[derive(InputObject, Debug)]
#[graphql(name = "mp_bool_exp")]
pub struct MpBoolExp {
    #[graphql(name = "_and")]
    pub _and: Option<Vec<MpBoolExp>>,
    #[graphql(name = "_not")]
    pub _not: Box<Option<MpBoolExp>>,
    #[graphql(name = "_or")]
    pub _or: Option<Vec<MpBoolExp>>,
    #[graphql(name = "by_group")]
    pub by_group: Option<LinksBoolExp>,
    #[graphql(name = "by_item")]
    pub by_item: Box<Option<MpBoolExp>>,
    #[graphql(name = "by_path_item")]
    pub by_path_item: Box<Option<MpBoolExp>>,
    #[graphql(name = "by_position")]
    pub by_position: Box<Option<MpBoolExp>>,
    #[graphql(name = "by_root")]
    pub by_root: Box<Option<MpBoolExp>>,
    #[graphql(name = "group_id")]
    pub group_id: Option<BigintComparisonExp>,
    #[graphql(name = "id")]
    pub id: Option<BigintComparisonExp>,
    #[graphql(name = "insert_category")]
    pub insert_category: Option<StringComparisonExp>,
    #[graphql(name = "item")]
    pub item: Option<LinksBoolExp>,
    #[graphql(name = "item_id")]
    pub item_id: Option<BigintComparisonExp>,
    #[graphql(name = "path_item")]
    pub path_item: Option<LinksBoolExp>,
    #[graphql(name = "path_item_depth")]
    pub path_item_depth: Option<BigintComparisonExp>,
    #[graphql(name = "path_item_id")]
    pub path_item_id: Option<BigintComparisonExp>,
    #[graphql(name = "position_id")]
    pub position_id: Option<StringComparisonExp>,
    #[graphql(name = "root")]
    pub root: Option<LinksBoolExp>,
    #[graphql(name = "root_id")]
    pub root_id: Option<BigintComparisonExp>,
}
