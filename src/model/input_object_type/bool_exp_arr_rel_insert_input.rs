use crate::model::*;
use async_graphql::{InputObject};

#[derive(InputObject, Debug)]
#[graphql(name = "bool_exp_arr_rel_insert_input")]
pub struct BoolExpArrRelInsertInput {
    #[graphql(name = "data")]
    pub data: Vec<BoolExpInsertInput>,
    #[graphql(name = "on_conflict")]
    pub on_conflict: Option<BoolExpOnConflict>,
}
