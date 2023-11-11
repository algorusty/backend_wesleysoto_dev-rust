use actix_web::{web, HttpResponse, Responder};

use crate::handlers;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.route("/", web::get().to(get_navbar_items));
}

async fn get_navbar_items() -> impl Responder {
    match handlers::get_navbar_items().await {
        Ok(items) => HttpResponse::Ok().json(items),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
