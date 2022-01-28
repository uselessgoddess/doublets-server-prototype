use crate::model::*;
use crate::model::{
    BoolExpAvgFields, BoolExpMaxFields, BoolExpMinFields, BoolExpStddevFields,
    BoolExpStddevPopFields, BoolExpStddevSampFields, BoolExpSumFields, BoolExpVarPopFields,
    BoolExpVarSampFields, BoolExpVarianceFields,
};
use async_graphql::{ComplexObject, InputObject, Object, SimpleObject};

use std::string::String;
#[derive(Debug, SimpleObject)]
// todo: #[graphql(complex)]
#[graphql(name = "bool_exp_aggregate_fields")]
pub struct BoolExpAggregateFields {
    #[graphql(name = "avg")]
    pub avg: Option<BoolExpAvgFields>,
    #[graphql(name = "count")]
    pub count: Option<i32>,
    #[graphql(name = "max")]
    pub max: Option<BoolExpMaxFields>,
    #[graphql(name = "min")]
    pub min: Option<BoolExpMinFields>,
    #[graphql(name = "stddev")]
    pub stddev: Option<BoolExpStddevFields>,
    #[graphql(name = "stddev_pop")]
    pub stddev_pop: Option<BoolExpStddevPopFields>,
    #[graphql(name = "stddev_samp")]
    pub stddev_samp: Option<BoolExpStddevSampFields>,
    #[graphql(name = "sum")]
    pub sum: Option<BoolExpSumFields>,
    #[graphql(name = "var_pop")]
    pub var_pop: Option<BoolExpVarPopFields>,
    #[graphql(name = "var_samp")]
    pub var_samp: Option<BoolExpVarSampFields>,
    #[graphql(name = "variance")]
    pub variance: Option<BoolExpVarianceFields>,
}
