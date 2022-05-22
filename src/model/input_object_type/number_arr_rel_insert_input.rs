use crate::model::*;
use async_graphql::{InputObject};

#[derive(InputObject, Debug)]
#[graphql(name = "number_arr_rel_insert_input")]
pub struct NumberArrRelInsertInput {
    #[graphql(name = "data")]
    pub data: Vec<NumberInsertInput>,
    #[graphql(name = "on_conflict")]
    pub on_conflict: Option<NumberOnConflict>,
}
