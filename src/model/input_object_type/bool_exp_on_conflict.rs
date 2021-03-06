use crate::model::*;
use async_graphql::{InputObject};

#[derive(InputObject, Debug)]
#[graphql(name = "bool_exp_on_conflict")]
pub struct BoolExpOnConflict {
    #[graphql(name = "constraint")]
    pub constraint: BoolExpConstraint,
    #[graphql(name = "update_columns")]
    pub update_columns: Vec<BoolExpUpdateColumn>,
    #[graphql(name = "where")]
    pub r#where: Option<BoolExpBoolExp>,
}
