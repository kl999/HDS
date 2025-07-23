use std::net::UdpSocket;
use std::io;

use crate::message::Message;
use crate::socket_worker::SocketWorker;

mod socket_worker;
mod message;

#[cfg(test)]
mod tests;

fn main() {
    _message_test();

    //_socket_test();

    println!("End!");
}

fn _socket_test() {
    println!("Server or client?");
    
    let mut cmd = String::new();
    
    io::stdin().read_line(&mut cmd).expect("Error read_line");
    
    match cmd.to_lowercase().trim() {
        "server" => {
            let sock = UdpSocket::bind("127.0.0.1:8080").unwrap();
            sock.set_nonblocking(true).expect("on set nonblocking");
    
            let mut worker = SocketWorker::new(sock, |msg| println!("{}", msg));
    
            loop {
                worker.work();
            }
        }
        "client" => {
            let sock = UdpSocket::bind("127.0.0.1:0").unwrap();
            sock.connect("127.0.0.1:8080").expect("Not connected!");
            sock.set_nonblocking(true).expect("on set nonblocking");
    
            let mut worker = SocketWorker::new(sock, |msg| println!("{}", msg));
    
            worker.send_message("Hello".to_string());
    
            loop {
                worker.work();
            }
        }
        _ => { println!("Wrong command!") }
    }
    }

fn _message_test() {
    let m1 = Message::new(1, format!("hi!!!").as_bytes().to_vec().into_boxed_slice());

    println!("{}", m1);
}
