use std::{
    net::UdpSocket,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    thread,
};
use rand::Rng;

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

fn worker_thread(mut socket: UdpSocket, wrk_mx: MsgExchange, stop_flag: Arc<AtomicBool>) {
    let mut task_stack = vec![];

    task_stack.push(vec![1u8, 2, 3]);

    while !stop_flag.load(Ordering::Relaxed) {
        send_message(&mut task_stack, &mut socket);
    }
}

fn send_message(task_stack: &mut Vec<Vec<u8>>, socket: &mut UdpSocket) {
    if task_stack.len() == 0 { return; }
    let body = &mut task_stack[0];

    let buf = make_message(body);

    socket.send(&buf).expect("Worker thread failed to send message");

    task_stack.remove(0);
}

fn make_message(body: &mut Vec<u8>) -> Vec<u8> {
    let mut buf = vec![];
    let msg_id: u32 = rand::thread_rng().gen_range(1..=2_000_000);
    buf.copy_from_slice(&msg_id.to_be_bytes());
    buf.push(0);
    buf.append(body);

    buf
}
