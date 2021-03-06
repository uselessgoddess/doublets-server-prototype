
use async_graphql::{SimpleObject};

#[derive(Debug, Default, SimpleObject)]
// todo: #[graphql(complex)]
#[graphql(name = "bool_exp_variance_fields")]
pub struct BoolExpVarianceFields {
    #[graphql(name = "id")]
    pub id: Option<f64>,
    #[graphql(name = "link_id")]
    pub link_id: Option<f64>,
}
