use crate::model::{Bigint, Links, LinksAggregate};


use async_graphql::{SimpleObject};

use std::string::String as StdString;
#[derive(Debug, Default, SimpleObject)]
// todo: #[graphql(complex)]
#[graphql(name = "string")]
pub struct String {
    #[graphql(name = "id")]
    pub id: Bigint,
    #[graphql(name = "link")]
    pub link: Vec<Links>,
    #[graphql(name = "link_aggregate")]
    pub link_aggregate: LinksAggregate,
    #[graphql(name = "link_id")]
    pub link_id: Option<Bigint>,
    #[graphql(name = "value")]
    pub value: Option<StdString>,
}
