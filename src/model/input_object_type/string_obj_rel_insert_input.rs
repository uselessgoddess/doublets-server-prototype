use crate::model::*;
use async_graphql::{InputObject};

#[derive(InputObject, Debug)]
#[graphql(name = "string_obj_rel_insert_input")]
pub struct StringObjRelInsertInput {
    #[graphql(name = "data")]
    pub data: StringInsertInput,
    #[graphql(name = "on_conflict")]
    pub on_conflict: Option<StringOnConflict>,
}
