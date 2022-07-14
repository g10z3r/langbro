mod app;
mod core;
mod db;

use actix_cors::Cors;
use actix_web::{
    get, middleware::Logger, route, web, App, Error, HttpResponse, HttpServer, Responder,
};
use actix_web_lab::respond::Html;
use juniper::http::{graphiql::graphiql_source, GraphQLRequest};

use crate::app::gateway::http_config;

use crate::app::gateway::schema::{create_schema, Context, Schema};

/// GraphQL endpoint
#[route("/graphql", method = "GET", method = "POST")]
pub async fn graphql(
    schema: web::Data<Schema>,
    data: web::Json<GraphQLRequest>,
) -> Result<HttpResponse, Error> {
    let ctx = Context {};

    let res = data.execute(&schema, &ctx).await;

    Ok(HttpResponse::Ok().json(res))
}

/// GraphiQL UI
#[get("/graphiql")]
async fn graphql_playground() -> impl Responder {
    Html(graphiql_source("/graphql", None))
}

pub fn register(config: &mut web::ServiceConfig) {
    config
        .app_data(web::Data::new(create_schema()))
        .service(graphql)
        .service(graphql_playground);
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting HTTP server: go to http://localhost:8080");

    HttpServer::new(|| App::new().configure(register))
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}
