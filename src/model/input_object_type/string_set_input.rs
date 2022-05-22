use crate::model::*;
use async_graphql::{InputObject};
use std::string::String;
#[derive(InputObject, Debug)]
#[graphql(name = "string_set_input")]
pub struct StringSetInput {
    #[graphql(name = "id")]
    pub id: Option<Bigint>,
    #[graphql(name = "link_id")]
    pub link_id: Option<Bigint>,
    #[graphql(name = "value")]
    pub value: Option<String>,
}
