use actix_web::{web, App, HttpServer};
use anyhow::Result;
use dotenv::dotenv;
use langbro::app::api::graphql::build_schema_with_context;
use langbro::app::core::context::Context;
use langbro::configure_service;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    pretty_env_logger::init();

    let ctx = Context::init().await?;
    let schema = web::Data::new(build_schema_with_context(ctx));

    let server = HttpServer::new(move || {
        App::new()
            .configure(configure_service)
            .app_data(schema.clone())
    })
    .bind("0.0.0.0:8080")?
    .run();

    server.await?;
    Ok(())
}
