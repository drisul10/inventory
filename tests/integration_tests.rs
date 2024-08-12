use actix_web::{test, web, App};
use diesel::r2d2::{self, ConnectionManager};
use diesel::PgConnection;
use dotenv::dotenv;
use inventory::{create_item, get_items, get_item, update_item, delete_item, NewItem, Item, DbPool};
use std::env;

fn setup_test_db() -> DbPool {
    dotenv::from_filename(".env").ok();
    let database_url = env::var("DATABASE_URL").expect("Auto testing DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.")
}

#[actix_rt::test]
async fn test_create_item() {
    let pool = setup_test_db();
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .route("/items", web::post().to(create_item))
    ).await;

    let new_item = NewItem {
        name: "Test Item".to_string(),
        unit: "pcs".to_string(),
        stock: 10.0,
        rack: Some("A1".to_string()),
        location: Some("Warehouse 1".to_string()),
    };

    let req = test::TestRequest::post()
        .uri("/items")
        .set_json(&new_item)
        .to_request();
    let resp = test::call_service(&app, req).await;

    assert!(resp.status().is_success());

    let item: Item = test::read_body_json(resp).await;
    assert_eq!(item.name, "Test Item");
    assert_eq!(item.stock, 10.0);
}

#[actix_rt::test]
async fn test_get_items() {
    let pool = setup_test_db();
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .route("/items", web::get().to(get_items))
    ).await;

    let req = test::TestRequest::get().uri("/items").to_request();
    let resp = test::call_service(&app, req).await;

    assert!(resp.status().is_success());

    let items: Vec<Item> = test::read_body_json(resp).await;
    assert!(!items.is_empty());
}

#[actix_rt::test]
async fn test_get_item() {
    let pool = setup_test_db();
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .route("/items/{id}", web::get().to(get_item))
    ).await;

    // First, create an item
    let new_item = NewItem {
        name: "Test Item for Get".to_string(),
        unit: "pcs".to_string(),
        stock: 5.0,
        rack: None,
        location: None,
    };

    let create_app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .route("/items", web::post().to(create_item))
    ).await;

    let create_req = test::TestRequest::post()
        .uri("/items")
        .set_json(&new_item)
        .to_request();
    let create_resp = test::call_service(&create_app, create_req).await;
    let created_item: Item = test::read_body_json(create_resp).await;

    // Now, test getting the item
    let req = test::TestRequest::get()
        .uri(&format!("/items/{}", created_item.id))
        .to_request();
    let resp = test::call_service(&app, req).await;

    assert!(resp.status().is_success());

    let item: Item = test::read_body_json(resp).await;
    assert_eq!(item.id, created_item.id);
    assert_eq!(item.name, "Test Item for Get");
}

#[actix_rt::test]
async fn test_update_item() {
    let pool = setup_test_db();
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .route("/items", web::post().to(create_item))
            .route("/items/{id}", web::put().to(update_item))
    ).await;

    // First, create an item
    let new_item = NewItem {
        name: "Test Item for Update".to_string(),
        unit: "pcs".to_string(),
        stock: 5.0,
        rack: None,
        location: None,
    };

    let create_req = test::TestRequest::post()
        .uri("/items")
        .set_json(&new_item)
        .to_request();
    let create_resp = test::call_service(&app, create_req).await;
    let created_item: Item = test::read_body_json(create_resp).await;

    // Now, update the item
    let update_item = NewItem {
        name: "Updated Test Item".to_string(),
        unit: "kg".to_string(),
        stock: 10.0,
        rack: Some("B2".to_string()),
        location: Some("Warehouse 2".to_string()),
    };

    let update_req = test::TestRequest::put()
        .uri(&format!("/items/{}", created_item.id))
        .set_json(&update_item)
        .to_request();
    let update_resp = test::call_service(&app, update_req).await;

    assert!(update_resp.status().is_success());

    let updated_item: Item = test::read_body_json(update_resp).await;
    assert_eq!(updated_item.id, created_item.id);
    assert_eq!(updated_item.name, "Updated Test Item");
    assert_eq!(updated_item.stock, 10.0);
}

#[actix_rt::test]
async fn test_delete_item() {
    let pool = setup_test_db();
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .route("/items", web::post().to(create_item))
            .route("/items/{id}", web::delete().to(delete_item))
            .route("/items/{id}", web::get().to(get_item))
    ).await;

    // First, create an item
    let new_item = NewItem {
        name: "Test Item for Delete".to_string(),
        unit: "pcs".to_string(),
        stock: 5.0,
        rack: None,
        location: None,
    };

    let create_req = test::TestRequest::post()
        .uri("/items")
        .set_json(&new_item)
        .to_request();
    let create_resp = test::call_service(&app, create_req).await;
    let created_item: Item = test::read_body_json(create_resp).await;

    // Now, delete the item
    let delete_req = test::TestRequest::delete()
        .uri(&format!("/items/{}", created_item.id))
        .to_request();
    let delete_resp = test::call_service(&app, delete_req).await;

    assert!(delete_resp.status().is_success());

    // Try to get the deleted item
    let get_req = test::TestRequest::get()
        .uri(&format!("/items/{}", created_item.id))
        .to_request();
    let get_resp = test::call_service(&app, get_req).await;

    assert_eq!(get_resp.status(), 404);
}