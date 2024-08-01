use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use std::{thread, time};

pub fn update(period: u64, port: u64, peers: Arc<Mutex<Vec<SocketAddr>>>) {
    let data = serde_json::json!({
        "address": format!("127.0.0.1:{}", port)
    });
    let client = reqwest::blocking::Client::new();
    loop {
        thread::sleep(time::Duration::from_secs(period));
        let list = peers.lock().unwrap();
        for address in list.iter() {
            let url: String = format!("http://{}/ping", address);
            let _ = client
                .post(url)
                .header(reqwest::header::CONTENT_TYPE, "application/json")
                .body(data.to_string())
                .send();
        }
        println!(
            "# {} - Sending message to {}",
            chrono::Local::now().format("%H:%M:%S"),
            format!(
                "{:?}",
                list.iter()
                    .map(|addr| format!("{}", addr))
                    .collect::<Vec<String>>()
            )
        );
    }
}
