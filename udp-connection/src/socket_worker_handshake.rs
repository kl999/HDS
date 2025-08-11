use std::net::UdpSocket;

use crate::socket_worker::SocketWorker;

pub fn receive_handshake(address: String) -> std::io::Result<SocketWorker> {

}

pub fn send_handshake(address: String) -> std::io::Result<SocketWorker> {

}

fn expect_handshake(sock: &UdpSocket) {
    let mut buf = [0; 5];
    let (number_of_bytes, src_addr) = sock.recv_from(&mut buf).expect("Didn't receive data");
    let msg = String::from_utf8_lossy(&buf[..number_of_bytes]).to_string();
    println!(
        "Received {} bytes from {}: '{}'",
        number_of_bytes, src_addr, msg
    );
    if msg == "Hello" {
        println!("handshaking");
        let con = UdpSocket::bind("127.0.0.1:0").unwrap();
        let port = con.local_addr().unwrap().port();
        let buf = format!("Connect port {}", port);
        sock.send_to(buf.as_bytes(), src_addr).unwrap();
        //echo "Hello" | nc -u -w1 127.0.0.1 8080

        let worker = SocketWorker::new(con);
    }
}
