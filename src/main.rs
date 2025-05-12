use hds::Hds;
use msg_exchange::Msg;
use std::io;

mod hds;
mod msg_exchange;

fn main() {
    println!("Start!");

    let hds = Hds::new();

    control(hds);

    println!("End!");
}

fn control(hds: Hds) {
    loop {
        let mut cmd = String::new();

        io::stdin().read_line(&mut cmd).expect("Error read_line");

        //println!("Command: {}", cmd);

        match cmd.trim() {
            "set" => {
                println!("Write '[key]:[value]' to set");
                let mut param = String::new();
                io::stdin().read_line(&mut param).expect("Error read_line");
                let param = param.trim();

                hds.messenger
                    .snd
                    .send(Msg::new("set".to_string(), param.to_string()))
                    .unwrap();
            }
            "get" => {
                println!("Write '[key]' to get");
                let mut param = String::new();
                io::stdin().read_line(&mut param).expect("Error read_line");
                let param = param.trim();

                hds.messenger
                .snd
                .send(Msg::new("get".to_string(), param.to_string()))
                .unwrap();

                let msg = hds.messenger
                .rcv
                .recv().unwrap();
                println!("key '{param}' is '{}'", msg.value)
            }
            "exit" => {
                break;
            }
            _ => println!("Unknown command!"),
        }
    }
}
