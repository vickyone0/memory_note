use actix_web::{HttpResponse, Responder};
use sqlx::PgPool;



pub async fn create_user(
    pool: web::Data<PgPool>,
    user: web::Json<NewUser>,
) -> impl Responder {
    let result
}