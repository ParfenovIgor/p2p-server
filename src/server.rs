use actix_web::{web, App, HttpServer, HttpResponse};
use std::net::SocketAddr;
use serde::Deserialize;
use std::sync::{Arc, Mutex};

#[derive(Deserialize)]
struct ArgInput {
    address: SocketAddr,
}

struct AppState {
    peers: Arc<Mutex<Vec<SocketAddr>>>,
}

async fn connect(data: web::Data<AppState>, item: web::Json<ArgInput>) -> HttpResponse {
    let mut peers = data.peers.lock().unwrap();
    println!("Received request for connection from \"{}\"", item.address);
    let response = serde_json::json!(*peers);
    (*peers).push(item.address);
    HttpResponse::Ok().json(response)
}

async fn ping(_data: web::Data<AppState>, item: web::Json<ArgInput>) -> HttpResponse {
    println!("Received message from \"{}\"", item.address);
    HttpResponse::Ok().finish()
}

#[actix_web::main]
pub async fn init_server(port: usize, peers: Arc<Mutex<Vec<SocketAddr>>>) -> std::io::Result<()> {
    let shared_state = web::Data::new(AppState {
        peers: peers
    });

    HttpServer::new(move || {
        App::new()
            .app_data(shared_state.clone())
            .route("/connect", web::post().to(connect))
            .route("/ping", web::post().to(ping))
    })
    .bind(format!("127.0.0.1:{port}"))?
    .run()
    .await
}
