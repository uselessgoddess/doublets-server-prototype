use crate::model::*;
use async_graphql::{InputObject};

#[derive(InputObject, Debug)]
#[graphql(name = "number_insert_input")]
pub struct NumberInsertInput {
    #[graphql(name = "id")]
    pub id: Option<Bigint>,
    #[graphql(name = "link")]
    pub link: Option<LinksArrRelInsertInput>,
    #[graphql(name = "link_id")]
    pub link_id: Option<Bigint>,
    #[graphql(name = "value")]
    pub value: Option<Float8>,
}
