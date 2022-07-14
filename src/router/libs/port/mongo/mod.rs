use crate::router::libs::domain::model::{Order};
use crate::router::libs::adapter::mongo::OrderCollection;
use actix_web::{web};
use crate::router::libs::infrastructure::appstate::AppState;

//#![feature(async_await)]
use async_trait::async_trait;

#[async_trait]
pub trait OrderAbstract: private::OrderAbstract {
    fn new(client : web::Data<AppState>) -> OrderCollection;
    fn next_indentity(&self) -> String;
    fn from_id(&self,id : String) -> String;
    fn save(&self,entity : Order);
}

pub(crate) mod private {
    pub trait OrderAbstract {
        
    }
}