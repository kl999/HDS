use std::{io, net::UdpSocket, thread};

fn main() {
    println!("Start!");

    thread::spawn(|| {
        let sock = UdpSocket::bind("127.0.0.1:8080").unwrap();

        loop {
            let mut buf = [0; 5];
            let (number_of_bytes, src_addr) =
                sock.recv_from(&mut buf).expect("Didn't receive data");
            let msg = String::from_utf8_lossy(&buf[..number_of_bytes]).to_string();
            println!("Received {} bytes from {}: '{}'", number_of_bytes, src_addr, msg);

            if msg == "Hello" {
                println!("handshaking");
                let con = UdpSocket::bind("127.0.0.1:0").unwrap();
                let port = con.local_addr().unwrap().port();
                let buf = format!("Connect port {}", port);
                sock.send_to(buf.as_bytes(), src_addr).unwrap();
                //echo "Hello" | nc -u 127.0.0.1 8080

            }
        }
    });

    //fs::read_to_string("config").unwrap();

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
