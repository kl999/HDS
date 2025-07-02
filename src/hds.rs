use std::{
    collections::HashMap,
    net::UdpSocket,
    thread::{self, JoinHandle},
    time::Duration,
};

use crate::msg_exchange::{Msg, MsgExchange};
use crate::socket_worker::SocketWorker;

pub struct Hds {
    pub handle: JoinHandle<()>,
    pub messenger: MsgExchange,
}

impl Hds {
    pub fn new() -> Hds {
        let (messenger, away) = MsgExchange::make_pair();

        let handle = thread::spawn(move || {
            start_in_thread(away);
        });

        Hds { handle, messenger }
    }
}

fn start_in_thread(mx: MsgExchange) {
    let sock = UdpSocket::bind("127.0.0.1:8080").unwrap();

    let mut state = HashMap::new();

    loop {
        get_command(&mx, &mut state);

        expect_handshake(&sock)
    }
}

fn get_command(mx: &MsgExchange, state: &mut HashMap<String, String>) {
    match mx.rcv.recv_timeout(Duration::from_millis(10)) {
        Ok(Msg::Kvp(key, value)) => match key.trim() {
            "set" => {
                //println!("set {}", msg.value)

                let prm_split: Vec<&str> = value.split(':').collect();

                println!("{:?}",prm_split);

                state.insert(prm_split[0].to_string(), prm_split[1].to_string());
            }
            "get" => {
                //println!("get {}", msg.value)

                if state.contains_key(&value) {
                    mx.snd
                    .send(Msg::Kvp("Get response".to_string(), state[&value].clone()))
                    .unwrap();
                }
            }
            _ => {
                mx.snd
                    .send(Msg::Kvp("Unknown error".to_string(), String::new()))
                    .unwrap();
            }
        },
        Err(_err) => {}
    }
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

/*fn for_ref(sock: UdpSocket, mut state: HashMap<String, String>) {
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

            //mxs.push(soc_mx);

            thread::spawn(move || {
                use_thread(con, con_mx);
            });
        }
    }
}

fn use_thread(con: UdpSocket, mx: MsgExchange) {
    mx.snd
        .send(Msg::new("msg".to_string(), "hello".to_string()))
        .unwrap();
    let buf = [1];
    con.send(&buf).unwrap();
    let msg = mx.rcv.recv().unwrap();
    println!("{:?}", msg);
}*/
