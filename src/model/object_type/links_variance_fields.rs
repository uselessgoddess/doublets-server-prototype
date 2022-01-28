use crate::model::*;
use async_graphql::{ComplexObject, InputObject, Object, SimpleObject};
use std::string::String;
#[derive(Debug, SimpleObject)]
// todo: #[graphql(complex)]
#[graphql(name = "links_variance_fields")]
pub struct LinksVarianceFields {
    #[graphql(name = "from_id")]
    pub from_id: Option<f64>,
    #[graphql(name = "id")]
    pub id: Option<f64>,
    #[graphql(name = "to_id")]
    pub to_id: Option<f64>,
    #[graphql(name = "type_id")]
    pub type_id: Option<f64>,
}