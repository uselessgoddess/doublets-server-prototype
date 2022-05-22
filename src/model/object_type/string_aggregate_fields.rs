
use crate::model::{
    StringAvgFields, StringMaxFields, StringMinFields, StringStddevFields, StringStddevPopFields,
    StringStddevSampFields, StringSumFields, StringVarPopFields, StringVarSampFields,
    StringVarianceFields,
};
use async_graphql::{SimpleObject};


#[derive(Debug, Default, SimpleObject)]
// todo: #[graphql(complex)]
#[graphql(name = "string_aggregate_fields")]
pub struct StringAggregateFields {
    #[graphql(name = "avg")]
    pub avg: Option<StringAvgFields>,
    #[graphql(name = "count")]
    pub count: Option<i32>,
    #[graphql(name = "max")]
    pub max: Option<StringMaxFields>,
    #[graphql(name = "min")]
    pub min: Option<StringMinFields>,
    #[graphql(name = "stddev")]
    pub stddev: Option<StringStddevFields>,
    #[graphql(name = "stddev_pop")]
    pub stddev_pop: Option<StringStddevPopFields>,
    #[graphql(name = "stddev_samp")]
    pub stddev_samp: Option<StringStddevSampFields>,
    #[graphql(name = "sum")]
    pub sum: Option<StringSumFields>,
    #[graphql(name = "var_pop")]
    pub var_pop: Option<StringVarPopFields>,
    #[graphql(name = "var_samp")]
    pub var_samp: Option<StringVarSampFields>,
    #[graphql(name = "variance")]
    pub variance: Option<StringVarianceFields>,
}
