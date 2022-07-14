
use serde::{Deserialize, Serialize};
use serde_json::Value;
#[derive(Serialize, Deserialize, Debug, PartialEq,Clone)]
pub struct Info {
    pub order_id : String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct OrderService {
    pub items : Vec<Product>,
    pub customer_id : String,
    pub user_id : String
}

#[derive(Serialize, Deserialize, Debug, PartialEq,Clone)]
pub struct Product {
    pub product_id : String,
    pub amount : u64
}

#[derive(Serialize, Deserialize, Debug, PartialEq,Clone)]
pub struct Order {
    pub id : String,
    pub items : Vec<Product>,
    pub customer_id : String,
    pub user_id : String,
    pub submitted : bool
}

#[derive(Serialize, Deserialize, Debug, PartialEq,Clone)]
pub struct Event {
    pub order : Order,
    pub event_type : String,
    pub version : u64
}

#[derive(Serialize, Deserialize, Debug, PartialEq,Clone)]
pub struct AllOrder {
    pub id : String,
    pub version : u64,
    pub events : Vec<Event>,
    pub state : Order,
    pub deleted : bool
}

#[derive(Serialize, Deserialize, Debug, PartialEq,Clone)]
pub enum Data {
    Value,Info
}

#[derive(Serialize, Deserialize, Debug, PartialEq,Clone)]
pub struct Payload {
    pub message : String,
    pub data : Vec<Info>
}

#[derive(Serialize, Deserialize, Debug, PartialEq,Clone)]
pub struct Response {
    pub code : u64,
    pub success : bool,
    pub payload : Payload
}


