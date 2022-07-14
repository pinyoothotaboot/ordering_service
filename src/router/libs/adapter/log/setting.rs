
use dotenv::dotenv;
use std::env;
extern crate chrono;
use chrono::prelude::DateTime;
use chrono::Utc;
use std::time::{ UNIX_EPOCH, Duration};
use serde::{Deserialize, Serialize};

use crate::router::libs::infrastructure::redis::{connect_redis};
use crate::router::libs::constant::{LOG_KEY};
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Log {
    pub action : String , 
    pub info : String,
    pub timestamp : u128,
    pub status : String
}

#[derive()]
pub struct Logger {
    pub log : Log
}

impl Logger {
    pub fn new(log : Log) -> Logger {
        Logger {log}
    }

    fn add_log(&self,item : String) {
        if item.is_empty() {
            return;
        }
        
        let mut con = connect_redis();
        let _: () = redis::Cmd::lpush(LOG_KEY.to_string(), item)
        .query(&mut con)
        .expect("failed to execute LPUSH for 'item'");
    }

    fn log_to_string(&self) -> String {
        let new_log = serde_json::to_string(&self.log);
        match new_log {
            Ok(data) => return data,
            Err(e) => return "".to_string(),
        }
    }

    fn timestamp_to_date(&self) -> String {
        let d = UNIX_EPOCH + Duration::from_secs((self.log.timestamp/1000) as u64);
        let datetime = DateTime::<Utc>::from(d);
        let timestamp_str = datetime.format("%Y-%m-%d %H:%M:%S.%f").to_string();
        return timestamp_str;
    }

    fn display(&self) {
        dotenv().ok();
        let log_display : String = env::var("LOG_DISPLAY")
        .expect("Not found log display in .env file");
        
        if log_display == "dev" {
            let display_log = format!(
                "[{}] - {} -> {} : {}",
                self.timestamp_to_date(),
                self.log.info,
                self.log.action,
                self.log.status
            );
            println!("{:?}",display_log);
        }
    }

    pub fn info(&self) {
        self.add_log(self.log_to_string());
        self.display();
    }

    pub fn warn(&self) {
        self.add_log(self.log_to_string());
        self.display();
    }

    pub fn debug(&self) {
        self.add_log(self.log_to_string());
        self.display();
    }

    pub fn error(&self) {
        self.add_log(self.log_to_string());
        self.display();
    }
}