use std::{io, thread};

mod msg_exchange;
mod hds;

fn main() {
    println!("Start!");

    hds::start();

    let ctrl_hndl = thread::spawn(|| {
        control();
    });

    ctrl_hndl.join().unwrap();

    println!("End!");
}

fn control() {
    loop {
        let mut cmd = String::new();

        io::stdin().read_line(&mut cmd).expect("Error read_line");

        println!("Command: {}", cmd);

        match cmd.trim() {
            "exit" => {
                break;
            }
            _ => println!("Unknown command!"),
        }
    }
}
