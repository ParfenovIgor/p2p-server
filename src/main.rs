use clap::{Arg, Command};
use std::net::SocketAddr;

use crate::server::init_server;

pub mod server;

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

    let period: usize = arguments.get_one::<String>("period").unwrap().trim().parse().expect("Invalid period number");
    let port: usize = arguments.get_one::<String>("port").unwrap().trim().parse().expect("Invalid port number");

    if let Some(str) = arguments.get_one::<String>("connect") {
        let connect: SocketAddr = str.parse().expect("Invalid connect address");
    }

    let _ = init_server(port);
}
