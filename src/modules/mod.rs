use actix_web::web;
use crate::controller;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/api").configure(controller::user_controller::configure));
}
