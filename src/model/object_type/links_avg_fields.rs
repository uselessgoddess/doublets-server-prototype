
use async_graphql::{SimpleObject};

#[derive(Debug, Default, SimpleObject)]
// todo: #[graphql(complex)]
#[graphql(name = "links_avg_fields")]
pub struct LinksAvgFields {
    #[graphql(name = "from_id")]
    pub from_id: Option<f64>,
    #[graphql(name = "id")]
    pub id: Option<f64>,
    #[graphql(name = "to_id")]
    pub to_id: Option<f64>,
    #[graphql(name = "type_id")]
    pub type_id: Option<f64>,
}
