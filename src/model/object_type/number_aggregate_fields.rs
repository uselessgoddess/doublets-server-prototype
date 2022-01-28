use crate::model::input_object_type::number_var_samp_order_by::NumberVarSampOrderBy;
use crate::model::*;
use crate::model::{
    NumberAvgFields, NumberMaxFields, NumberMinFields, NumberStddevFields, NumberStddevPopFields,
    NumberStddevSampFields, NumberSumFields, NumberVarPopFields, NumberVarianceFields,
};
use async_graphql::{ComplexObject, InputObject, Object, SimpleObject};

use std::string::String;
#[derive(Debug, SimpleObject)]
// todo: #[graphql(complex)]
#[graphql(name = "number_aggregate_fields")]
pub struct NumberAggregateFields {
    #[graphql(name = "avg")]
    pub avg: Option<NumberAvgFields>,
    #[graphql(name = "count")]
    pub count: Option<i32>,
    #[graphql(name = "max")]
    pub max: Option<NumberMaxFields>,
    #[graphql(name = "min")]
    pub min: Option<NumberMinFields>,
    #[graphql(name = "stddev")]
    pub stddev: Option<NumberStddevFields>,
    #[graphql(name = "stddev_pop")]
    pub stddev_pop: Option<NumberStddevPopFields>,
    #[graphql(name = "stddev_samp")]
    pub stddev_samp: Option<NumberStddevSampFields>,
    #[graphql(name = "sum")]
    pub sum: Option<NumberSumFields>,
    #[graphql(name = "var_pop")]
    pub var_pop: Option<NumberVarPopFields>,
    #[graphql(name = "var_samp")]
    pub var_samp: Option<NumberVarSampFields>,
    #[graphql(name = "variance")]
    pub variance: Option<NumberVarianceFields>,
}
