use crate::model::*;
use crate::model::{
    BoolExp, BoolExpAggregate, Links, LinksAggregate, Mp, MpAggregate, Number, NumberAggregate,
    String, StringAggregate,
};
use async_graphql::{ComplexObject, InputObject, Object, SimpleObject};

#[derive(Debug, SimpleObject)]
// todo: #[graphql(complex)]
#[graphql(name = "subscription_root")]
pub struct SubscriptionRoot {
    #[graphql(name = "bool_exp")]
    pub bool_exp: Vec<BoolExp>,
    #[graphql(name = "bool_exp_aggregate")]
    pub bool_exp_aggregate: BoolExpAggregate,
    #[graphql(name = "bool_exp_by_pk")]
    pub bool_exp_by_pk: Option<BoolExp>,
    #[graphql(name = "links")]
    pub links: Vec<Links>,
    #[graphql(name = "links_aggregate")]
    pub links_aggregate: LinksAggregate,
    #[graphql(name = "links_by_pk")]
    pub links_by_pk: Option<Links>,
    #[graphql(name = "mp")]
    pub mp: Vec<Mp>,
    #[graphql(name = "mp_aggregate")]
    pub mp_aggregate: MpAggregate,
    #[graphql(name = "mp_by_pk")]
    pub mp_by_pk: Option<Mp>,
    #[graphql(name = "number")]
    pub number: Vec<Number>,
    #[graphql(name = "number_aggregate")]
    pub number_aggregate: NumberAggregate,
    #[graphql(name = "number_by_pk")]
    pub number_by_pk: Option<Number>,
    #[graphql(name = "string")]
    pub string: Vec<String>,
    #[graphql(name = "string_aggregate")]
    pub string_aggregate: StringAggregate,
    #[graphql(name = "string_by_pk")]
    pub string_by_pk: Option<String>,
}
