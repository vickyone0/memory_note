//register_user
use actix_web::{web, Responder};
use crate::models::user::User;
use crate::repository::user_repository::create_user;
use crate::models::user::NewUser;
use sqlx::PgPool;


 pub async fn register_user(
    pool: web::Data<PgPool>,
    user: web::Json<NewUser>,
) -> impl Responder {
    create_user(pool, user).await
}