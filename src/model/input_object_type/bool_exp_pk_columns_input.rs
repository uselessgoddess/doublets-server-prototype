use crate::model::*;
use async_graphql::{InputObject};

#[derive(InputObject, Debug)]
#[graphql(name = "bool_exp_pk_columns_input")]
pub struct BoolExpPkColumnsInput {
    #[graphql(name = "id")]
    pub id: Bigint,
}
