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

pub struct Msg {
    key: String,
    value: String
}
