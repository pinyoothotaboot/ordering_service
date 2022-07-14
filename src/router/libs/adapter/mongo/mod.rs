use actix_web::{web};
use dotenv::dotenv;
use std::env;
use std::str::FromStr;
use mongodb::{bson::oid::ObjectId,bson,options::FindOneAndUpdateOptions};
use bson::{doc, Bson, Document};
use crate::router::libs::infrastructure::appstate::AppState;
use crate::router::libs::domain::model::{AllOrder,Order,Product,Event};
use crate::router::libs::port::mongo::{OrderAbstract,private};

pub struct OrderCollection {
    pub client : web::Data<AppState>
}

impl OrderCollection {
    pub async fn new(client : web::Data<AppState>) -> OrderCollection {
        OrderCollection { client }
    }

    pub async fn next_indentity(&self) -> String {
        let id = ObjectId::default().to_string();
        return id;
    }
    
    async fn doc_all_order(&self,entity : AllOrder) -> Document {
        let value = doc! {
            "$set" : {
                "id": bson::to_bson(&entity.id).unwrap() ,
                "version": bson::to_bson(&entity.version).unwrap(),
                "events": bson::to_bson(&entity.events).unwrap(),
                "state": bson::to_bson(&entity.state).unwrap(),
                "deleted" : bson::to_bson(&entity.deleted).unwrap(),
            }
        };
        return value;
    }

    async fn doc_to_all_order(&self,doc : Result<Option<AllOrder>, mongodb::error::Error>) -> Result<AllOrder,&'static str> {
        let result = match doc {
            Ok(reply) => reply,
            Err(e) => None,
        };
        let all_order = if let Some(reply) = result {
            Ok(reply)
        } else {
            Err("Not found collection")
        };
        return all_order;
    }

    pub async fn from_id(&self,id : String) -> Result<AllOrder,&'static str> {
        dotenv().ok();
        let mongo_collection : String = env::var("MONGO_COLLECTION")
        .expect("Not found mongodb collection in .env file");
        let name = format!("{}", mongo_collection);
        let collection = self.client.db.collection::<AllOrder>(&name);
        let filter = doc! {
            "id" : id,
            "deleted" : false
        };
        let doc: Result<Option<AllOrder>, mongodb::error::Error> = collection.find_one(filter, None).await;
        let all_order = self.doc_to_all_order(doc).await;
        return all_order;
    }

    pub async fn save(&self,entity : AllOrder,current_version : u64) -> bool {
        dotenv().ok();
        let mongo_collection : String = env::var("MONGO_COLLECTION")
        .expect("Not found mongodb collection in .env file");
        let name = format!("{}", mongo_collection);
        let id  = entity.clone().id;
        let update = self.doc_all_order(entity).await;
        let filter = doc! {
            "id" : bson::to_bson(&id).unwrap(), 
            "version" : bson::to_bson(&current_version).unwrap()
        };
        let options = FindOneAndUpdateOptions::builder().upsert(true).build();
        let collection = self.client.db.collection::<AllOrder>(&name);
        let result = collection.find_one_and_update(filter,update,options).await;
        
        match result {
            Ok(_reply) => {
                println!("Insert or Update order to database successfully");
                return true; 
            },
            Err(e) => {
                println!("Cannot insert or update order to database : {:?}",e);
                return false; 
            },
        }
    }
}
