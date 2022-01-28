use crate::model::*;
use async_graphql::{ComplexObject, InputObject, Object, SimpleObject};
use std::string::String;
#[derive(InputObject, Debug)]
#[graphql(name = "JWTInput")]
pub struct JWTInput {
    #[graphql(name = "linkId")]
    pub link_id: Option<i32>,
    #[graphql(name = "role")]
    pub role: Option<String>,
}
