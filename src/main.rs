use std::{io, thread};

mod hds;
mod msg_exchange;

fn main() {
    println!("Start!");

    hds::start();

    control();

    println!("End!");
}

fn control() {
    loop {
        let mut cmd = String::new();

        io::stdin().read_line(&mut cmd).expect("Error read_line");

        println!("Command: {}", cmd);

        match cmd.trim() {
            "set" => {
                let mut param = String::new();
                io::stdin().read_line(&mut param).expect("Error read_line");

                let params_split = param.split(":");

                println!("Command set. Params: {:?}", params_split);
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
