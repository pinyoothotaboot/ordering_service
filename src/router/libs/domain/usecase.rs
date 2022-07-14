use serde_json::Value;
use serde_json::json;
use actix_web::{web};
use crate::router::libs::constant::ERROR;
use crate::router::libs::constant::HTTP_BAD_REQUEST;
use crate::router::libs::constant::{HTTP_OK};
use crate::router::libs::domain::utilities::{packet,order_id};
use crate::router::libs::adapter::log::usecase::{add_log};
use crate::router::libs::constant::{SUCCESS};
use crate::router::libs::infrastructure::appstate::AppState;
use crate::router::libs::domain::model::{OrderService,Info,Response};
use crate::router::libs::domain::repository::{create_order,update_order,confirm_order,cancel_order};

pub fn get_home() -> Value {
    let reply = packet(
        "Hello,I'am order service".to_string(),
        [].to_vec(), 
        HTTP_OK, 
        true
    );
    return reply;
}

pub async fn get_order() -> Result<Value,&'static str> {
    let reply = packet(
        "Hello,I'am order service".to_string(),
        [].to_vec(), 
        HTTP_OK, 
        true
    );
    
    let data : Result<Value,&str> = Ok(reply);
    add_log(
        "Hello,I'am order service".to_string(),
        "get_order()".to_string(),
        SUCCESS
    ).await;

    return data;
}

pub async fn create_new_order(state: web::Data<AppState>,order : web::Json<OrderService>) -> Result<Value,Value>{
    let reply = create_order(state, order).await;
    match reply["success"].as_bool() {
        Some(success) => {
            match  success {
                true => {
                    let data : Result<Value,Value> = Ok(reply);
                    return data;
                },
                false => {
                    let data : Result<Value,Value> = Err(reply);
                    add_log(
                        "Cannot create new order".to_string(),
                        "create_new_order()".to_string(),
                        ERROR
                    ).await;
                    return data;
                }
            }
        },
        None => {
            let data : Result<Value,Value> = Err(reply);
            add_log(
                "Create new order has problem".to_string(),
                "create_new_order()".to_string(),
                ERROR
            ).await;
            return data;
        }
    }
}

pub async fn update_changed_order(state: web::Data<AppState>,order : web::Json<OrderService>,info : web::Path<Info>) -> Result<Value,Value> {
    let order_id = order_id(info);
    if order_id.is_empty() {
        let order_resp = json!({
            "order_id" : order_id 
        });
        let result = packet(
            "Order id is empty!.".to_string(),
            [order_resp].to_vec(), 
            HTTP_BAD_REQUEST, 
            false
        );

        let data : Result<Value,Value> = Err(result);
        add_log(
            "Order id is empty!.".to_string(),
            "update_order()".to_string(),
            ERROR
        ).await;
        return data;
    } else {
        let reply = update_order(state, order, order_id.clone()).await;
        match reply["success"].as_bool() {
            Some(success) => {
                match  success {
                    true => {
                        let data : Result<Value,Value> = Ok(reply);
                        return data;
                    },
                    false => {
                        let data : Result<Value,Value> = Err(reply);
                        add_log(
                            format!("Cannot update changed order id {}",order_id).to_string(),
                            "update_order()".to_string(),
                            ERROR
                        ).await;
                        return data;
                    }
                }
            },
            None => {
                let data : Result<Value,Value> = Err(reply);
                add_log(
                    format!("Update changed order id {} has problem",order_id).to_string(),
                    "update_order()".to_string(),
                    ERROR
                ).await;
                return data;
            }
        }
    }
}

pub async fn confirm_changed_order(state: web::Data<AppState>,info : web::Path<Info>) -> Result<Value,Value> {
    let order_id = order_id(info);
    if order_id.is_empty() {
        let order_resp = json!({
            "order_id" : order_id 
        });
        let result = packet(
            "Order id is empty!.".to_string(),
            [order_resp].to_vec(), 
            HTTP_BAD_REQUEST, 
            false
        );

        let data : Result<Value,Value> = Err(result);
        add_log(
            "Order id is empty!.".to_string(),
            "confirm_changed_order()".to_string(),
            ERROR
        ).await;
        return data;

    } else {
        let reply = confirm_order(state, order_id.clone()).await;
        match reply["success"].as_bool() {
            Some(success) => {
                match  success {
                    true => {
                        let data : Result<Value,Value> = Ok(reply);
                        return data;
                    },
                    false => {
                        let data : Result<Value,Value> = Err(reply);
                        add_log(
                            format!("Cannot confirm changed order id {}",order_id).to_string(),
                            "confirm_changed_order()".to_string(),
                            ERROR
                        ).await;
                        return data;
                    }
                }
            },
            None => {
                let data : Result<Value,Value> = Err(reply);
                add_log(
                    format!("Confirm changed order id {} has problem",order_id).to_string(),
                    "confirm_changed_order()".to_string(),
                    ERROR
                ).await;
                return data;
            }
        }
    }
}

pub async fn cancel_changed_order(state: web::Data<AppState>,info : web::Path<Info>) -> Result<Value,Value> { 
    let order_id = order_id(info);
    if order_id.is_empty() {
        let order_resp = json!({
            "order_id" : order_id 
        });
        let result = packet(
            "Order id is empty!.".to_string(),
            [order_resp].to_vec(), 
            HTTP_BAD_REQUEST, 
            false
        );

        let data : Result<Value,Value> = Err(result);
        add_log(
            "Order id is empty!.".to_string(),
            "cancel_changed_order()".to_string(),
            ERROR
        ).await;
        return data;
    } else {
        let reply = cancel_order(state, order_id.clone()).await;
        match reply["success"].as_bool() {
            Some(success) => {
                match  success {
                    true => {
                        let data : Result<Value,Value> = Ok(reply);
                        return data;
                    },
                    false => {
                        let data : Result<Value,Value> = Err(reply);
                        add_log(
                            format!("Cannot cancel changed order id {}",order_id).to_string(),
                            "cancel_changed_order()".to_string(),
                            ERROR
                        ).await;
                        return data;
                    }
                }
            },
            None => {
                let data : Result<Value,Value> = Err(reply);
                add_log(
                    format!("Cancel changed order id {} has problem",order_id).to_string(),
                    "cancel_changed_order()".to_string(),
                    ERROR
                ).await;
                return data;
            }
        }
    }
}
