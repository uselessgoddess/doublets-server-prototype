use crate::model::*;
use async_graphql::{InputObject};

#[derive(InputObject, Debug)]
#[graphql(name = "links_stddev_pop_order_by")]
pub struct LinksStddevPopOrderBy {
    #[graphql(name = "from_id")]
    pub from_id: Option<OrderBy>,
    #[graphql(name = "id")]
    pub id: Option<OrderBy>,
    #[graphql(name = "to_id")]
    pub to_id: Option<OrderBy>,
    #[graphql(name = "type_id")]
    pub type_id: Option<OrderBy>,
}
