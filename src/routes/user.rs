//register_user
use actix_web::{web, Responder};
use crate::models::user::User;


 pub async fn register_user() -> impl Responder{

    let user = User {  
        id: 1,
        name: "vignesh".to_string()
    };

    web::Json(user)
     
}