use crate::model::*;
use async_graphql::{InputObject};

#[derive(InputObject, Debug)]
#[graphql(name = "links_inc_input")]
pub struct LinksIncInput {
    #[graphql(name = "from_id")]
    pub from_id: Option<Bigint>,
    #[graphql(name = "id")]
    pub id: Option<Bigint>,
    #[graphql(name = "to_id")]
    pub to_id: Option<Bigint>,
    #[graphql(name = "type_id")]
    pub type_id: Option<Bigint>,
}
