
use async_graphql::{InputObject};
use std::string::String;
#[derive(InputObject, Debug)]
#[graphql(name = "JWTInput")]
pub struct JWTInput {
    #[graphql(name = "linkId")]
    pub link_id: Option<i32>,
    #[graphql(name = "role")]
    pub role: Option<String>,
}
