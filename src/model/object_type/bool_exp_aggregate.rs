use crate::model::*;
use async_graphql::{SimpleObject};


#[derive(Debug, Default, SimpleObject)]
// todo: #[graphql(complex)]
#[graphql(name = "bool_exp_aggregate")]
pub struct BoolExpAggregate {
    #[graphql(name = "aggregate")]
    pub aggregate: Option<BoolExpAggregateFields>,
    #[graphql(name = "nodes")]
    pub nodes: Vec<BoolExp>,
}
