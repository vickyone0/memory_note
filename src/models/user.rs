use serde::{Serialize,Deserialize};

#[derive(Serialize,Deserialize)]
pub struct User{
    pub id: i32,
    pub name: String,
}


#[derive(Serialize,Deserialize)]
pub struct NewUser {
    pub name: String,
}