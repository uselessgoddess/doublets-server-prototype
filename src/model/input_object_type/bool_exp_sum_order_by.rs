use crate::model::*;
use async_graphql::{InputObject};

#[derive(InputObject, Debug)]
#[graphql(name = "bool_exp_sum_order_by")]
pub struct BoolExpSumOrderBy {
    #[graphql(name = "id")]
    pub id: Option<OrderBy>,
    #[graphql(name = "link_id")]
    pub link_id: Option<OrderBy>,
}
