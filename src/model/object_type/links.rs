use crate::model::*;
use crate::model::{Bigint, LinksAggregate};
use crate::model::{BoolExp, Mp, MpAggregate, Number};
use async_graphql::{ComplexObject, InputObject, Object, SimpleObject};

use std::string::String;
#[derive(Debug, SimpleObject)]
// todo: #[graphql(complex)]
#[graphql(name = "links")]
pub struct Links {
    #[graphql(name = "_by_group")]
    pub _by_group: Vec<Mp>,
    #[graphql(name = "_by_group_aggregate")]
    pub _by_group_aggregate: MpAggregate,
    #[graphql(name = "_by_item")]
    pub _by_item: Vec<Mp>,
    #[graphql(name = "_by_item_aggregate")]
    pub _by_item_aggregate: MpAggregate,
    #[graphql(name = "_by_path_item")]
    pub _by_path_item: Vec<Mp>,
    #[graphql(name = "_by_path_item_aggregate")]
    pub _by_path_item_aggregate: MpAggregate,
    #[graphql(name = "_by_root")]
    pub _by_root: Vec<Mp>,
    #[graphql(name = "_by_root_aggregate")]
    pub _by_root_aggregate: MpAggregate,
    #[graphql(name = "bool_exp")]
    pub bool_exp: Option<BoolExp>,
    #[graphql(name = "from")]
    pub from: Box<Option<Links>>,
    #[graphql(name = "from_id")]
    pub from_id: Option<Bigint>,
    #[graphql(name = "id")]
    pub id: Bigint,
    #[graphql(name = "in")]
    pub r#in: Vec<Links>,
    #[graphql(name = "in_aggregate")]
    pub in_aggregate: LinksAggregate,
    #[graphql(name = "number")]
    pub number: Option<Number>,
    #[graphql(name = "out")]
    pub out: Vec<Links>,
    #[graphql(name = "out_aggregate")]
    pub out_aggregate: LinksAggregate,
    #[graphql(name = "string")]
    pub string: Option<String>,
    #[graphql(name = "to")]
    pub to: Box<Option<Links>>,
    #[graphql(name = "to_id")]
    pub to_id: Option<Bigint>,
    #[graphql(name = "type")]
    pub r#type: Box<Option<Links>>,
    #[graphql(name = "type_id")]
    pub type_id: Bigint,
}
