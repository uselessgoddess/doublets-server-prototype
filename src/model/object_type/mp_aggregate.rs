use crate::model::Mp;
use crate::model::MpAggregateFields;
use crate::model::*;
use async_graphql::{ComplexObject, InputObject, Object, SimpleObject};

use std::string::String;
#[derive(Debug, Default, SimpleObject)]
// todo: #[graphql(complex)]
#[graphql(name = "mp_aggregate")]
pub struct MpAggregate {
    #[graphql(name = "aggregate")]
    pub aggregate: Option<MpAggregateFields>,
    #[graphql(name = "nodes")]
    pub nodes: Vec<Mp>,
}
