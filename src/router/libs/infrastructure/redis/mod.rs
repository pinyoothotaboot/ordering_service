use dotenv::dotenv;
use std::env;
use redis::{Client,Connection};

pub fn connect_redis()-> Connection {
    dotenv().ok();
    let redis_host : String = env::var("REDIS_HOST").expect("Not found redis host in .env file");
    let redis_conn_url = format!("redis://{}", redis_host);
    let client = Client::open(redis_conn_url)
        .expect("Invalid connection URL")
        .get_connection()
        .expect("failed to connect to Redis");
    return client;
}