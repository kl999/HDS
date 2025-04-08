use std::io;
use msg_exchange::{Msg, MsgExchange};

mod hds;
mod msg_exchange;

fn main() {
    println!("Start!");

    let (worker_mx, control_mx) = MsgExchange::make_pair();

    hds::start(worker_mx);

    control(control_mx);

    println!("End!");
}

fn control(mx: MsgExchange) {
    loop {
        let mut cmd = String::new();

        io::stdin().read_line(&mut cmd).expect("Error read_line");

        //println!("Command: {}", cmd);

        match cmd.trim() {
            "set" => {
                let mut param = String::new();
                io::stdin().read_line(&mut param).expect("Error read_line");

                let mut params_split = param.split(":");

                println!("Command set. Params: {:?}", params_split);

                mx.snd.send(Msg::new(params_split.next().unwrap().to_string(), params_split.next().unwrap().to_string())).unwrap();
            }
            "get" => {
                let mut param = String::new();
                io::stdin().read_line(&mut param).expect("Error read_line");
                println!("Get {}", param)
            }
            "exit" => {
                break;
            }
            _ => println!("Unknown command!"),
        }
    }
}
