use actix_web::{web, HttpResponse, Responder};

pub async fn posts() -> impl Responder {
    HttpResponse::Ok().body("This is the posts API endpoint")
}
