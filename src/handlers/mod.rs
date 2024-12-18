use crate::models::CreateUserRequest;
use crate::services;
use actix_web::{web, HttpResponse, Responder};

pub async fn create_user(
    db_pool: web::Data<sqlx::PgPool>,
    user_data: web::Json<CreateUserRequest>,
) -> impl Responder {
    match services::create_user(db_pool.get_ref(), user_data.into_inner()).await {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(services::ServiceError::DatabaseError(msg)) => {
            HttpResponse::InternalServerError().body(msg)
        }
        Err(services::ServiceError::ValidationError(msg)) => HttpResponse::BadRequest().body(msg),
    }
}

pub async fn get_users(db_pool: web::Data<sqlx::PgPool>) -> impl Responder {
    match services::get_all_users(db_pool.get_ref()).await {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(err) => HttpResponse::InternalServerError().body(format!("Error: {:?}", err)),
    }
}
