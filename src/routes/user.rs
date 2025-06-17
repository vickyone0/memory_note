//register_user
use actix_web::{web, HttpResponse, Responder};
use serde::Serialize;
use crate::repository::user_repository::create_user;
use crate::models::user::NewUser;
use sqlx::PgPool;
use crate::auth::create_jwt;


#[derive(Serialize)]
struct RegisterResponse {
    id: i32,
    name: String,
    token: String,
}


 pub async fn register_user(
    pool: web::Data<PgPool>,
    user: web::Json<NewUser>,
) -> impl Responder {
    let result =create_user(pool, user).await;

    match  result {
        Ok(user) => {
            let exp = chrono::Utc::now().timestamp() as usize + 24 * 3600;
            let token = create_jwt(user.name.clone(), exp).unwrap_or_default();
            let response = RegisterResponse {
                id: user.id,
                name: user.name,
                token,
            };
            HttpResponse::Ok().json(response)

        }
        Err(e) => {
            HttpResponse::InternalServerError().body(format!("Failed to register user: {}",e))
        }
    }
}