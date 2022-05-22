use crate::model::*;
use async_graphql::{InputObject};

#[derive(InputObject, Debug)]
#[graphql(name = "links_aggregate_order_by")]
pub struct LinksAggregateOrderBy {
    #[graphql(name = "avg")]
    pub avg: Option<LinksAvgOrderBy>,
    #[graphql(name = "count")]
    pub count: Option<OrderBy>,
    #[graphql(name = "max")]
    pub max: Option<LinksMaxOrderBy>,
    #[graphql(name = "min")]
    pub min: Option<LinksMinOrderBy>,
    #[graphql(name = "stddev")]
    pub stddev: Option<LinksStddevOrderBy>,
    #[graphql(name = "stddev_pop")]
    pub stddev_pop: Option<LinksStddevPopOrderBy>,
    #[graphql(name = "stddev_samp")]
    pub stddev_samp: Option<LinksStddevSampOrderBy>,
    #[graphql(name = "sum")]
    pub sum: Option<LinksSumOrderBy>,
    #[graphql(name = "var_pop")]
    pub var_pop: Option<LinksVarPopOrderBy>,
    #[graphql(name = "var_samp")]
    pub var_samp: Option<LinksVarSampOrderBy>,
    #[graphql(name = "variance")]
    pub variance: Option<LinksVarianceOrderBy>,
}
