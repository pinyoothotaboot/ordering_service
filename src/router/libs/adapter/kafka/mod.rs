
use actix_web::{web};
use crate::router::libs::infrastructure::appstate::AppState;
use rdkafka::producer::{FutureRecord};
use rdkafka::util::Timeout;
use std::time::Duration;
use crate::router::libs::domain::model::{Event,AllOrder};
use serde_json;
pub struct MessageBroker {
    pub broker_producer : web::Data<AppState>
}

impl MessageBroker {
    pub async fn new(broker_producer : web::Data<AppState>) -> MessageBroker {
        MessageBroker {broker_producer}
    }

    pub async fn produce(&self,events : AllOrder , event_key : String) {
        let event_payload = serde_json::to_string(&events);
        match  event_payload {
            Ok(payload) => {
                let record = FutureRecord::to(&event_key).payload(&payload).key("");
                let queue_timeout:Timeout = Timeout::After(Duration::from_secs(10));
                match self.broker_producer.producer.clone().send(record,queue_timeout).await {
                    Ok(_reply) => {
                        println!("Send message queue successfully");
                    },
                    Err(e) => {
                        println!("{:?}",e);
                    }
                };
            },
            Err(e) => {
                println!("{:?}",e);
            }
        }
    }
}