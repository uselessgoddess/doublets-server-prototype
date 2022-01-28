use crate::model::*;
use async_graphql::{ComplexObject, InputObject, Object, SimpleObject};
use std::string::String;
#[derive(InputObject, Debug)]
#[graphql(name = "number_aggregate_order_by")]
pub struct NumberAggregateOrderBy {
    #[graphql(name = "avg")]
    pub avg: Option<NumberAvgOrderBy>,
    #[graphql(name = "count")]
    pub count: Option<OrderBy>,
    #[graphql(name = "max")]
    pub max: Option<NumberMaxOrderBy>,
    #[graphql(name = "min")]
    pub min: Option<NumberMinOrderBy>,
    #[graphql(name = "stddev")]
    pub stddev: Option<NumberStddevOrderBy>,
    #[graphql(name = "stddev_pop")]
    pub stddev_pop: Option<NumberStddevPopOrderBy>,
    #[graphql(name = "stddev_samp")]
    pub stddev_samp: Option<NumberStddevSampOrderBy>,
    #[graphql(name = "sum")]
    pub sum: Option<NumberSumOrderBy>,
    #[graphql(name = "var_pop")]
    pub var_pop: Option<NumberVarPopOrderBy>,
    #[graphql(name = "var_samp")]
    pub var_samp: Option<NumberVarSampOrderBy>,
    #[graphql(name = "variance")]
    pub variance: Option<NumberVarianceOrderBy>,
}
