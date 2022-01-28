use crate::model::*;
use async_graphql::{ComplexObject, InputObject, Object, SimpleObject};
use std::string::String;
#[derive(InputObject, Debug)]
#[graphql(name = "string_aggregate_order_by")]
pub struct StringAggregateOrderBy {
    #[graphql(name = "avg")]
    pub avg: Option<StringAvgOrderBy>,
    #[graphql(name = "count")]
    pub count: Option<OrderBy>,
    #[graphql(name = "max")]
    pub max: Option<StringMaxOrderBy>,
    #[graphql(name = "min")]
    pub min: Option<StringMinOrderBy>,
    #[graphql(name = "stddev")]
    pub stddev: Option<StringStddevOrderBy>,
    #[graphql(name = "stddev_pop")]
    pub stddev_pop: Option<StringStddevPopOrderBy>,
    #[graphql(name = "stddev_samp")]
    pub stddev_samp: Option<StringStddevSampOrderBy>,
    #[graphql(name = "sum")]
    pub sum: Option<StringSumOrderBy>,
    #[graphql(name = "var_pop")]
    pub var_pop: Option<StringVarPopOrderBy>,
    #[graphql(name = "var_samp")]
    pub var_samp: Option<StringVarSampOrderBy>,
    #[graphql(name = "variance")]
    pub variance: Option<StringVarianceOrderBy>,
}
