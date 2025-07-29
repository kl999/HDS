use std::{
    collections::{HashMap, VecDeque},
    net::UdpSocket,
    rc::Rc,
};

use crate::message::Message;

pub struct SocketWorker {
    socket: UdpSocket,
    outgoing: VecDeque<Message>,
    incoming: HashMap<u64, Rc<Message>>,
    notify: fn(Rc<Message>),
    message_id: u64,
}

impl SocketWorker {
    pub fn new(socket: UdpSocket, f: fn(Rc<Message>)) -> SocketWorker {
        SocketWorker {
            socket,
            outgoing: VecDeque::with_capacity(1000),
            incoming: HashMap::new(),
            notify: f,
            message_id: 1u64,
        }
    }

    pub fn work(&mut self) {
        self.receive();
        self.send();
    }

    pub fn send_message(&mut self, msg: Box<[u8]>) {
        let msg = Message::new(self.message_id, msg);
        self.message_id += 1;
        self.outgoing.push_back(msg);
    }

    fn receive(&mut self) {
        let mut buf = [0; 1024];
        match &self.socket.recv_from(&mut buf) {
            Ok((number_of_bytes, src_addr)) => {
                let msg = Message::deserialize(&buf[..*number_of_bytes]);
                println!(
                    "Received {} bytes from {}: C({}) '{}'",
                    number_of_bytes,
                    src_addr,
                    msg.check_hash(),
                    msg
                );

                if !msg.check_hash() {
                    return;
                }

                if msg.id == 0 {
                    self.handle_ctrl(msg);
                    return;
                }

                let msg = Rc::new(msg);

                _ = self.incoming.insert(1, msg.clone());
                (self.notify)(msg);
            }
            Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                // No data is available right now
            }
            Err(e) => {
                panic!("On receive {}", e)
            }
        }
    }

    fn send(&mut self) {
        if let Some(msg) = self.outgoing.front() {
            print!("Sending '{}'", msg);
            self.socket.send(&msg.serialize()).unwrap();
            let msg = self.outgoing.pop_front().expect("wtf?");
            self.outgoing.push_back(msg);
        }
    }

    fn handle_ctrl(&mut self, msg: Message) {
        match msg.data[0] {
            1 => {
                let id = u64::from_be_bytes(msg.data[1..9].try_into().expect("Error casting id!"));

                let rem_ind = self
                    .outgoing
                    .iter()
                    .position(|i| i.id == id)
                    .expect("No outgoing to remove!");
                self.outgoing.remove(rem_ind);
            }
            _ => {
                panic!("Unknown message {}", msg.data[0]);
            }
        }
    }
}
