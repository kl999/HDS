use std::{
    collections::{HashMap, VecDeque}, net::UdpSocket, rc::Rc
};

use crate::message::Message;

pub struct SocketWorker
{
    socket: UdpSocket,
    outgoing: VecDeque<Message>,
    incoming: HashMap<u64, Rc<Message>>,
    notify: fn(Rc<Message>),
}

impl SocketWorker {
    pub fn new(socket: UdpSocket, f: fn(Rc<Message>)) -> SocketWorker {
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

    pub fn send_message(&mut self, msg: Message) {
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

                todo!("Check hash");

                let msg = Rc::new(msg);

                _ = self.incoming.insert(1, msg.clone());
                (self.notify)(msg);
            }
            Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                // No data is available right now
            }
            Err(e) => { panic!("On receive {}", e) }
        }
    }
    
    fn send(&mut self) {
        if let Some(msg) = self.outgoing.front() {
            print!("Sending '{}'", msg);
            self.socket.send(&msg.serialize()).unwrap();
            let msg = self.outgoing.pop_front().expect("wtf?");
            self.outgoing.push_back(msg);

            todo!("Delete received");
        }
    }
}
