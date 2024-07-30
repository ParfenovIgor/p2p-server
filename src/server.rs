use actix_web::{web, App, HttpServer, HttpRequest, HttpResponse};
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};

struct AppState {
    peers: Arc<Mutex<Vec<SocketAddr>>>,
}

async fn connect(data: web::Data<AppState>, req: HttpRequest) -> HttpResponse {
    let address = req.peer_addr().unwrap();
    let mut peers = data.peers.lock().unwrap();
    println!("Received request for connection from \"{}\"", address);
    let response = serde_json::json!(*peers);
    (*peers).push(address);
    HttpResponse::Ok().json(response)
}

async fn ping(_data: web::Data<AppState>, req: HttpRequest) -> HttpResponse {
    println!("Received message from \"{}\"", req.peer_addr().unwrap());
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
            .route("/connect", web::get().to(connect))
            .route("/ping", web::get().to(ping))
    })
    .bind(format!("127.0.0.1:{port}"))?
    .run()
    .await
}
