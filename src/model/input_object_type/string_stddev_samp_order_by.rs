use crate::model::*;
use async_graphql::{InputObject};

#[derive(InputObject, Debug)]
#[graphql(name = "string_stddev_samp_order_by")]
pub struct StringStddevSampOrderBy {
    #[graphql(name = "id")]
    pub id: Option<OrderBy>,
    #[graphql(name = "link_id")]
    pub link_id: Option<OrderBy>,
}
