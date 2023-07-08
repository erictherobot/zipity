// src/api_handler.rs

use actix_web::{ web, HttpResponse, Responder };

pub async fn api_handler(path: web::Path<String>) -> impl Responder {
    let name = &path.into_inner();
    let content = get_api_content(name);

    HttpResponse::Ok().json(content)
}

fn get_api_content(name: &str) -> serde_json::Value {
    let mut content = serde_json::Map::new();
    content.insert("name".to_owned(), serde_json::Value::String(name.to_owned()));
    content.insert("message".to_owned(), serde_json::Value::String(format!("Hello, {}!", name)));

    serde_json::Value::Object(content)
}
