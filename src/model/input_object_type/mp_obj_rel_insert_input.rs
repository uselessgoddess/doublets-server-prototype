use crate::model::*;
use async_graphql::{ComplexObject, InputObject, Object, SimpleObject};
use std::string::String;
#[derive(InputObject, Debug)]
#[graphql(name = "mp_obj_rel_insert_input")]
pub struct MpObjRelInsertInput {
    #[graphql(name = "data")]
    pub data: MpInsertInput,
    #[graphql(name = "on_conflict")]
    pub on_conflict: Option<MpOnConflict>,
}
