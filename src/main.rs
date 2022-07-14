use actix_cors::Cors;
use actix_web::{http, middleware, App, HttpServer};
use dotenv::dotenv;
use std::env;

mod router;
use crate::router::libs::infrastructure::redis::{connect_redis};
use router::libs::infrastructure::mongo::{connect_mongo,connect_database};
use router::libs::infrastructure::kafka::{broker_producer};
use router::libs::infrastructure::appstate::{AppState};
use router::libs::infrastructure::distributed_locked::{Locking};

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env::set_var("ORDER_LOGS", "actix_web=debug,actix_server=info");
    env_logger::init();
    
    let host : String = env::var("HOST").expect("Not found host in .env file");
    let num_worker= env::var("WORKER").expect("Not found worker in .env file").parse::<usize>();
    let worker = match num_worker {
        Ok(num) => num,
        Err(e) => 4
    };
    println!("Server Running at {} ....", host);
    let client = connect_mongo().await;
    let database = connect_database(client).await;
    let producer = broker_producer().await.expect("failed to create kafka producer");
    let rl = Locking().await;

    HttpServer::new( move || {
        let cors = Cors::default()
              .allowed_methods(vec!["GET", "POST", "DELETE","PUT","PATCH"])
              .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
              .allowed_header(http::header::CONTENT_TYPE)
              .max_age(3600);
        App::new()
        .data(AppState {db:database.clone(),producer:producer.clone(),rl:rl.clone()})
        .wrap(cors)
        .configure(router::home::init)
        .configure(router::order::init)}
    )
    .workers(worker)
    .bind(host)?
    .run()
    .await
}