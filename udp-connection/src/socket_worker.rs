use std::{
    collections::{HashMap, VecDeque}, net::UdpSocket
};

pub struct SocketWorker
{
    socket: UdpSocket,
    outgoing: VecDeque<String>,
    incoming: HashMap<u64, String>,
    notify: fn(String),
}

impl SocketWorker {
    pub fn new(socket: UdpSocket, f: fn(String)) -> SocketWorker {
        SocketWorker {
            socket,
            outgoing: VecDeque::with_capacity(1000),
            incoming: HashMap::new(),
            notify: f,
        }
    }

    pub fn work(&mut self) {
        self.receive();
        self.send();
    }

    pub fn send_message(&mut self, msg: String) {
        self.outgoing.push_back(msg);
    }

    fn receive(&mut self) {
        let mut buf = [0; 1024];
        let (number_of_bytes, src_addr) = &self.socket.recv_from(&mut buf).expect("Didn't receive data");
        let msg = String::from_utf8_lossy(&buf[..*number_of_bytes]).to_string();
        println!(
            "Received {} bytes from {}: '{}'",
            number_of_bytes, src_addr, msg
        );

        self.incoming.insert(1, msg.clone()).unwrap();
        (self.notify)(msg);
    }
    
    fn send(&mut self) {
        if let Some(msg) = self.outgoing.front() {
            print!("Sending '{}'", msg);
            let buf = format!("{}", msg);
            self.socket.send_to(buf.as_bytes(), "127.0.0.1:8080").unwrap();
            let _ = self.outgoing.pop_front().expect("wtf?");
        }
    }
}
