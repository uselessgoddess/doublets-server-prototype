use crate::model::*;
use async_graphql::{ComplexObject, InputObject, Object, SimpleObject};

use std::string::String;
#[derive(InputObject, Debug)]
#[graphql(name = "string_insert_input")]
pub struct StringInsertInput {
    #[graphql(name = "id")]
    pub id: Option<Bigint>,
    #[graphql(name = "link")]
    pub link: Option<LinksArrRelInsertInput>,
    #[graphql(name = "link_id")]
    pub link_id: Option<Bigint>,
    #[graphql(name = "value")]
    pub value: Option<String>,
}
