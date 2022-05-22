use crate::model::*;
use async_graphql::{InputObject};

#[derive(InputObject, Debug)]
#[graphql(name = "string_on_conflict")]
pub struct StringOnConflict {
    #[graphql(name = "constraint")]
    pub constraint: StringConstraint,
    #[graphql(name = "update_columns")]
    pub update_columns: Vec<StringUpdateColumn>,
    #[graphql(name = "where")]
    pub r#where: Option<StringBoolExp>,
}
