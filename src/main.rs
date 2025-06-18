
pub mod models;
pub mod routes;
pub mod database;
pub mod repository;
pub mod auth;
pub mod middleware;

use actix_web::{web, App, HttpRequest, HttpServer};

use database::establish_connection;


// #[actix_web::main]
// async fn main() -> std::io::Result<()> {



//             let db_pool = establish_connection().await;
//             HttpServer::new(move || {
//                 App::new()
//                     .app_data(web::Data::new(db_pool.clone()))
//                     .configure(routes::config)
//             })
//             .bind(("127.0.0.1",8080))?
//             .run()
//             .await
// }


//use actix_web::{web, App, HttpServer, Responder};
use actix_web_actors::ws;

struct MyWebSocket;

impl StreamHandler<Result<ws::HttpMessage, ws::ProtocolError>> for MyWebSocket {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Text(text)) => ctx.text(text),
            Ok(ws::Message::Close(reason)) => ctx.close(reason),
            _ => (),
            
        }
    }
}

async fn websocket_route(req: HttpRequest, stream: web::Payload) -> impl Responder {
    ws::start(MyWebSocket {}, &req, stream)
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/ws", web::get().to(websocket_route))  
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}



