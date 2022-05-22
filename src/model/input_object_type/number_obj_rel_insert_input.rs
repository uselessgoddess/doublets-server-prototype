use crate::model::*;
use async_graphql::{InputObject};

#[derive(InputObject, Debug)]
#[graphql(name = "number_obj_rel_insert_input")]
pub struct NumberObjRelInsertInput {
    #[graphql(name = "data")]
    pub data: NumberInsertInput,
    #[graphql(name = "on_conflict")]
    pub on_conflict: Option<NumberOnConflict>,
}
