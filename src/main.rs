mod models;
mod handler;
mod errors;
mod config;
mod db;

use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use tokio_postgres::NoTls;
use crate::config::Config;
use models::customer::Customer;
use handler::add_customer::add_customer;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load .env file
    dotenv().ok();

    // Load Config Struct and init DB
    let config = Config::from_env().unwrap();
    let pool = config.pg.create_pool(NoTls).unwrap();

    // Spinup HttpServer
    let server = HttpServer::new(move || {
        // init actix
        // attach service endpoints
        App::new()
            .data(pool.clone())
            .service(web::resource("/v1/add").route(web::post().to(add_customer)))
    })
    .bind(config.server_addr.clone())?
    .run();

    // stdout
    println!("Server running at http://{}/", config.server_addr);

    server.await
}
