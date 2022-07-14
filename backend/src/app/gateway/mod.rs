pub mod schema;

use actix_web::{
    web::{self, Data},
    HttpResponse,
};
use juniper::http::{graphiql::graphiql_source, GraphQLRequest};
use std::sync::Arc;

use self::schema::{Context, Schema};

pub fn http_config(cfg: &mut web::ServiceConfig) {
    let schema = Arc::new(schema::create_schema());

    // cfg.data(schema.clone()).service(
    //     web::scope("/api").service(
    //         web::scope("/v1")
    //             .route("/data", web::post().to(graphql))
    //             .route("/graphiql", web::get().to(graphiql)),
    //     ),
    // );
}

async fn graphql(data: web::Json<GraphQLRequest>, schema: web::Data<Schema>) -> HttpResponse {
    let context = Context {};
    let res = data.execute(&schema, &context).await;

    HttpResponse::Ok().json(res)
}

async fn graphiql() -> HttpResponse {
    // Get the HTML content
    let html = graphiql_source("http://localhost:8080/api/v1/data", None);

    // Render the HTML content
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}
