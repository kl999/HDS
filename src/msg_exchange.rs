use std::sync::mpsc::{Receiver, Sender};

pub struct MsgExchange {
    pub snd: Sender<Msg>,
    pub rcv: Receiver<Msg>
}

impl MsgExchange {
    pub fn new(snd: Sender<Msg>, rcv: Receiver<Msg>) -> MsgExchange {
        MsgExchange {
            snd,
            rcv
        }
    }
}

#[derive(Debug)]
pub struct Msg {
    key: String,
    value: String
}

impl Msg {
    pub fn new(key: String, value: String) -> Msg {
        Msg{
            key,
            value
        }
    }
}
