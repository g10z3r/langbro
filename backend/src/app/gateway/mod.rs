pub mod schema;

use actix_web::{get, route, web, Error, HttpResponse, Responder};
use actix_web_lab::respond::Html;
use juniper::http::{graphiql::graphiql_source, GraphQLRequest};
use mongodb::sync::Client;
use std::sync::Arc;

use crate::db;

use self::schema::{Context, Schema};

#[route("/graphql", method = "GET", method = "POST")]
pub async fn graphql(
    schema: web::Data<Schema>,
    mdb: web::Data<Client>,
    data: web::Json<GraphQLRequest>,
) -> Result<HttpResponse, Error> {
    let ctx = Context {
        mongodb: Arc::new(mdb.database(dotenv!("MONGO_DATABASE_NAME"))),
    };

    let res = data.execute(&schema, &ctx).await;

    Ok(HttpResponse::Ok().json(res))
}

#[get("/graphiql")]
async fn graphql_playground() -> impl Responder {
    Html(graphiql_source("/graphql", None))
}

pub fn config(config: &mut web::ServiceConfig) {
    config
        .app_data(web::Data::new(schema::create_schema()))
        .app_data(web::Data::new(db::connect().unwrap()))
        .service(graphql)
        .service(graphql_playground);
}
