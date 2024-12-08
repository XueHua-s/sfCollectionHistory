use crate::controller;
use actix_web::web;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/api").configure(controller::user_controller::configure))
        .configure(controller::book_controller::configure);
}
