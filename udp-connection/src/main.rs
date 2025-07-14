use std::net::UdpSocket;
use std::io;

mod udp_socket;

fn main() {
    println!("Server or client?");

    let mut cmd = String::new();

    io::stdin().read_line(&mut cmd).expect("Error read_line");

    match cmd.to_lowercase().trim() {
        "server" => {
            let sock = UdpSocket::bind("127.0.0.1:8080").unwrap();

            loop {
                let mut buf = [0; 1024];
                let (number_of_bytes, src_addr) = sock.recv_from(&mut buf).expect("Didn't receive data");
                let msg = String::from_utf8_lossy(&buf[..number_of_bytes]).to_string();
                println!(
                    "Received {} bytes from {}: '{}'",
                    number_of_bytes, src_addr, msg
                );
            }
        }
        "client" => {
            let sock = UdpSocket::bind("127.0.0.1:0").unwrap();
            sock.connect("127.0.0.1:8080").expect("Not connected!");

            let buf = format!("Hello!");
            sock.send_to(buf.as_bytes(), "127.0.0.1:8080").unwrap();
        }
        _ => { println!("Wrong command!") }
    }

    println!("End!");
}
