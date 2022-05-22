use crate::model::*;
use async_graphql::{InputObject};

#[derive(InputObject, Debug)]
#[graphql(name = "mp_aggregate_order_by")]
pub struct MpAggregateOrderBy {
    #[graphql(name = "avg")]
    pub avg: Option<MpAvgOrderBy>,
    #[graphql(name = "count")]
    pub count: Option<OrderBy>,
    #[graphql(name = "max")]
    pub max: Option<MpMaxOrderBy>,
    #[graphql(name = "min")]
    pub min: Option<MpMinOrderBy>,
    #[graphql(name = "stddev")]
    pub stddev: Option<MpStddevOrderBy>,
    #[graphql(name = "stddev_pop")]
    pub stddev_pop: Option<MpStddevPopOrderBy>,
    #[graphql(name = "stddev_samp")]
    pub stddev_samp: Option<MpStddevSampOrderBy>,
    #[graphql(name = "sum")]
    pub sum: Option<MpSumOrderBy>,
    #[graphql(name = "var_pop")]
    pub var_pop: Option<MpVarPopOrderBy>,
    #[graphql(name = "var_samp")]
    pub var_samp: Option<MpVarSampOrderBy>,
    #[graphql(name = "variance")]
    pub variance: Option<MpVarianceOrderBy>,
}
