use crate::model::*;
use async_graphql::{ComplexObject, InputObject, Object, SimpleObject};
use std::string::String;
#[derive(InputObject, Debug)]
#[graphql(name = "bool_exp_aggregate_order_by")]
pub struct BoolExpAggregateOrderBy {
    #[graphql(name = "avg")]
    pub avg: Option<BoolExpAvgOrderBy>,
    #[graphql(name = "count")]
    pub count: Option<OrderBy>,
    #[graphql(name = "max")]
    pub max: Option<BoolExpMaxOrderBy>,
    #[graphql(name = "min")]
    pub min: Option<BoolExpMinOrderBy>,
    #[graphql(name = "stddev")]
    pub stddev: Option<BoolExpStddevOrderBy>,
    #[graphql(name = "stddev_pop")]
    pub stddev_pop: Option<BoolExpStddevPopOrderBy>,
    #[graphql(name = "stddev_samp")]
    pub stddev_samp: Option<BoolExpStddevSampOrderBy>,
    #[graphql(name = "sum")]
    pub sum: Option<BoolExpSumOrderBy>,
    #[graphql(name = "var_pop")]
    pub var_pop: Option<BoolExpVarPopOrderBy>,
    #[graphql(name = "var_samp")]
    pub var_samp: Option<BoolExpVarSampOrderBy>,
    #[graphql(name = "variance")]
    pub variance: Option<BoolExpVarianceOrderBy>,
}
