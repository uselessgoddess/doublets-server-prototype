use crate::model::Links;
use crate::model::*;
use async_graphql::{ComplexObject, InputObject, Object, SimpleObject};

use std::string::String;
#[derive(Debug, Default, SimpleObject)]
// todo: #[graphql(complex)]
#[graphql(name = "links_aggregate")]
pub struct LinksAggregate {
    #[graphql(name = "aggregate")]
    pub aggregate: Option<LinksAggregateFields>,
    #[graphql(name = "nodes")]
    pub nodes: Vec<Links>,
}
