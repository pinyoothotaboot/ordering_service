use mongodb::{Client,Database};
use dotenv::dotenv;
use std::env;

pub async fn connect_mongo() -> Client {
    dotenv().ok();
    let mongo_host : String = env::var("MONGO_HOST").expect("Not found mongodb host in .env file");
    let mongo_conn_url = format!("mongodb://{}", mongo_host);
    let result = Client::with_uri_str(mongo_conn_url).await;
    let client = result.expect("Failed to connect!");
    return client;
}

pub async fn connect_database(client : Client) -> Database {
    dotenv().ok();
    let mongo_database : String = env::var("MONGO_DATABASE").expect("Not found mongodb database in .env file");
    let database = format!("{}", mongo_database);
    return client.database(&database.to_string());
}

