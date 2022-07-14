use actix_web::{web,Error};
use chrono::format::format;
use serde_json::Value;
use serde_json::json;
use std::sync::{Arc, Mutex,MutexGuard};
use crate::router::libs::domain::model::{Product,Order,Event,AllOrder,OrderService,Info,Response,Payload,Data};

pub fn packet(message : String , data : Vec<Value> , status : u64,success : bool) -> Value {
    let reply : Value = json!({
        "code": status,
        "success": success,
        "payload" : {
            "message" : message,
            "data" : data
        }
    });
    return reply;
}

pub fn get_lock(id : String) -> Arc<Mutex<String>> {
    let lock = Arc::new(Mutex::new(id));
    return lock;
}

pub fn unlock(mutex : MutexGuard<String>) {
    std::mem::drop(mutex);
}

fn index(info: web::Path<Info>) -> Result<String, Error> {
    Ok(format!("{}", info.order_id))
}

pub fn order_id(info: web::Path<Info>) -> String {
    let index_id = index(info);
    let id = match index_id {
        Ok(id) => id,
        Err(e) => "".to_string(),
    };
    return id;
}

async fn get_product_item(items : &Vec<Product>) -> Vec<Product> {
    let mut new_items : Vec<Product> = vec![];

    for item in items.iter() {
        let product_id = item.product_id.clone();
        let amount = item.amount;
        let product = Product { product_id: product_id, amount: amount };
        new_items.push(product);
    }

    return new_items;
}

async fn get_order(id : String , items : Vec<Product> , customer_id : String,submitted : bool,user_id : String) -> Order {
    let new_order : Order = Order {id : id,items : items,customer_id : customer_id,submitted : submitted,user_id : user_id};
    return new_order;
}

async fn get_event(order : Order , event_type : String, version : u64) -> Event {
    let event = Event {order : order,event_type : event_type,version : version};
    return event;
}

async fn get_all_order(id : String , version : u64 , events : Vec<Event> , state : Order  , deleted : bool) -> AllOrder {
    let all_order = AllOrder {id : id,version : version,events : events , state : state , deleted : deleted};
    return all_order;
}

pub async fn new_order(order : web::Json<OrderService> , id : String , order_id : String , event_type : String) -> AllOrder {
    let version : u64 = 1;
    let customer_id = format!("{}", order.customer_id);
    let user_id = format!("{}",order.user_id);
    let items  = &order.items;
    let new_items = get_product_item(items).await;
    let new_order = get_order(id, new_items, customer_id, false,user_id).await;
    let state = new_order.clone();
    let event = get_event(new_order,event_type,version.clone()).await;
    let events : Vec<Event> = vec![event];
    let all_order = get_all_order(order_id,1,events,state,false).await;
    return all_order;
}

pub async fn update_changed_order(order_id: String,all_order : AllOrder ,order : web::Json<OrderService>, next_version : u64,event_type : String) -> AllOrder {
    let customer_id = format!("{}", order.customer_id);
    let user_id = format!("{}",order.user_id);
    let items  = &order.items;
    let new_items = get_product_item(items).await;
    let id = all_order.state.id.clone();
    let new_order = get_order(id, new_items, customer_id, false,user_id).await;
    let state = new_order.clone();
    let event = get_event(new_order,event_type,next_version.clone()).await;
    let mut events = all_order.events.clone();
    events.push(event);
    let new_all_order = get_all_order(order_id,next_version,events,state,false).await;
    return new_all_order; 
}

pub async fn confirm_changed_order(order_id: String,all_order : AllOrder ,order : Order, next_version : u64,event_type : String) -> AllOrder {
    let customer_id = format!("{}", order.customer_id);
    let user_id = format!("{}",order.user_id);
    let items  = &order.items;
    let new_items = get_product_item(items).await;
    let id = all_order.state.id.clone();
    let new_order = get_order(id, new_items, customer_id, true,user_id).await;
    let state = new_order.clone();
    let event = get_event(new_order,event_type,next_version.clone()).await;
    let mut events = all_order.events.clone();
    events.push(event);
    let new_all_order = get_all_order(order_id,next_version,events,state,false).await;
    return new_all_order; 
}

pub async fn cancel_changed_order(order_id: String,all_order : AllOrder ,order : Order, next_version : u64,event_type : String) -> AllOrder {
    let customer_id = format!("{}", order.customer_id);
    let user_id = format!("{}",order.user_id);
    let items  = &order.items;
    let new_items = get_product_item(items).await;
    let id = all_order.state.id.clone();
    let new_order = get_order(id, new_items, customer_id, true,user_id).await;
    let state = new_order.clone();
    let event = get_event(new_order,event_type,next_version.clone()).await;
    let mut events = all_order.events.clone();
    events.push(event);
    let new_all_order = get_all_order(order_id,next_version,events,state,true).await;
    return new_all_order; 
}