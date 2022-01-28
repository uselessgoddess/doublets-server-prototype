use crate::model::*;
use async_graphql::{ComplexObject, InputObject, Object, SimpleObject};
use std::string::String;
#[derive(Debug, Default, SimpleObject)]
// todo: #[graphql(complex)]
#[graphql(name = "string_stddev_pop_fields")]
pub struct StringStddevPopFields {
    #[graphql(name = "id")]
    pub id: Option<f64>,
    #[graphql(name = "link_id")]
    pub link_id: Option<f64>,
}
