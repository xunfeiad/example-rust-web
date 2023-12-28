// author: xunfei
// Project: untitled4
// File: main
// Date: 2023/12/24 14:28
pub mod api;
pub mod common;
pub mod middleware;
pub mod migration;
pub mod model;
pub mod service;
pub mod test;
use actix_web::middleware::Logger;
use actix_web::{App, HttpServer};
use api::user::{get_users, update};
use serde::de::Error;
#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    middleware::log::set_env();
    HttpServer::new(move || {
        let logger = Logger::default();
        App::new()
            .wrap(logger)
            .service(get_users)
            .service(update)
            .service(api::ttmp::get_vehicle_service)
            .service(api::ttmp::get_channel)
    })
    .workers(1)
    .bind(("0.0.0.0", 30000))?
    .run()
    .await
}
