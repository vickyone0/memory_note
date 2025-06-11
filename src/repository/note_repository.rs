


use actix_web::{web,HttpResponse, Responder};
use sqlx::PgPool;
use crate::models::note::{NewNote, Note, UpdateNote};


pub async fn create_note(
    pool: web::Data<PgPool>,
    note: web::Json<NewNote>,
) -> impl Responder {

    let result = sqlx::query!(
        r#"
        INSERT INTO notes (user_id, title, description)
        VALUES ($1, $2, $3)
        RETURNING note_id, user_id, title, description, created_at
        "#,
        note.user_id,
        note.title,
        note.description
    )
    .fetch_one(pool.get_ref())
    .await;


    match result {
        Ok(record) => {
            let note_response = Note {
                note_id: record.note_id,
                user_id: record.user_id,
                title: record.title,
                description: record.description,
                created_at: record.created_at,
            };
            HttpResponse::Ok().json(note_response)
        }
        Err(e) => {
            eprint!("Failed to insert note: {}", e);
            HttpResponse::InternalServerError().body("Failed to create note")
        }
    }
}


pub async fn update_note(
    pool: web::Data<PgPool>,
    note: web::Json<UpdateNote>,
) -> impl Responder {
    let result = sqlx::query!(
        r#"
        UPDATE notes
        SET title = COALESCE($2, title),
            description = COALESCE($3, description)
        WHERE note_id = $1
        RETURNING note_id, user_id, title, description, created_at
        "#,
        note.note_id,
        note.title,
        note.description
    )
    .fetch_one(pool.get_ref())
    .await;

    match result {
        Ok(record) => {
            let note_response = Note {
                note_id: record.note_id,
                user_id: record.user_id,
                title: record.title,
                description: record.description,
                created_at: record.created_at,
            };
            HttpResponse::Ok().json(note_response)
        }
        Err(e) => {
            eprint!("Failed to update note: {}", e);
            HttpResponse::InternalServerError().body("Failed to update note")
        }
    }
}



pub async fn get_all_notes(
    pool: web::Data<PgPool>,
) -> impl Responder{
    let result = sqlx::query_as!(
        Note,
        r#"
        SELECT note_id, user_id, title, description, created_at
        FROM notes
        ORDER BY created_at DESC
        "#
    )
    .fetch_all(pool.get_ref())
    .await;

    match result {
        Ok(notes) => HttpResponse::Ok().json(notes),
        Err(e) => {
            eprint!("Failed to fetch notes: {}", e);
            HttpResponse::InternalServerError().body("Failed to fetch notes")
        }
        
    }
}


pub async fn get_note(
    pool: web::Data<PgPool>,
    note_id: web::Path<i32>,
) -> impl Responder {
    let result = sqlx::query_as!(
        Note,
        r#"
        SELECT note_id, user_id, title, description, created_at
        FROM notes
        WHERE note_id = $1
        "#,
        *note_id
    )
    .fetch_optional(pool.get_ref())
    .await;

    match result {
        Ok(Some(note)) => HttpResponse::Ok().json(note),
        Ok(None) => HttpResponse::NotFound().body("Note not found"),
        Err(e) => {
            eprintln!("Failed to fetch note: {}", e);
            HttpResponse::InternalServerError().body("Failed to fetch note")
        }
    }
}


pub async fn delete_note(
    pool: web::Data<PgPool>,
    note_id: web::Path<i32>,
) -> impl Responder {
    let result = sqlx::query!(
        "DELETE FROM notes WHERE note_id = $1",
        *note_id
    )
    .execute(pool.get_ref())
    .await;

    match result {
        Ok(res) if res.rows_affected() > 0 => HttpResponse::Ok().body("Note deleted"),
        Ok(_) => HttpResponse::NotFound().body("Note not found"),
        Err(e) => {
            eprintln!("Failed to delete note: {}", e);
            HttpResponse::InternalServerError().body("Failed to delete note")
        }
    }
}