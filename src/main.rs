#![feature(never_type)]
#![feature(default_free_fn)]

mod model;

use crate::model::Bigint;
use actix_web::web::Data;
use actix_web::{guard, web, App, HttpResponse, HttpServer, Result};
use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql::{EmptyMutation, EmptySubscription};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};
use async_std::sync::{Arc, RwLock};
use doublets::doublets::mem::splited;
use doublets::mem::FileMappedMem;
use std::collections::HashMap;
use std::error::Error;
use std::mem::MaybeUninit;

pub type LinksCtx = Arc<RwLock<splited::Links<Bigint, FileMappedMem, FileMappedMem>>>;
type Schema = async_graphql::Schema<model::QueryRoot, model::MutationRoot, EmptySubscription>;

async fn index(schema: web::Data<Schema>, req: GraphQLRequest) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

async fn index_playground() -> Result<HttpResponse> {
    let source = playground_source(GraphQLPlaygroundConfig::new("/").subscription_endpoint("/"));
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(source))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let schema = Schema::build(
        model::QueryRoot {},
        model::MutationRoot {},
        EmptySubscription,
    )
    .data::<LinksCtx>(Arc::new(RwLock::new(splited::Links::<Bigint, _, _>::new(
        FileMappedMem::new("data.links")?,
        FileMappedMem::new("index.links")?,
    )?)))
    .finish();

    //println!("{}", schema.sdl());

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(schema.clone()))
            .service(web::resource("/").guard(guard::Post()).to(index))
            .service(web::resource("/").guard(guard::Get()).to(index_playground))
    })
    .bind("127.0.0.1:1234")?
    .run()
    .await
    .map_err(|e| e.into())
}
