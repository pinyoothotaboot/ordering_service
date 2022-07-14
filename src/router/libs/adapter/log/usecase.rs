
use std::time::{SystemTime,UNIX_EPOCH};

use crate::router::libs::adapter::log::setting::{Log,Logger};
use crate::router::libs::constant::{SUCCESS,WARNING,DEBUG,ERROR};

pub async fn add_log(action : String , info : String , status : &str) {
    let timestamp = SystemTime::now()
    .duration_since(UNIX_EPOCH)
    .unwrap()
    .as_millis();
    let log : Log = Log { action: action, info: info, timestamp: timestamp, status: status.to_string() };
    let logger : Logger = Logger::new(log);
    let status_match = String::from(status.to_string());

    match status_match.as_str() {
        SUCCESS => {
            logger.info();
         },
         WARNING => {
            logger.warn();
         },
         DEBUG => {
            logger.debug();
         },
         ERROR => {
            logger.error();
         },
         _ => {
             println!("No");
         }
    }
}