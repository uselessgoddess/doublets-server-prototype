use crate::model::{Bigint, Float8, Links, LinksAggregate};


use async_graphql::{SimpleObject};


#[derive(Debug, Default, SimpleObject)]
// todo: #[graphql(complex)]
#[graphql(name = "number")]
pub struct Number {
    #[graphql(name = "id")]
    pub id: Bigint,
    #[graphql(name = "link")]
    pub link: Vec<Links>,
    #[graphql(name = "link_aggregate")]
    pub link_aggregate: LinksAggregate,
    #[graphql(name = "link_id")]
    pub link_id: Option<Bigint>,
    #[graphql(name = "value")]
    pub value: Option<Float8>,
}
