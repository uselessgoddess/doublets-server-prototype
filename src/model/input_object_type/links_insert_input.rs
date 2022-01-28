use crate::model::*;
use async_graphql::{ComplexObject, InputObject, Object, SimpleObject};
use std::string::String;
#[derive(InputObject, Debug)]
#[graphql(name = "links_insert_input")]
pub struct LinksInsertInput {
    #[graphql(name = "_by_group")]
    pub _by_group: Option<MpArrRelInsertInput>,
    #[graphql(name = "_by_item")]
    pub _by_item: Option<MpArrRelInsertInput>,
    #[graphql(name = "_by_path_item")]
    pub _by_path_item: Option<MpArrRelInsertInput>,
    #[graphql(name = "_by_root")]
    pub _by_root: Option<MpArrRelInsertInput>,
    #[graphql(name = "bool_exp")]
    pub bool_exp: Option<BoolExpObjRelInsertInput>,
    #[graphql(name = "from")]
    pub from: Box<Option<LinksObjRelInsertInput>>,
    #[graphql(name = "from_id")]
    pub from_id: Option<Bigint>,
    #[graphql(name = "id")]
    pub id: Option<Bigint>,
    #[graphql(name = "in")]
    pub r#in: Option<LinksArrRelInsertInput>,
    #[graphql(name = "number")]
    pub number: Box<Option<NumberObjRelInsertInput>>,
    #[graphql(name = "out")]
    pub out: Option<LinksArrRelInsertInput>,
    #[graphql(name = "string")]
    pub string: Box<Option<StringObjRelInsertInput>>,
    #[graphql(name = "to")]
    pub to: Box<Option<LinksObjRelInsertInput>>,
    #[graphql(name = "to_id")]
    pub to_id: Option<Bigint>,
    #[graphql(name = "type")]
    pub r#type: Box<Option<LinksObjRelInsertInput>>,
    #[graphql(name = "type_id")]
    pub type_id: Option<Bigint>,
}
