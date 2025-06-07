use actix_web::{web, Responder};

//create_note
//get_all_notes
//update_note
//get_note
//delete_note
//use crate::models::note;

pub async  fn create_note() -> impl Responder {

    web::Json("note created")
   
}

pub async fn get_all_notes() -> impl Responder{
    web::Json("listed all notes")

}

pub async fn update_note() -> impl Responder {

    web::Json("upated noted")
}

pub async fn get_note() -> impl Responder{

    web::Json("get note")

}

pub async fn delete_note() -> impl Responder{
    web::Json("note deleted")

}