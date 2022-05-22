use crate::model::*;
use crate::LinksCtx;
use async_graphql::{ComplexObject, Context, InputObject, Object, Result, SimpleObject};
use doublets::Doublets;
use std::default::default;
use std::string::String;

#[derive(Debug)]
pub struct MutationRoot {
    // #[graphql(name = "delete_bool_exp")]
// pub delete_bool_exp: Option<BoolExpMutationResponse>,
// #[graphql(name = "delete_bool_exp_by_pk")]
// pub delete_bool_exp_by_pk: Option<BoolExp>,
// #[graphql(name = "delete_links")]
// pub delete_links: Option<LinksMutationResponse>,
// #[graphql(name = "delete_links_by_pk")]
// pub delete_links_by_pk: Option<Links>,
// #[graphql(name = "delete_mp")]
// pub delete_mp: Option<MpMutationResponse>,
// #[graphql(name = "delete_mp_by_pk")]
// pub delete_mp_by_pk: Option<Mp>,
// #[graphql(name = "delete_number")]
// pub delete_number: Option<NumberMutationResponse>,
// #[graphql(name = "delete_number_by_pk")]
// pub delete_number_by_pk: Option<Number>,
// #[graphql(name = "delete_string")]
// pub delete_string: Option<StringMutationResponse>,
// #[graphql(name = "delete_string_by_pk")]
// pub delete_string_by_pk: Option<String>,
// #[graphql(name = "insert_bool_exp")]
// pub insert_bool_exp: Option<BoolExpMutationResponse>,
// #[graphql(name = "insert_bool_exp_one")]
// pub insert_bool_exp_one: Option<BoolExp>,
// #[graphql(name = "insert_links")]
// pub insert_links: Option<LinksMutationResponse>,
// #[graphql(name = "insert_links_one")]
// pub insert_links_one: Option<Links>,
// #[graphql(name = "insert_mp")]
// pub insert_mp: Option<MpMutationResponse>,
// #[graphql(name = "insert_mp_one")]
// pub insert_mp_one: Option<Mp>,
// #[graphql(name = "insert_number")]
// pub insert_number: Option<NumberMutationResponse>,
// #[graphql(name = "insert_number_one")]
// pub insert_number_one: Option<Number>,
// #[graphql(name = "insert_string")]
// pub insert_string: Option<StringMutationResponse>,
// #[graphql(name = "insert_string_one")]
// pub insert_string_one: Option<String>,
// #[graphql(name = "update_bool_exp")]
// pub update_bool_exp: Option<BoolExpMutationResponse>,
// #[graphql(name = "update_bool_exp_by_pk")]
// pub update_bool_exp_by_pk: Option<BoolExp>,
// #[graphql(name = "update_links")]
// pub update_links: Option<LinksMutationResponse>,
// #[graphql(name = "update_links_by_pk")]
// pub update_links_by_pk: Option<Links>,
// #[graphql(name = "update_mp")]
// pub update_mp: Option<MpMutationResponse>,
// #[graphql(name = "update_mp_by_pk")]
// pub update_mp_by_pk: Option<Mp>,
// #[graphql(name = "update_number")]
// pub update_number: Option<NumberMutationResponse>,
// #[graphql(name = "update_number_by_pk")]
// pub update_number_by_pk: Option<Number>,
// #[graphql(name = "update_string")]
// pub update_string: Option<StringMutationResponse>,
// #[graphql(name = "update_string_by_pk")]
// pub update_string_by_pk: Option<String>,
}

#[Object]
impl MutationRoot {
    #[graphql(name = "insert_links")]
    pub async fn insert_links(
        &self,
        ctx: &Context<'_>,
        #[graphql(name = "objects")] objects: Vec<LinksInsertInput>,
        #[graphql(name = "on_conflict")] on_conflict: Option<LinksOnConflict>,
    ) -> Result<LinksMutationResponse> {
        let mut links = ctx.data_unchecked::<LinksCtx>().write().await;
        let mut vec = Vec::new();
        for link in objects {
            let to_id = link.from_id.unwrap();
            let from_id = link.to_id.unwrap();
            let id = links.get_or_create(from_id, to_id)?;
            vec.push(Links {
                from_id: Some(from_id),
                id,
                to_id: Some(to_id),
                ..default()
            });
        }
        Ok(LinksMutationResponse {
            affected_rows: vec.len() as i32,
            returning: vec,
        })
    }
}
