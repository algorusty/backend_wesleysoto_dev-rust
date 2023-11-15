use crate::data::DataStore;
use actix_web::{web, HttpResponse, Responder, http::StatusCode};
use actix_web::error::InternalError;
use log::error;

pub async fn get_navbar_items() -> Result<Vec<String>, actix_web::Error> {
    let data_store_result = DataStore::new().await;
    match data_store_result {
        Ok(data_store) => {
            let nav_items = data_store.objects().await;
            Ok(nav_items)            
        },
        Err(e) => {
            error!("Failed to initialize DataStore: {:?}", e);
            Err(InternalError::new("An error occurred", StatusCode::INTERNAL_SERVER_ERROR).into())
        },
    }
}

async fn get_navbar_items_responder() -> impl Responder {
    match get_navbar_items().await {
        Ok(items) => HttpResponse::Ok().json(items),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("").route(web::get().to(get_navbar_items_responder)));
}

async fn health_check_responder() -> impl Responder {
    HttpResponse::Ok().finish()
}

pub fn config_health_check(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("").route(web::get().to(health_check_responder)));
}