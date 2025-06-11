use actix_web::{web, HttpResponse, Responder};
use crate::models::note::{NewNote, Note, UpdateNote};
use crate::repository::note_repository::{create_note, update_note, get_all_notes, get_note,delete_note};
use sqlx::PgPool;

//create_note
//get_all_notes
//update_note
//get_note
//delete_note
//use crate::models::note;


pub async  fn create_user_note(
     pool: web::Data<PgPool>,
     note: web::Json<NewNote>,
    ) -> impl Responder {
    create_note(pool,note).await
}

pub async fn get_all_notes_db(
    pool: web::Data<PgPool>,
) -> impl Responder{

     get_all_notes(pool).await
}

pub async fn update_user_note(
    pool:web::Data<PgPool>,
    note: web::Json<UpdateNote>,
) -> impl Responder {

   update_note(pool, note).await
   
}

pub async fn get_note_noteid(
    pool: web::Data<PgPool>,
    note_id: web::Path<i32>,
) -> impl Responder{
    get_note(pool, note_id).await

}

pub async fn delete_note_noteid(
    pool: web::Data<PgPool>,
    note_id: web::Path<i32>
) -> impl Responder{
    delete_note(pool, note_id).await

}