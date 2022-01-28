use crate::model::*;
use async_graphql::{ComplexObject, InputObject, Object, SimpleObject};
use std::string::String;
#[derive(InputObject, Debug)]
#[graphql(name = "links_pk_columns_input")]
pub struct LinksPkColumnsInput {
    #[graphql(name = "id")]
    pub id: Bigint,
}
