
pub mod models;
pub mod routes;
pub mod database;
pub mod repository;

use actix_web::{App, HttpServer, web};

use database::establish_connection;


#[actix_web::main]
async fn main() -> std::io::Result<()> {



            let db_pool = establish_connection().await;
            HttpServer::new(move || {
                App::new()
                    .app_data(web::Data::new(db_pool.clone()))
                    .configure(routes::config)
            })
            .bind(("127.0.0.1",8080))?
            .run()
            .await
}


