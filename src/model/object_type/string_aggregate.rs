use crate::model::{String, StringAggregateFields};


use async_graphql::{SimpleObject};
#[derive(Debug, Default, SimpleObject)]
// todo: #[graphql(complex)]
#[graphql(name = "string_aggregate")]
pub struct StringAggregate {
    #[graphql(name = "aggregate")]
    pub aggregate: Option<StringAggregateFields>,
    #[graphql(name = "nodes")]
    pub nodes: Vec<String>,
}
