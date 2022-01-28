use crate::model::*;
use async_graphql::{ComplexObject, InputObject, Object, SimpleObject};
use std::string::String;
#[derive(Debug, Default, SimpleObject)]
// todo: #[graphql(complex)]
#[graphql(name = "mp_stddev_fields")]
pub struct MpStddevFields {
    #[graphql(name = "group_id")]
    pub group_id: Option<f64>,
    #[graphql(name = "id")]
    pub id: Option<f64>,
    #[graphql(name = "item_id")]
    pub item_id: Option<f64>,
    #[graphql(name = "path_item_depth")]
    pub path_item_depth: Option<f64>,
    #[graphql(name = "path_item_id")]
    pub path_item_id: Option<f64>,
    #[graphql(name = "root_id")]
    pub root_id: Option<f64>,
}
