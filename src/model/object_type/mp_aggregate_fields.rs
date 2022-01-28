use crate::model::*;
use crate::model::{
    MpAvgFields, MpMaxFields, MpMinFields, MpStddevFields, MpStddevPopFields, MpSumFields,
    MpVarPopFields, MpVarSampFields, MpVarianceFields,
};
use async_graphql::{ComplexObject, InputObject, Object, SimpleObject};

use std::string::String;
#[derive(Debug, SimpleObject)]
// todo: #[graphql(complex)]
#[graphql(name = "mp_aggregate_fields")]
pub struct MpAggregateFields {
    #[graphql(name = "avg")]
    pub avg: Option<MpAvgFields>,
    #[graphql(name = "count")]
    pub count: Option<i32>,
    #[graphql(name = "max")]
    pub max: Option<MpMaxFields>,
    #[graphql(name = "min")]
    pub min: Option<MpMinFields>,
    #[graphql(name = "stddev")]
    pub stddev: Option<MpStddevFields>,
    #[graphql(name = "stddev_pop")]
    pub stddev_pop: Option<MpStddevPopFields>,
    #[graphql(name = "stddev_samp")]
    pub stddev_samp: Option<MpStddevFields>,
    #[graphql(name = "sum")]
    pub sum: Option<MpSumFields>,
    #[graphql(name = "var_pop")]
    pub var_pop: Option<MpVarPopFields>,
    #[graphql(name = "var_samp")]
    pub var_samp: Option<MpVarSampFields>,
    #[graphql(name = "variance")]
    pub variance: Option<MpVarianceFields>,
}
