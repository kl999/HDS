use std::{
    net::UdpSocket,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    thread,
};

use crate::msg_exchange::MsgExchange;

pub struct SocketWorker {
    pub main_mx: MsgExchange,
    worker_thread_handler: thread::JoinHandle<()>,
    stop_flag: Arc<AtomicBool>,
}

impl SocketWorker {
    pub fn new(sock: UdpSocket) -> SocketWorker {
        let (main_mx, wrk_mx) = MsgExchange::make_pair();

        let stop_flag = Arc::new(AtomicBool::new(false));
        let stop_flag_copy = Arc::clone(&stop_flag);

        let worker_thread_handler = thread::spawn(move || worker_thread(sock, wrk_mx, stop_flag_copy));

        SocketWorker {
            main_mx,
            worker_thread_handler,
            stop_flag,
        }
    }

    pub fn join(self) {
        self.stop_flag.store(true, Ordering::Relaxed);
        self.worker_thread_handler.join().expect("join failed");
    }
}

fn worker_thread(socket: UdpSocket, wrk_mx: MsgExchange, stop_flag: Arc<AtomicBool>) {
    let buf = [1];
    socket.send(&buf).unwrap();

    while !stop_flag.load(Ordering::Relaxed) {
        todo!()
    }
}
