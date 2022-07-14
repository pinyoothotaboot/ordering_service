use actix_web::{web};
use serde_json::Value;
use serde_json::json;
use crate::router::libs::infrastructure::appstate::AppState;
use crate::router::libs::domain::model::{OrderService,Response};
use crate::router::libs::adapter::mongo::{OrderCollection};
use crate::router::libs::domain::utilities::{
    get_lock,new_order,unlock,update_changed_order,
    confirm_changed_order,packet,cancel_changed_order
};
use crate::router::libs::constant::{
    CREATE_NEW_ORDER,UPDATE_ORDER,CONFIRM_ORDER,
    HTTP_OK,HTTP_BAD_REQUEST,HTTP_CREATED,HTTP_NOT_FOUND,CANCEL_ORDER,
    HTTP_NO_CONTENT
};
use crate::router::libs::adapter::kafka::{MessageBroker};

pub async fn create_order(state: web::Data<AppState>,order : web::Json<OrderService>) -> Value {
    let repo = OrderCollection::new(state.clone()).await;
    let id = repo.next_indentity().await;
    let order_id = repo.next_indentity().await;
    let mutex = state;
    let lock;
    loop {
        match mutex.rl.lock(order_id.clone().as_bytes(), 1000) {
            Some(l) => {
                lock = l;
                break
            },
            None => ()
        }
    }

    let all_order = new_order(order ,id,order_id.clone(),CREATE_NEW_ORDER.to_string()).await;
    let current_version  = all_order.version.clone();
    let do_order_save = repo.save(all_order,current_version).await;
    mutex.rl.unlock(&lock);
    match do_order_save {
        true => {
            let order_resp = json!({
                "order_id" : order_id 
            });

            let result = packet(
                "Create new order successfully".to_string(),
                [order_resp].to_vec(), 
                HTTP_CREATED, 
                true
            );
            return result;
        },
        false => {
            let result = packet(
                "Cannot create new order!.".to_string(),
                [].to_vec(), 
                HTTP_NOT_FOUND, 
                false
            );
            return result; 
        },
    }
}

pub async fn update_order(state: web::Data<AppState>,order : web::Json<OrderService>,order_id : String) -> Value {
    let lock;
    let mutex = state.clone();
    loop {
        match mutex.rl.lock(order_id.clone().as_bytes(), 1000) {
            Some(l) => {
                lock = l;
                break
            },
            None => ()
        }
    }

    let repo = OrderCollection::new(state).await;
    let all_order = repo.from_id(order_id.clone()).await;

    match all_order {
        Ok(update_all_order) => {
            let current_version = update_all_order.version.clone();
            let next_version = update_all_order.version.clone() + 1;
            let new_all_order = update_changed_order(order_id.clone(),update_all_order,order,next_version,UPDATE_ORDER.to_string()).await;
            let do_order_save = repo.save(new_all_order,current_version).await;
            mutex.rl.unlock(&lock);
            match do_order_save {
                true => {
                    let order_resp = json!({
                        "order_id" : order_id 
                    });
                    let result = packet(
                        "Update order successfully".to_string(),
                        [order_resp].to_vec(), 
                        HTTP_OK, 
                        true
                    );
                    return result;
                },
                false => {
                    let order_resp = json!({
                        "order_id" : order_id 
                    });
                    let result = packet(
                        "Cannot update order!.".to_string(),
                        [order_resp].to_vec(), 
                        HTTP_NOT_FOUND, 
                        false
                    );
                    return result;  
                },
            }
        },
        Err(e) => {
            mutex.rl.unlock(&lock);
            let order_resp = json!({
                "order_id" : order_id 
            });
            let result = packet(
                e.to_string(),
                [order_resp].to_vec(), 
                HTTP_NOT_FOUND, 
                false
            );
            return result;
        }
    }
    
}

pub async fn confirm_order(state: web::Data<AppState>,order_id : String) -> Value {
    let lock;
    let mutex = state.clone();
    loop {
        match mutex.rl.lock(order_id.clone().as_bytes(), 1000) {
            Some(l) => {
                lock = l;
                break
            },
            None => ()
        }
    }
    let repo = OrderCollection::new(state.clone()).await;
    let all_order = repo.from_id(order_id.clone()).await;

    match  all_order {
        Ok(new_all_order) => {
            let confirm_order = new_all_order.state.clone();
            let current_version = new_all_order.version.clone();
            let next_version = new_all_order.version.clone() + 1;
            let confirm_all_order = confirm_changed_order(order_id.clone(),new_all_order,confirm_order,next_version,CONFIRM_ORDER.to_string()).await;
            let do_order_save = repo.save(confirm_all_order.clone(),current_version).await;
            mutex.rl.unlock(&lock);
            match do_order_save {
                true => {
                    let order_resp = json!({
                        "order_id" : order_id 
                    });
                    let result = packet(
                        "Confirm order successfully".to_string(),
                        [order_resp].to_vec(), 
                        HTTP_OK, 
                        true
                    );
                    let broker = MessageBroker::new(state).await;
                    broker.produce(confirm_all_order, CONFIRM_ORDER.to_string()).await;
                    return result;
                },
                false => {
                    let order_resp = json!({
                        "order_id" : order_id 
                    });
                    let result = packet(
                        "Cannot confirm order!.".to_string(),
                        [order_resp].to_vec(), 
                        HTTP_NOT_FOUND, 
                        false
                    );
                    return result;
                },
            }
        },
        Err(e) => {
            mutex.rl.unlock(&lock);
            let order_resp = json!({
                "order_id" : order_id 
            });
            let result = packet(
                e.to_string(),
                [order_resp].to_vec(), 
                HTTP_NOT_FOUND, 
                false
            );
            return result;
        }
    }
    
    
}

pub async fn cancel_order(state: web::Data<AppState>,order_id : String) -> Value {
    let lock;
    let mutex = state.clone();
    loop {
        match mutex.rl.lock(order_id.clone().as_bytes(), 1000) {
            Some(l) => {
                lock = l;
                break
            },
            None => ()
        }
    }

    let repo = OrderCollection::new(state).await;
    let all_order = repo.from_id(order_id.clone()).await;
    match  all_order {
        Ok(new_all_order) => {
            let confirm_order = new_all_order.state.clone();
            let current_version = new_all_order.version.clone();
            let next_version = new_all_order.version.clone() + 1;
            let confirm_all_order = cancel_changed_order(order_id.clone(),new_all_order,confirm_order,next_version,CANCEL_ORDER.to_string()).await;
            let do_order_save = repo.save(confirm_all_order,current_version).await;
            mutex.rl.unlock(&lock);
            match do_order_save {
                true => {
                    let order_resp = json!({
                        "order_id" : order_id 
                    });
                    let result = packet(
                        "Cancel order successfully".to_string(),
                        [order_resp].to_vec(), 
                        HTTP_NO_CONTENT, 
                        true
                    );
                    return result;
                },
                false => {
                    let order_resp = json!({
                        "order_id" : order_id 
                    });
                    let result = packet(
                        "Cannot cancel order!.".to_string(),
                        [order_resp].to_vec(), 
                        HTTP_NOT_FOUND, 
                        false
                    );
                    return result;
                },
            }
        },
        Err(e) => {
            mutex.rl.unlock(&lock);
            let order_resp = json!({
                "order_id" : order_id 
            });
            let result = packet(
                e.to_string(),
                [order_resp].to_vec(), 
                HTTP_NOT_FOUND, 
                false
            );
            return result;
        }
    }
}