use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use std::{thread, time};

pub fn update(period: u64, peers: Arc<Mutex<Vec<SocketAddr>>>) {
    loop {
        thread::sleep(time::Duration::from_secs(period));
        let list = peers.lock().unwrap();
        for address in list.iter() {
            let _ = reqwest::blocking::get(address.to_string() + "/ping");
        }
        println!("Sending message to {:?}", list);
    }
}
