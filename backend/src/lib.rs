pub mod app;
mod model;

#[macro_use]
extern crate dotenv_codegen;

// #[macro_use]
// extern crate lazy_static;

#[macro_use]
extern crate serde;
extern crate serde_json;

use actix_web::{guard, web, HttpRequest, HttpResponse, Result};
use app::api::security;
use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql::Schema;
use async_graphql_actix_web::{GraphQLRequest, GraphQLSubscription};

use app::api::graphql::AppSchema;

pub fn configure_service(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/")
            .route(web::post().to(index))
            .route(
                web::get()
                    .guard(guard::Header("upgrade", "websocket"))
                    .to(index_ws),
            )
            .route(web::get().to(index_playground)),
    );
}

async fn index(
    schema: web::Data<AppSchema>,
    http_req: HttpRequest,
    req: GraphQLRequest,
) -> async_graphql_actix_web::GraphQLResponse {
    let mut query = req.into_inner();
    let getting_claims_result = security::auth::parse_auth(http_req);
    query = query.data(getting_claims_result);
    schema.execute(query).await.into()
}

async fn index_ws(
    schema: web::Data<AppSchema>,
    req: HttpRequest,
    payload: web::Payload,
) -> Result<HttpResponse> {
    GraphQLSubscription::new(Schema::clone(&*schema)).start(&req, payload)
}

async fn index_playground() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(playground_source(
            GraphQLPlaygroundConfig::new("/").subscription_endpoint("/"),
        ))
}
