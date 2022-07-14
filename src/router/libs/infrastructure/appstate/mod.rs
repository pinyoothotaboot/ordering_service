use mongodb::{Client,Database};
use rdkafka::producer::{FutureProducer};
use redis::Connection;
use redlock::RedLock;
pub struct AppState { 
    pub db: Database, 
    pub producer : FutureProducer,
    pub rl : RedLock
}