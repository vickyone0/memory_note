use actix_web::{web,HttpResponse, Responder};
use sqlx::PgPool;
use crate::models::user::{User,NewUser};




pub async fn create_user(
    pool: web::Data<PgPool>,
    user: web::Json<NewUser>,
) -> impl Responder {
    let result = sqlx::query!(
        "INSERT INTO users (name) VALUES ($1) RETURNING id, name",
        user.name
    )
    .fetch_one(pool.get_ref())
    .await;

    match result {
        Ok(record) => {
            let user_response = User {
                id: record.id,
                name: record.name,
            };
            HttpResponse::Ok().json(user_response)
        },
        Err(e) => {
            eprintln!("Failed to insert the user: {}", e);
            HttpResponse::InternalServerError().body("Failed to create user")
        }
    }
}

