use super::super::object_type::j_w_t_output::JWTOutput;


use async_graphql::{SimpleObject};

#[derive(Debug, Default, SimpleObject)]
// todo: #[graphql(complex)]
#[graphql(name = "Query")]
pub struct Query {
    #[graphql(name = "jwt")]
    pub jwt: Option<JWTOutput>,
}
