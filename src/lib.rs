mod shard;
mod shard_manager;

use actix_web::{
    dev::Server, http::StatusCode, web, App, HttpResponse, HttpServer, Responder, Result,
};
use serde::{Deserialize, Serialize};
use shard_manager::ShardManager;
use std::net::TcpListener;
use std::sync::{Arc, Mutex};

const SHARD_COUNT: usize = 4;

#[derive(Deserialize, Serialize)]
struct KeyValuePair {
    key: String,
    value: String,
}

// Retrieve the value associated with the key
async fn get_value(
    path: web::Path<String>,
    shard_manager: web::Data<Arc<Mutex<ShardManager>>>,
) -> impl Responder {
    let key = path.into_inner();
    let shard_manager = shard_manager.lock().unwrap(); // Lock the mutex
    let value = shard_manager.get(&key);

    match value {
        Some(value) => HttpResponse::Ok().json(value),
        None => HttpResponse::NotFound().finish(),
    }
}

// Add a new key-value pair
async fn add_key_value(
    item: web::Json<KeyValuePair>,
    shard_manager: web::Data<Arc<Mutex<ShardManager>>>,
) -> Result<HttpResponse> {
    let key = &item.key;
    let value = &item.value;

    let mut locked_shard_manager = shard_manager.lock().unwrap();
    let shard_index = locked_shard_manager.set(key.clone(), value.clone());
    Ok(HttpResponse::Ok().json(format!(
        "Added key: {}, with value: {} to shard: {}",
        key, value, shard_index
    )))
}

// Delete the key-value pair
async fn delete_key(
    path: web::Path<String>,
    shard_manager: web::Data<Arc<Mutex<ShardManager>>>,
) -> impl Responder {
    let key = path.into_inner();

    let mut locked_shard_manager = shard_manager.lock().unwrap();
    locked_shard_manager.delete(&key);
    // TODO: Delete the key from the sharded HashMap
    HttpResponse::Ok().json(format!("Deleted key: {}", key))
}

pub fn run(listener: TcpListener) -> std::io::Result<Server> {
    let shard_manager = Arc::new(Mutex::new(ShardManager::new(SHARD_COUNT)));

    let server = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(shard_manager.clone()))
            .route("/api/{key}", web::get().to(get_value))
            .route("/api", web::post().to(add_key_value))
            .route("/api/{key}", web::delete().to(delete_key))
    })
    .listen(listener)?
    .run();

    Ok(server)
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{test, web, App};
    use tokio;

    #[tokio::test]
    async fn test_get_value_existing() {
        let shard_manager = Arc::new(Mutex::new(ShardManager::new(SHARD_COUNT)));
        let mut locked_shard_manager = shard_manager.lock().unwrap();
        locked_shard_manager.set("key1".to_string(), "value1".to_string());
        drop(locked_shard_manager); // Release the lock

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(shard_manager.clone()))
                .route("/api/{key}", web::get().to(get_value)),
        )
        .await;

        let req = test::TestRequest::get().uri("/api/key1").to_request();
        let resp = test::call_service(&app, req).await;

        assert_eq!(resp.status(), StatusCode::OK);

        let body = test::read_body(resp).await;
        assert_eq!(body, r#""value1""#);
    }

    #[tokio::test]
    async fn test_get_value_non_existing() {
        let shard_manager = Arc::new(Mutex::new(ShardManager::new(SHARD_COUNT)));

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(shard_manager.clone()))
                .route("/api/{key}", web::get().to(get_value)),
        )
        .await;

        let req = test::TestRequest::get()
            .uri("/api/non_existent_key")
            .to_request();
        let resp = test::call_service(&app, req).await;

        assert_eq!(resp.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_add_key_value() {
        let shard_manager = Arc::new(Mutex::new(ShardManager::new(SHARD_COUNT)));

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(shard_manager.clone()))
                .route("/api", web::post().to(add_key_value)),
        )
        .await;

        let kv = KeyValuePair {
            key: "k".to_string(),
            value: "v".to_string(),
        };

        // Calculate the expected shard index
        let expected_shard_index = {
            let mut locked_shard_manager = shard_manager.lock().unwrap();
            locked_shard_manager.get_shard_index(&kv.key)
        };

        let req = test::TestRequest::post()
            .uri("/api")
            .set_json(&kv)
            .to_request();
        let resp = test::call_service(&app, req).await;

        assert_eq!(resp.status(), StatusCode::OK);

        let body = test::read_body(resp).await;
        let expected_response = format!(
            r#""Added key: k, with value: v to shard: {}""#,
            expected_shard_index
        );
        assert_eq!(body, expected_response);
    }

    #[tokio::test]
    async fn test_delete_key() {
        let shard_manager = Arc::new(Mutex::new(ShardManager::new(SHARD_COUNT)));
        let mut locked_shard_manager = shard_manager.lock().unwrap();
        locked_shard_manager.set("key1".to_string(), "value1".to_string());
        drop(locked_shard_manager); // Release the lock

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(shard_manager.clone()))
                .route("/api/{key}", web::delete().to(delete_key)),
        )
        .await;

        let req = test::TestRequest::delete().uri("/api/key1").to_request();

        let resp = test::call_service(&app, req).await;

        assert_eq!(resp.status(), StatusCode::OK);

        let body = test::read_body(resp).await;
        assert_eq!(body, r#""Deleted key: key1""#);
    }
}
