//have 2 entities user, note 
//user one to many for note
//create userr --> create note --> check notes --> edit note --> check note --> delete note 

// end points are user reg - POST user , create note - POST user/note , 
//check notes - GET user/notes , edit note - PUT user/note?{id} , check note - GET user/note?{id} 
//delete note - DELETE user/note
mod user;
mod note;

use actix_web::web;
use user::register_user;
use note::{create_note,  update_note, get_note, delete_note, get_all_notes};





pub fn config(cfg: &mut web::ServiceConfig){
    cfg.service(
        web::scope("/user")
                    .route("", web::post().to(register_user))
                    .service(
                        web::scope("/note")
                                    .route("", web::post().to(create_note))
                                    .route("", web::put().to(update_note))
                                    .route("", web::get().to(get_note))
                                    .route("",web::delete().to(delete_note))

                    )
                    .route("/notes", web::get().to(get_all_notes) )


    );
}

//register_user
//create_note
//get_all_notes
//update_note
//get_note
//delete_note