use actix_web::{web, App, HttpServer};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use dotenv::dotenv;
use std::env;
use inventory::{create_item, get_items, get_item, update_item, delete_item};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let bind_address = env::var("BIND_ADDRESS").unwrap_or_else(|_| "127.0.0.1:8080".to_string());
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .route("/items", web::post().to(create_item))
            .route("/items", web::get().to(get_items))
            .route("/items/{id}", web::get().to(get_item))
            .route("/items/{id}", web::put().to(update_item))
            .route("/items/{id}", web::delete().to(delete_item))
    })
    .bind(bind_address)?
    .run()
    .await
}