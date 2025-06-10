

#[derive(serde::Serialize)]
pub struct User{
    pub id: i32,
    pub name: String,
}


#[derive(serde::Deserialize)]
pub struct NewUser {
    pub name: String,
}