use crate::model::*;
use async_graphql::{ComplexObject, InputObject, Object, SimpleObject};
use std::string::String;
#[derive(InputObject, Debug)]
#[graphql(name = "links_var_samp_order_by")]
pub struct LinksVarSampOrderBy {
    #[graphql(name = "from_id")]
    pub from_id: Option<OrderBy>,
    #[graphql(name = "id")]
    pub id: Option<OrderBy>,
    #[graphql(name = "to_id")]
    pub to_id: Option<OrderBy>,
    #[graphql(name = "type_id")]
    pub type_id: Option<OrderBy>,
}
