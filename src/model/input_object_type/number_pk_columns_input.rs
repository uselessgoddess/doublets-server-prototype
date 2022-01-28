use crate::model::*;
use async_graphql::{ComplexObject, InputObject, Object, SimpleObject};
use std::string::String;
#[derive(InputObject, Debug)]
#[graphql(name = "number_pk_columns_input")]
pub struct NumberPkColumnsInput {
    #[graphql(name = "id")]
    pub id: Bigint,
}
