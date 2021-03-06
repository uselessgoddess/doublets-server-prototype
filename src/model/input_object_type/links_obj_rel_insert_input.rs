use crate::model::*;
use async_graphql::{InputObject};

#[derive(InputObject, Debug)]
#[graphql(name = "links_obj_rel_insert_input")]
pub struct LinksObjRelInsertInput {
    #[graphql(name = "data")]
    pub data: LinksInsertInput,
    #[graphql(name = "on_conflict")]
    pub on_conflict: Option<LinksOnConflict>,
}
