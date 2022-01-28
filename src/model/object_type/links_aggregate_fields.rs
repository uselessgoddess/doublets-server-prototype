use crate::model::*;
use crate::model::{
    LinksAvgFields, LinksMaxFields, LinksMinFields, LinksStddevFields, LinksStddevPopFields,
    LinksStddevSampFields, LinksSumFields, LinksVarPopFields, LinksVarSampFields,
    LinksVarianceFields,
};
use async_graphql::{ComplexObject, InputObject, Object, SimpleObject};

use std::string::String;
#[derive(Debug, Default, SimpleObject)]
// todo: #[graphql(complex)]
#[graphql(name = "links_aggregate_fields")]
pub struct LinksAggregateFields {
    #[graphql(name = "avg")]
    pub avg: Option<LinksAvgFields>,
    #[graphql(name = "count")]
    pub count: Option<i32>,
    #[graphql(name = "max")]
    pub max: Option<LinksMaxFields>,
    #[graphql(name = "min")]
    pub min: Option<LinksMinFields>,
    #[graphql(name = "stddev")]
    pub stddev: Option<LinksStddevFields>,
    #[graphql(name = "stddev_pop")]
    pub stddev_pop: Option<LinksStddevPopFields>,
    #[graphql(name = "stddev_samp")]
    pub stddev_samp: Option<LinksStddevSampFields>,
    #[graphql(name = "sum")]
    pub sum: Option<LinksSumFields>,
    #[graphql(name = "var_pop")]
    pub var_pop: Option<LinksVarPopFields>,
    #[graphql(name = "var_samp")]
    pub var_samp: Option<LinksVarSampFields>,
    #[graphql(name = "variance")]
    pub variance: Option<LinksVarianceFields>,
}
