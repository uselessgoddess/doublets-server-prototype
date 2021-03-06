use crate::model::*;
use async_graphql::{InputObject};

#[derive(InputObject, Debug)]
#[graphql(name = "links_arr_rel_insert_input")]
pub struct LinksArrRelInsertInput {
    #[graphql(name = "data")]
    pub data: Vec<LinksInsertInput>,
    #[graphql(name = "on_conflict")]
    pub on_conflict: Option<LinksOnConflict>,
}
