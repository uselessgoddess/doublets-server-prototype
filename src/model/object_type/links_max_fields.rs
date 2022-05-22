use crate::model::Bigint;


use async_graphql::{SimpleObject};

#[derive(Debug, Default, SimpleObject)]
// todo: #[graphql(complex)]
#[graphql(name = "links_max_fields")]
pub struct LinksMaxFields {
    #[graphql(name = "from_id")]
    pub from_id: Option<Bigint>,
    #[graphql(name = "id")]
    pub id: Option<Bigint>,
    #[graphql(name = "to_id")]
    pub to_id: Option<Bigint>,
    #[graphql(name = "type_id")]
    pub type_id: Option<Bigint>,
}
