use crate::model::Bigint;

use crate::model::*;
use async_graphql::{ComplexObject, InputObject, Object, SimpleObject};
use std::string::String;
#[derive(Debug, SimpleObject)]
// todo: #[graphql(complex)]
#[graphql(name = "number_sum_fields")]
pub struct NumberSumFields {
    #[graphql(name = "id")]
    pub id: Option<Bigint>,
    #[graphql(name = "link_id")]
    pub link_id: Option<Bigint>,
    #[graphql(name = "value")]
    pub value: Option<Float8>,
}
