use serde::Deserialize;
use std::net::SocketAddr;

#[derive(Deserialize)]
struct PeersOutput {
    peers: Vec<SocketAddr>,
}

pub fn connect(address: SocketAddr, port: u64) -> Vec<SocketAddr> {
    let url: String = format!("http://{}/connect", address);
    let data = serde_json::json!({
        "address": format!("127.0.0.1:{}", port)
    });
    let client = reqwest::blocking::Client::new();
    let response = client
        .post(url)
        .header(reqwest::header::CONTENT_TYPE, "application/json")
        .body(data.to_string())
        .send()
        .expect(&format!("The address {} is unreachable", address))
        .text()
        .unwrap();
    let mut list: Vec<SocketAddr> = serde_json::from_str::<PeersOutput>(&response)
        .unwrap()
        .peers;

    for address in list.iter() {
        let url: String = format!("http://{}/connect", address);
        let _ = client
            .post(url)
            .header(reqwest::header::CONTENT_TYPE, "application/json")
            .body(data.to_string())
            .send();
    }

    list.push(address);
    println!(
        "# {} - Connected to the peers at {}",
        chrono::Local::now().format("%H:%M:%S"),
        format!(
            "{:?}",
            list.iter()
                .map(|addr| format!("{}", addr))
                .collect::<Vec<String>>()
        )
    );
    list
}
