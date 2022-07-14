use dotenv::dotenv;
use std::env;
use rdkafka::config::ClientConfig;
use rdkafka::error::KafkaError;
use rdkafka::producer::{FutureProducer};

pub async fn broker_producer() -> Result<FutureProducer, KafkaError>  {
    dotenv().ok();
    let kafka_host : String = env::var("KAFKA_HOST").expect("Not found kafka host in .env file");
    let kafka_conn_url = format!("{}", kafka_host);
    ClientConfig::new()
        .set("bootstrap.servers", kafka_conn_url)
        .set("produce.offset.report", "true")
        .set("message.timeout.ms", "5000")
        .create()
}