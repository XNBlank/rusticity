use actix_web::web;

pub mod index;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(index::index);
}