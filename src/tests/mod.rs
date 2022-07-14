#[cfg(test)]
mod tests {
    use std::fmt::format;

    use actix_web::{http::header::ContentType, test, web, App};
    use serde_json::Value;
    use serde_json::json;
    use tinyjson::{JsonParseError, JsonValue,JsonGenerateError};
    //use bytes::Bytes;
    extern crate byte_string;
    use byte_string::ByteStr;
    use bson::{doc, Bson, Document};
    use dotenv::dotenv;
    use crate::router;
    use crate::router::libs::domain::model::{Product,OrderService,Response};
    use crate::router::libs::infrastructure::appstate::AppState;
    use crate::router::libs::infrastructure::mongo::{connect_mongo,connect_database};
    use crate::router::libs::infrastructure::kafka::{broker_producer};
    use crate::router::libs::infrastructure::distributed_locked::{Locking};
    use crate::router::libs::constant::{HTTP_OK,HTTP_CREATED,HTTP_BAD_REQUEST,HTTP_NOT_FOUND};

    async fn app_state() -> AppState {
        dotenv().ok();
        let client = connect_mongo().await;
        let database = connect_database(client).await;
        let producer = broker_producer().await.expect("failed to create kafka producer");
        let rl = Locking().await;
        AppState {
            db : database , 
            producer : producer,
            rl:rl
        }
    }

    fn new_order() -> Value {
        let data = r#"
        {
            "items": [
                {
                    "product_id" : "1234",
                    "amount" : 16
                }
            ],
            "customer_id" : "1234-4321",
            "user_id" : "1234-5678-999"
        }"#;

        return serde_json::from_str(data).unwrap();
    }

    fn update_order() -> Value {
        let data = r#"
        {
            "items": [
                {
                    "product_id" : "1234",
                    "amount" : 16
                },
                {
                    "product_id" : "4321",
                    "amount" : 20
                },
                {
                    "product_id" : "5555",
                    "amount" : 1
                }
            ],
            "customer_id" : "1234-4321",
            "user_id" : "1234-5678-999"
        }"#;

        return serde_json::from_str(data).unwrap();
    }

    fn data_empty() -> Value {
        let data = r#"{}"#;
        return serde_json::from_str(data).unwrap();
    }
    
    #[actix_rt::test]
    async fn test_get_home() {
        let app = App::new()
            .data(app_state().await)
            .configure(router::home::init);
        let mut app = test::init_service(app).await;
        let req = test::TestRequest::get().uri("/api/v1").to_request();
        let res = test::call_service(&mut app,req).await;
        assert_eq!(res.status().as_u16(),HTTP_OK as u16);
    }

    #[actix_rt::test]
    async fn test_create_new_order_with_data_passed() {
        let app = App::new()
            .data(app_state().await)
            .configure(router::order::init);
        let mut app = test::init_service(app).await;
        let data = new_order();
        let req = test::TestRequest::post()
        .uri("/api/v1/order/")
        .insert_header(ContentType::json())
        .set_payload(data.to_string())
        .to_request();
        let res = test::call_service(&mut app,req).await;
        assert_eq!(res.status().as_u16(),HTTP_CREATED as u16);
    }

    #[actix_rt::test]
    async fn test_create_new_order_without_data_failed() {
        let app = App::new()
            .data(app_state().await)
            .configure(router::order::init);
        let mut app = test::init_service(app).await;
        let data = data_empty();
        let req = test::TestRequest::post()
        .uri("/api/v1/order/")
        .insert_header(ContentType::json())
        .set_payload(data.to_string())
        .to_request();
        let res = test::call_service(&mut app,req).await;
        assert_eq!(res.status().as_u16(),HTTP_BAD_REQUEST as u16);
    }

    #[actix_rt::test]
    async fn test_update_changed_order_with_data_passed() {
        let app = App::new()
            .data(app_state().await)
            .configure(router::order::init);
        let mut app = test::init_service(app).await;
        let data = new_order();
        let req = test::TestRequest::post()
        .uri("/api/v1/order/")
        .insert_header(ContentType::json())
        .set_payload(data.to_string())
        .to_request();
        let res = test::call_service(&mut app,req).await;
        let reply : Response = test::read_body_json(res).await;
        let result  = json!(reply);
        match result["payload"]["data"][0]["order_id"].as_str() {
            Some(order_id) => {
                let path = format!("/api/v1/order/{}/",order_id);
                let data = update_order();
                let req = test::TestRequest::put()
                .uri(&path)
                .insert_header(ContentType::json())
                .set_payload(data.to_string())
                .to_request();
                let res = test::call_service(&mut app,req).await;
                assert_eq!(res.status().as_u16(),HTTP_OK as u16);
            },
            None => {
                println!("None data");
            }
        }
    }

    #[actix_rt::test]
    async fn test_update_changed_order_without_data_failed() { 
        let app = App::new()
            .data(app_state().await)
            .configure(router::order::init);
        let mut app = test::init_service(app).await;
        let data = new_order();
        let req = test::TestRequest::post()
        .uri("/api/v1/order/")
        .insert_header(ContentType::json())
        .set_payload(data.to_string())
        .to_request();
        let res = test::call_service(&mut app,req).await;
        let reply : Response = test::read_body_json(res).await;
        let result  = json!(reply);
        match result["payload"]["data"][0]["order_id"].as_str() {
            Some(order_id) => {
                let path = format!("/api/v1/order/{}/",order_id);
                let data = data_empty();
                let req = test::TestRequest::put()
                .uri(&path)
                .insert_header(ContentType::json())
                .set_payload(data.to_string())
                .to_request();
                let res = test::call_service(&mut app,req).await;
                assert_eq!(res.status().as_u16(),HTTP_BAD_REQUEST as u16);
            },
            None => {
                println!("None data");
            }
        }
    }

