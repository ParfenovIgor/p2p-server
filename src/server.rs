use actix_web::{web, App, HttpServer, HttpRequest, HttpResponse};
use std::net::SocketAddr;
use serde::Deserialize;

#[derive(Deserialize)]
struct ArgInput {
    address: SocketAddr,
}

async fn connect(req: HttpRequest, item: web::Json<ArgInput>) -> HttpResponse {
    println!("Received request for connection from \"{}\"", item.address);
    let response = serde_json::json!([
        item.address
    ]);
    HttpResponse::Ok().json(response)
}

async fn ping(req: HttpRequest, item: web::Json<ArgInput>) -> HttpResponse {
    println!("Received message from \"{}\"", item.address);
    HttpResponse::Ok().finish()
}

#[actix_web::main]
pub async fn init_server(port: usize) -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/connect", web::post().to(connect))
            .route("/ping", web::post().to(ping))
    })
    .bind(format!("127.0.0.1:{port}"))?
    .run()
    .await
}
