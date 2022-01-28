use crate::model::Bigint;

use crate::model::*;
use async_graphql::{ComplexObject, InputObject, Object, SimpleObject};
use std::string::String;
#[derive(Debug, Default, SimpleObject)]
// todo: #[graphql(complex)]
#[graphql(name = "links_min_fields")]
pub struct LinksMinFields {
    #[graphql(name = "from_id")]
    pub from_id: Option<Bigint>,
    #[graphql(name = "id")]
    pub id: Option<Bigint>,
    #[graphql(name = "to_id")]
    pub to_id: Option<Bigint>,
    #[graphql(name = "type_id")]
    pub type_id: Option<Bigint>,
}
