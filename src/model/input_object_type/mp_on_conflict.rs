use crate::model::*;
use async_graphql::{ComplexObject, InputObject, Object, SimpleObject};
use std::string::String;
#[derive(InputObject, Debug)]
#[graphql(name = "mp_on_conflict")]
pub struct MpOnConflict {
    #[graphql(name = "constraint")]
    pub constraint: MpConstraint,
    #[graphql(name = "update_columns")]
    pub update_columns: Vec<MpUpdateColumn>,
    #[graphql(name = "where")]
    pub r#where: Option<MpBoolExp>,
}
