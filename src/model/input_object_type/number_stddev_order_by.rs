use crate::model::*;
use async_graphql::{InputObject};

#[derive(InputObject, Debug)]
#[graphql(name = "number_stddev_order_by")]
pub struct NumberStddevOrderBy {
    #[graphql(name = "id")]
    pub id: Option<OrderBy>,
    #[graphql(name = "link_id")]
    pub link_id: Option<OrderBy>,
    #[graphql(name = "value")]
    pub value: Option<OrderBy>,
}