    #[actix_rt::test]
    async fn test_update_changed_order_with_invalid_order_id_failed() { 
        let app = App::new()
            .data(app_state().await)
            .configure(router::order::init);
        let mut app = test::init_service(app).await;
        let data = new_order();
        let req = test::TestRequest::post()
        .uri("/api/v1/order/")
        .insert_header(ContentType::json())
        .set_payload(data.to_string())
        .to_request();
        let res = test::call_service(&mut app,req).await;
        let reply : Response = test::read_body_json(res).await;
        let result  = json!(reply);
        match result["payload"]["data"][0]["order_id"].as_str() {
            Some(order_id) => {
                let path = format!("/api/v1/order/order_{}/",order_id);
                let data = update_order();
                let req = test::TestRequest::put()
                .uri(&path)
                .insert_header(ContentType::json())
                .set_payload(data.to_string())
                .to_request();
                let res = test::call_service(&mut app,req).await;
                assert_eq!(res.status().as_u16(),HTTP_NOT_FOUND as u16);
            },
            None => {
                println!("None data");
            }
        }
    }
    
    #[actix_rt::test]
    async fn test_confirm_changed_order_with_data_passed() {
        let app = App::new()
            .data(app_state().await)
            .configure(router::order::init);
        let mut app = test::init_service(app).await;
        let data = new_order();
        let req = test::TestRequest::post()
        .uri("/api/v1/order/")
        .insert_header(ContentType::json())
        .set_payload(data.to_string())
        .to_request();
        let res = test::call_service(&mut app,req).await;
        let reply : Response = test::read_body_json(res).await;
        let result  = json!(reply);
        match result["payload"]["data"][0]["order_id"].as_str() {
            Some(order_id) => {
                let path = format!("/api/v1/order/{}/",order_id);
                let req = test::TestRequest::patch()
                .uri(&path)
                .insert_header(ContentType::json())
                .to_request();
                let res = test::call_service(&mut app,req).await;
                assert_eq!(res.status().as_u16(),HTTP_OK as u16);
            },
            None => {
                println!("None data");
            }
        }
    }

    #[actix_rt::test]
    async fn test_confirm_changed_order_with_invalid_order_id_failed() {
        let app = App::new()
            .data(app_state().await)
            .configure(router::order::init);
        let mut app = test::init_service(app).await;
        let data = new_order();
        let req = test::TestRequest::post()
        .uri("/api/v1/order/")
        .insert_header(ContentType::json())
        .set_payload(data.to_string())
        .to_request();
        let res = test::call_service(&mut app,req).await;
        let reply : Response = test::read_body_json(res).await;
        let result  = json!(reply);
        match result["payload"]["data"][0]["order_id"].as_str() {
            Some(order_id) => {
                let path = format!("/api/v1/order/oder_{}/",order_id);
                let req = test::TestRequest::patch()
                .uri(&path)
                .insert_header(ContentType::json())
                .to_request();
                let res = test::call_service(&mut app,req).await;
                assert_eq!(res.status().as_u16(),HTTP_NOT_FOUND as u16);
            },
            None => {
                println!("None data");
            }
        }
    }

    #[actix_rt::test]
    async fn test_cancel_changed_order_with_data_passed() {
        let app = App::new()
            .data(app_state().await)
            .configure(router::order::init);
        let mut app = test::init_service(app).await;
        let data = new_order();
        let req = test::TestRequest::post()
        .uri("/api/v1/order/")
        .insert_header(ContentType::json())
        .set_payload(data.to_string())
        .to_request();
        let res = test::call_service(&mut app,req).await;
        let reply : Response = test::read_body_json(res).await;
        let result  = json!(reply);
        match result["payload"]["data"][0]["order_id"].as_str() {
            Some(order_id) => {
                let path = format!("/api/v1/order/{}/",order_id);
                let req = test::TestRequest::delete()
                .uri(&path)
                .insert_header(ContentType::json())
                .to_request();
                let res = test::call_service(&mut app,req).await;
                assert_eq!(res.status().as_u16(),HTTP_OK as u16);
            },
            None => {
                println!("None data");
            }
        }
    }


    #[actix_rt::test]
    async fn test_cancel_changed_order_with_invalid_order_id_failed() {
        let app = App::new()
            .data(app_state().await)
            .configure(router::order::init);
        let mut app = test::init_service(app).await;
        let data = new_order();
        let req = test::TestRequest::post()
        .uri("/api/v1/order/")
        .insert_header(ContentType::json())
        .set_payload(data.to_string())
        .to_request();
        let res = test::call_service(&mut app,req).await;
        let reply : Response = test::read_body_json(res).await;
        let result  = json!(reply);
        match result["payload"]["data"][0]["order_id"].as_str() {
            Some(order_id) => {
                let path = format!("/api/v1/order/order_{}/",order_id);
                let req = test::TestRequest::delete()
                .uri(&path)
                .insert_header(ContentType::json())
                .to_request();
                let res = test::call_service(&mut app,req).await;
                assert_eq!(res.status().as_u16(),HTTP_NOT_FOUND as u16);
            },
            None => {
                println!("None data");
            }
        }
    }
}