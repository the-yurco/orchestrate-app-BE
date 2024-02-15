use actix_web::{web, App, HttpServer};
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

extern crate diesel;

pub mod schema;

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL not set");
    SqliteConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use schema::{folders, files}; 

    let _ = establish_connection();

    HttpServer::new(|| {
        App::new()
            .service(web::resource("/").to(|| async { "Hello, Actix!" }))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
