use crate::model::{String, StringAggregateFields};

use crate::model::*;
use async_graphql::{ComplexObject, InputObject, Object, SimpleObject};
#[derive(Debug, SimpleObject)]
// todo: #[graphql(complex)]
#[graphql(name = "string_aggregate")]
pub struct StringAggregate {
    #[graphql(name = "aggregate")]
    pub aggregate: Option<StringAggregateFields>,
    #[graphql(name = "nodes")]
    pub nodes: Vec<String>,
}
