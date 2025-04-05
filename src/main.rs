use std::{io, net::UdpSocket, thread};

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
            "send" => {
                /*let buf = "Hello UDP!".as_bytes();
                sock.send_to(buf, "127.0.0.1:8080").unwrap();*/
            }
            "receive" => {
                /*let mut buf = [0; 1024];
                let (number_of_bytes, src_addr) = sock.recv_from(&mut buf)
                                        .expect("Didn't receive data");
                println!(
                    "Received {} bytes from {}: {}",
                    number_of_bytes,
                    src_addr,
                    String::from_utf8_lossy(&buf[..number_of_bytes])
                );*/
            }
            "exit" => {
                break;
            }
            _ => println!("Unknown command!"),
        }
    }
}
