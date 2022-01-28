use crate::model::*;
use async_graphql::{ComplexObject, InputObject, Object, SimpleObject};
use std::string::String;
#[derive(InputObject, Debug)]
#[graphql(name = "mp_arr_rel_insert_input")]
pub struct MpArrRelInsertInput {
    #[graphql(name = "data")]
    pub data: Vec<MpInsertInput>,
    #[graphql(name = "on_conflict")]
    pub on_conflict: Option<MpOnConflict>,
}
