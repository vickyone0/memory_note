use std::env;
use sqlx::{Pool, Postgres, postgres::PgPoolOptions};

pub async fn establish_connection() ->Pool<Postgres>{

    dotenv::dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE url must be set");

    PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Falied to create pool")

}