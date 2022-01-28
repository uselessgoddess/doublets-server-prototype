use super::super::object_type::j_w_t_output::JWTOutput;

use crate::model::*;
use async_graphql::{ComplexObject, InputObject, Object, SimpleObject};
use std::string::String;
#[derive(Debug, SimpleObject)]
// todo: #[graphql(complex)]
#[graphql(name = "Query")]
pub struct Query {
    #[graphql(name = "jwt")]
    pub jwt: Option<JWTOutput>,
}
