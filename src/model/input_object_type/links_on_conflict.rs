use crate::model::*;
use async_graphql::{InputObject};

#[derive(InputObject, Debug)]
#[graphql(name = "links_on_conflict")]
pub struct LinksOnConflict {
    #[graphql(name = "constraint")]
    pub constraint: LinksConstraint,
    #[graphql(name = "update_columns")]
    pub update_columns: Vec<LinksUpdateColumn>,
    #[graphql(name = "where")]
    pub r#where: Option<LinksBoolExp>,
}
