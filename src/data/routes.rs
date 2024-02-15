use crate::models::{Item, NewItem};
use diesel::prelude::*;
use crate::schema::items;  
use actix_web::{web, HttpResponse, Responder};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .route("/items", web::get().to(get_items))
            .route("/items/{id}", web::get().to(get_item))
            .route("/items", web::post().to(create_item)),
    );
}

async fn get_items(pool: web::Data<crate::DbPool>) -> impl Responder {
    let conn = pool.get().expect("Couldn't get DB connection from pool");

    match web::block(move || items::table.load::<Item>(&conn)).await {
        Ok(items) => HttpResponse::Ok().json(items),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

async fn get_item(path: web::Path<i32>, pool: web::Data<crate::DbPool>) -> impl Responder {
    let conn = pool.get().expect("Couldn't get DB connection from pool");
    let item_id = path.into_inner();

    match web::block(move || items::table.find(item_id).first::<Item>(&conn)).await {
        Ok(item) => HttpResponse::Ok().json(item),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}

async fn create_item(item: web::Json<NewItem>, pool: web::Data<crate::DbPool>) -> impl Responder {
    let conn = pool.get().expect("Couldn't get DB connection from pool");

    match web::block(move || diesel::insert_into(items::table).values(item.into_inner()).execute(&conn)).await {
        Ok(_) => HttpResponse::Created(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
