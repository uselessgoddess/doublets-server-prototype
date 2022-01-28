use crate::model::*;
use async_graphql::{ComplexObject, InputObject, Object, SimpleObject};
use std::string::String;
#[derive(InputObject, Debug)]
#[graphql(name = "string_arr_rel_insert_input")]
pub struct StringArrRelInsertInput {
    #[graphql(name = "data")]
    pub data: Vec<StringInsertInput>,
    #[graphql(name = "on_conflict")]
    pub on_conflict: Option<StringOnConflict>,
}
