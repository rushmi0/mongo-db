use actix_web::web;

mod index;
mod event;

use index::index_service;
use event::{save_event_service, fetch_event_service};


pub fn service_hub(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1")
            .service(index_service)
            .service(save_event_service)
            .service(fetch_event_service)
    );
}
