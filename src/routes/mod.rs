use actix_web::web;

use crate::handlers::{create_user, get_users, hello_world, find_user};

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/users")
            .route(web::get().to(get_users))
            .route(web::post().to(create_user))
    );
    cfg.service(web::resource("/hello").route(web::get().to(hello_world)));
    cfg.service(
        web::resource("/users/{user_id}")
            .route(web::get().to(find_user)),
    );
}
