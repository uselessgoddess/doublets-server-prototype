
use crate::model::{Number, NumberAggregateFields};
use async_graphql::{SimpleObject};


#[derive(Debug, Default, SimpleObject)]
// todo: #[graphql(complex)]
#[graphql(name = "number_aggregate")]
pub struct NumberAggregate {
    #[graphql(name = "aggregate")]
    pub aggregate: Option<NumberAggregateFields>,
    #[graphql(name = "nodes")]
    pub nodes: Vec<Number>,
}
