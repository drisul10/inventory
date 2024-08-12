pub mod schema;

use actix_web::{web, HttpResponse, Responder};
use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::schema::items::dsl::*;

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[derive(Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::items)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Item {
    pub id: Uuid,
    pub sequence_id: i64,
    pub name: String,
    pub unit: String,
    pub stock: f64,
    pub rack: Option<String>,
    pub location: Option<String>,
    pub is_deleted: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Insertable, AsChangeset, Deserialize, Serialize)]
#[diesel(table_name = crate::schema::items)]
pub struct NewItem {
    pub name: String,
    pub unit: String,
    pub stock: f64,
    pub rack: Option<String>,
    pub location: Option<String>,
}

pub async fn create_item(pool: web::Data<DbPool>, item: web::Json<NewItem>) -> impl Responder {
    let mut conn = pool.get().expect("couldn't get db connection from pool");
    let new_item = diesel::insert_into(items)
        .values(&item.into_inner())
        .get_result::<Item>(&mut conn);
    match new_item {
        Ok(item) => HttpResponse::Created().json(item),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn get_items(pool: web::Data<DbPool>) -> impl Responder {
    let mut conn = pool.get().expect("couldn't get db connection from pool");
    let items_result = items
        .filter(is_deleted.eq(false))
        .load::<Item>(&mut conn);
    match items_result {
        Ok(items_list) => HttpResponse::Ok().json(items_list),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn get_item(pool: web::Data<DbPool>, item_id: web::Path<Uuid>) -> impl Responder {
    let mut conn = pool.get().expect("couldn't get db connection from pool");
    let item_result = items
        .filter(id.eq(item_id.into_inner()))
        .filter(is_deleted.eq(false))
        .first::<Item>(&mut conn);
    match item_result {
        Ok(item) => HttpResponse::Ok().json(item),
        Err(diesel::result::Error::NotFound) => HttpResponse::NotFound().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn update_item(pool: web::Data<DbPool>, item_id: web::Path<Uuid>, item: web::Json<NewItem>) -> impl Responder {
    let mut conn = pool.get().expect("couldn't get db connection from pool");
    let updated_item = diesel::update(items)
        .filter(id.eq(item_id.into_inner()))
        .set(item.into_inner())
        .get_result::<Item>(&mut conn);
    match updated_item {
        Ok(item) => HttpResponse::Ok().json(item),
        Err(diesel::result::Error::NotFound) => HttpResponse::NotFound().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn delete_item(pool: web::Data<DbPool>, item_id: web::Path<Uuid>) -> impl Responder {
    let mut conn = pool.get().expect("couldn't get db connection from pool");
    let deleted_item = diesel::update(items)
        .filter(id.eq(item_id.into_inner()))
        .set(is_deleted.eq(true))
        .get_result::<Item>(&mut conn);
    match deleted_item {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(diesel::result::Error::NotFound) => HttpResponse::NotFound().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}