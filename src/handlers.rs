use crate::data_store;
use crate::models::NavItem;
use actix_web::{web, HttpResponse, Responder};

pub async fn get_navbar_items() -> Result<Vec<NavItem>, actix_web::Error> {
    // Simulate fetching data from a data store or database
    let nav_items = data_store::get_navbar_items();
    Ok(nav_items)
}

pub fn scoped_config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("").route(web::get().to(get_navbar_items_responder)));
}

async fn get_navbar_items_responder() -> impl Responder {
    match get_navbar_items().await {
        Ok(items) => HttpResponse::Ok().json(items),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
