use std::sync::mpsc::{channel, Receiver, Sender};

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

    pub fn make_pair() -> (MsgExchange, MsgExchange) {
        let (a_rx, b_tx) = channel();
        let (b_rx, a_tx) = channel();

        let a = MsgExchange::new(a_rx, a_tx);
        let b = MsgExchange::new(b_rx, b_tx);

        (a, b)
    }
}

#[derive(Debug)]
pub struct Msg {
    pub key: String,
    pub value: String
}

impl Msg {
    pub fn new(key: String, value: String) -> Msg {
        Msg{
            key,
            value
        }
    }
}

/*pub enum Command {
    Set(Msg),
    Get(String)
}*/
