use clap::{Arg, Command};
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use std::thread;

use crate::server::init_server;
use crate::update::update;
use crate::connect::connect;

pub mod server;
pub mod update;
pub mod connect;

fn main() {
    let arguments = Command::new("p2p-server")
        .arg(Arg::new("period")
            .long("period")
            .short('t')
            .action(clap::ArgAction::Set)
            .required(true)
            .help("Specifies the period of sending messages to other peers, s"))
        .arg(Arg::new("port")
            .long("port")
            .short('p')
            .action(clap::ArgAction::Set)
            .required(true)
            .help("Specifies the port, which the server will listen"))
        .arg(Arg::new("connect")
            .long("connect")
            .short('c')
            .action(clap::ArgAction::Set)
            .help("Specifies the ip address and the port of the server, to which this server has to connect"))
        .get_matches();
        
    let period: u64 = arguments.get_one::<String>("period").unwrap().trim().parse().expect("Invalid period number");
    let port: u64 = arguments.get_one::<String>("port").unwrap().trim().parse().expect("Invalid port number");
    
    let list: Vec<SocketAddr> = if let Some(str) = arguments.get_one::<String>("connect") {
        let address: SocketAddr = str.parse().expect("Invalid connect address");
        connect(address, port)
    }
    else {
        Vec::new()
    };
  
    let peers = Arc::new(Mutex::new(list));
    let peers_clone = Arc::clone(&peers);
    let _ = thread::spawn(move || {
        update(period, port, peers_clone);
    });

    let _ = init_server(port, peers);
}
