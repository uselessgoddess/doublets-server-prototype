use crate::model::*;
use async_graphql::{ComplexObject, InputObject, Object, SimpleObject};
use std::string::String;
#[derive(Debug, Default, SimpleObject)]
// todo: #[graphql(complex)]
#[graphql(name = "string_var_samp_fields")]
pub struct StringVarSampFields {
    #[graphql(name = "id")]
    pub id: Option<f64>,
    #[graphql(name = "link_id")]
    pub link_id: Option<f64>,
}
