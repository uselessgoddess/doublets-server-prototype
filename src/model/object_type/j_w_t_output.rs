
use async_graphql::{SimpleObject};
use std::string::String;
#[derive(Debug, Default, SimpleObject)]
// todo: #[graphql(complex)]
#[graphql(name = "JWTOutput")]
pub struct JWTOutput {
    #[graphql(name = "linkId")]
    pub link_id: Option<i32>,
    #[graphql(name = "role")]
    pub role: Option<String>,
    #[graphql(name = "token")]
    pub token: Option<String>,
}
