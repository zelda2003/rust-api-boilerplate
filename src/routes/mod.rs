use actix_web::web;

use crate::handlers::{create_user, get_users};

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/users").route(web::get().to(get_users)));
    cfg.service(web::resource("/users").route(web::post().to(create_user)));
}
