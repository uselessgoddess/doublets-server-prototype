
use async_graphql::{SimpleObject};

#[derive(Debug, Default, SimpleObject)]
// todo: #[graphql(complex)]
#[graphql(name = "bool_exp_var_samp_fields")]
pub struct BoolExpVarSampFields {
    #[graphql(name = "id")]
    pub id: Option<f64>,
    #[graphql(name = "link_id")]
    pub link_id: Option<f64>,
}
