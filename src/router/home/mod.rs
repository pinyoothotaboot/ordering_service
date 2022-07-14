use actix_web::{get, web, HttpResponse, Responder};

use crate::router::libs::domain::usecase::{get_home};
use crate::router::libs::infrastructure::appstate::AppState;

#[get("/api/v1")]
async fn home(state: web::Data<AppState>) -> impl Responder {
    let reply = get_home();
    return HttpResponse::Ok().json(reply);
}

pub fn init(cfg :&mut web::ServiceConfig) {
    cfg.service(home);
}