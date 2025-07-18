use std::net::UdpSocket;
use std::io;

use crate::socket_worker::SocketWorker;

mod socket_worker;

fn main() {
    println!("Server or client?");

    let mut cmd = String::new();

    io::stdin().read_line(&mut cmd).expect("Error read_line");

    match cmd.to_lowercase().trim() {
        "server" => {
            let sock = UdpSocket::bind("127.0.0.1:8080").unwrap();

            let mut worker = SocketWorker::new(sock, |msg| println!("{}", msg));

            loop {
                worker.work();
            }
        }
        "client" => {
            let sock = UdpSocket::bind("127.0.0.1:0").unwrap();
            sock.connect("127.0.0.1:8080").expect("Not connected!");

            let mut worker = SocketWorker::new(sock, |msg| println!("{}", msg));

            worker.send_message("Hello".to_string());

            loop {
                worker.work();
            }
        }
        _ => { println!("Wrong command!") }
    }

    println!("End!");
}
