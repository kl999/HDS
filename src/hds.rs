use std::{collections::HashMap, net::UdpSocket, sync::mpsc::channel, thread};

use crate::msg_exchange::{Msg, MsgExchange};

pub fn start(mx: MsgExchange) {
    thread::spawn(move || {
        start_in_thread(mx);
    });
}

fn start_in_thread(mx: MsgExchange) {
    let sock = UdpSocket::bind("127.0.0.1:8080").unwrap();

    let mut mxs = vec![];
    let mut state: HashMap<String, String> = HashMap::new();

    loop {
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

            let (soc_mx, con_mx) = MsgExchange::make_pair();

            mxs.push(soc_mx);

            thread::spawn(move || {
                use_thread(con, con_mx);
            });
        }
    }
}

fn use_thread(con: UdpSocket, mx: MsgExchange) {
    mx.snd.send(Msg::new("msg".to_string(), "hello".to_string())).unwrap();
    let buf = [1];
    con.send(&buf).unwrap();
    let msg = mx.rcv.recv().unwrap();
    println!("{:?}", msg);
}
