use crate::model::*;
use async_graphql::{ComplexObject, InputObject, Object, SimpleObject};
use std::string::String;
#[derive(InputObject, Debug)]
#[graphql(name = "bool_exp_obj_rel_insert_input")]
pub struct BoolExpObjRelInsertInput {
    #[graphql(name = "data")]
    pub data: BoolExpInsertInput,
    #[graphql(name = "on_conflict")]
    pub on_conflict: Option<BoolExpOnConflict>,
}
