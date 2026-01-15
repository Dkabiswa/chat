use std::io::{self, ErrorKind, Read, Write};
use std::net::TcpStream;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

const LOCAL: &str = "127.0.0.1:60000";
const MSG_SIZE: usize = 32;

fn sleep() {
    thread::sleep(Duration::from_millis(100));
}

fn main() {
    let mut client = TcpStream::connect(LOCAL).expect("Stream failed to connect");
    client
        .set_nonblocking(true)
        .expect("failed to initialize non-blocking");

    let (tx, _rx) = mpsc::channel::<String>();

    thread::spawn(move || {
        loop {
            let mut buff = vec![0; MSG_SIZE];

            match client.read_exact(&mut buff) {
                Ok(_) => {
                    let msg = buff.into_iter().take_while(|&x| x != 0).collect::<Vec<_>>();
                    let msg = String::from_utf8(msg).expect("Invalid utf8 message");

                    println!("message received {:?}", msg);
                }
                Err(ref err) if err.kind() == ErrorKind::WouldBlock => (),
                Err(_) => {
                    println!("connection with server was severed");
                    break;
                }
            }
            sleep()
        }
    });

    println!("Write a message:");
    loop {
        let mut buff = String::new();
        io::stdin()
            .read_line(&mut buff)
            .expect("reading from stdin failed");
        let msg = buff.trim().to_string();
        if msg == ":quit" || tx.send(msg).is_err() {
            break;
        }
    }

    println!("bye bye");
}
