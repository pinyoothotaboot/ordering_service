use actix_web::{get,post,put,delete,patch, web, HttpResponse, Responder};
use crate::router::libs::infrastructure::appstate::AppState;
use crate::router::libs::domain::usecase::{
    get_order,create_new_order,update_changed_order,confirm_changed_order,
    cancel_changed_order
};
use crate::router::libs::constant::{HTTP_BAD_REQUEST,HTTP_CREATED,HTTP_NOT_FOUND,HTTP_OK};
use crate::router::libs::domain::model::{OrderService,Info};

#[get("/api/v1/order/")]
async fn order() -> impl Responder {
    let result = get_order().await;
    match result {
        Ok(reply) => HttpResponse::Ok().json(reply),
        Err(e) => {
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[post("/api/v1/order/")]
async fn create_order(state: web::Data<AppState>,order_service : web::Json<OrderService>) -> impl Responder {
    let result = create_new_order(state,order_service).await;
    match result {
        Ok(reply) => HttpResponse::Created().json(reply),
        Err(e) => {
            match e["code"].as_u64() {
                Some(_code) => {
                    HttpResponse::NotFound().json(e)
                },
                None => {
                    HttpResponse::InternalServerError().finish()
                }
            }
        }
    }
}

#[put("/api/v1/order/{order_id}/")]
async fn update_order(state: web::Data<AppState>,order_service : web::Json<OrderService>,info : web::Path<Info>) -> impl Responder {
    let result = update_changed_order(state, order_service, info).await;
    match result {
        Ok(reply) => HttpResponse::Ok().json(reply),
        Err(e) => {
            match e["code"].as_u64() {
                Some(code) => {
                    if code == HTTP_BAD_REQUEST {
                        HttpResponse::BadRequest().json(e)
                    } else {
                        HttpResponse::NotFound().json(e)
                    }
                },
                None => {
                    HttpResponse::InternalServerError().finish()
                }
            }
        }
    }
}

#[patch("/api/v1/order/{order_id}/")]
async fn confirm_order(state: web::Data<AppState>,info : web::Path<Info>) -> impl Responder {
    let result = confirm_changed_order(state, info).await;
    match result {
        Ok(reply) => HttpResponse::Ok().json(reply),
        Err(e) => {
            match e["code"].as_u64() {
                Some(code) => {
                    if code == HTTP_BAD_REQUEST {
                        HttpResponse::BadRequest().json(e)
                    } else {
                        HttpResponse::NotFound().json(e)
                    }
                },
                None => {
                    HttpResponse::InternalServerError().finish()
                }
            }
        }
    }
}

#[delete("/api/v1/order/{order_id}/")]
async fn cancel_order(state: web::Data<AppState>,info : web::Path<Info>) -> impl Responder {
    let result = cancel_changed_order(state, info).await;
    match result {
        Ok(reply) => HttpResponse::Ok().json(reply),
        Err(e) => {
            match e["code"].as_u64() {
                Some(code) => {
                    if code == HTTP_BAD_REQUEST {
                        HttpResponse::BadRequest().json(e)
                    } else {
                        HttpResponse::NotFound().json(e)
                    }
                },
                None => {
                    HttpResponse::InternalServerError().finish()
                }
            }
        }
    }
}

pub fn init(cfg :&mut web::ServiceConfig) {
    cfg.service(order);
    cfg.service(create_order);
    cfg.service(update_order);
    cfg.service(confirm_order);
    cfg.service(cancel_order);
}