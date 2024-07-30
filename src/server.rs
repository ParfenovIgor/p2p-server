use actix_web::{web, App, HttpServer, HttpRequest, HttpResponse};

async fn connect(req: HttpRequest) -> HttpResponse {
    println!("Received request for connection from \"{}\"", req.peer_addr().unwrap());
    let response = serde_json::json!([
        req.peer_addr().unwrap()
    ]);
    HttpResponse::Ok().json(response)
}

async fn ping(req: HttpRequest) -> HttpResponse {
    println!("Received message from \"{}\"", req.peer_addr().unwrap());
    HttpResponse::Ok().finish()
}

#[actix_web::main]
pub async fn init_server(port: usize) -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/connect", web::get().to(connect))
            .route("/ping", web::get().to(ping))
    })
    .bind(format!("127.0.0.1:{port}"))?
    .run()
    .await
}
