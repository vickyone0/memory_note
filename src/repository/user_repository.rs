use actix_web::{web,HttpResponse, Responder};
use sqlx::PgPool;
use crate::models::user::{User,NewUser};




pub async fn create_user(
    pool: web::Data<PgPool>,
    user: web::Json<NewUser>,
) -> Result<User, sqlx::Error> {
    let record = sqlx::query!(
        "INSERT INTO users (name) VALUES ($1) RETURNING id, name",
        user.name
    )
    .fetch_one(pool.get_ref())
    .await?;

    Ok(User {
        id: record.id,
        name: record.name,
    })
}
