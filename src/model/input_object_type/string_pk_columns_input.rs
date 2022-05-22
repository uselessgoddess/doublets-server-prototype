use crate::model::*;
use async_graphql::{InputObject};

#[derive(InputObject, Debug)]
#[graphql(name = "string_pk_columns_input")]
pub struct StringPkColumnsInput {
    #[graphql(name = "id")]
    pub id: Bigint,
}
