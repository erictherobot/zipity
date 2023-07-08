// src/main.rs

use actix_web::{ web, App, HttpServer, middleware::Logger };
use actix_web::middleware::Compress;
use env_logger::Env;

mod root_handler;
mod page_handler;
mod sitemap_handler;
mod build_handler;
mod robots_handler;
mod api_handler;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Set up logger
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    println!("Server is running on http://127.0.0.1:8080");

    // Build the static HTML files
    build_handler::build_static_files().unwrap();

    // Start the server
    HttpServer::new(|| {
        App::new()
            .wrap(Compress::default()) // Enable compression middleware
            .wrap(Logger::default()) // Enable logger middleware
            .service(web::resource("/").route(web::get().to(root_handler::root)))
            .service(web::resource("/sitemap.xml").route(web::get().to(sitemap_handler::sitemap)))
            .service(web::resource("/robots.txt").route(web::get().to(robots_handler::robots)))
            .service(web::resource("/{name}").route(web::get().to(page_handler::page)))
            .service(actix_files::Files::new("/static", "./static").show_files_listing())
            .service(web::resource("/api/{name}").route(web::get().to(api_handler::api_handler)))
            .service(actix_files::Files::new("/static", "./out").show_files_listing()) // Or use "./dist" if you prefer
    })
        .bind("127.0.0.1:8080")?
        .run().await
}
