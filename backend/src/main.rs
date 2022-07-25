use actix_web::{App, HttpServer};
use dotenv::dotenv;
use langbro::app::gateway;

#[macro_use]
extern crate dotenv_codegen;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let port = dotenv!("SERVER_PORT").parse::<u16>().unwrap();
    let host = dotenv!("SERVER_HOST");

    HttpServer::new(|| App::new().configure(gateway::config))
        .bind((host, port))?
        .run()
        .await
}
