#[macro_use]
extern crate log;

use dotenv::dotenv;
use listenfd::ListenFd;
use std::env;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use sqlx::MySqlPool;
use anyhow::Result;

use rusticity::system::routes;

#[actix_web::main]
async fn main() -> Result<()> {
    dotenv().ok();
    env_logger::init();

    let mut listenfd = ListenFd::from_env();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in the .env file.");
    let db_pool = MySqlPool::connect(&database_url).await?;

    let mut server = HttpServer::new(move || {
        App::new()
            .data(db_pool.clone())
            .configure(routes::init)
    });

    server = match listenfd.take_tcp_listener(0)? {
        Some(listener) => server.listen(listener)?,
        None => {
            let host = env::var("HOST").expect("HOST is not set in the .env file.");
            let port = env::var("PORT").expect("PORT is not set in the .env file.");
            server.bind(format!("{}:{}", host, port))?
        }
    };

    info!("Starting Server");
    server.run().await?;

    Ok(())
}
