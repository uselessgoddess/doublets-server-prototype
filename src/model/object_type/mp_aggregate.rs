use crate::model::Mp;
use crate::model::MpAggregateFields;

use async_graphql::{SimpleObject};


#[derive(Debug, Default, SimpleObject)]
// todo: #[graphql(complex)]
#[graphql(name = "mp_aggregate")]
pub struct MpAggregate {
    #[graphql(name = "aggregate")]
    pub aggregate: Option<MpAggregateFields>,
    #[graphql(name = "nodes")]
    pub nodes: Vec<Mp>,
}
