use crate::model::*;
use async_graphql::{InputObject};

#[derive(InputObject, Debug)]
#[graphql(name = "mp_pk_columns_input")]
pub struct MpPkColumnsInput {
    #[graphql(name = "id")]
    pub id: Bigint,
}
