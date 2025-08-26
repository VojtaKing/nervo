// src/lib.rs
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;

pub fn send(text: &str, ip_port: &str) -> std::io::Result<()> {
    let mut stream = TcpStream::connect(ip_port)?;
    stream.write_all(text.as_bytes())?;
    Ok(())
}

pub fn rec(port: &str) -> Receiver<String> {
    let (tx, rx): (Sender<String>, Receiver<String>) = channel();
    let port = port.to_string();

    thread::spawn(move || {
        let listener =
            TcpListener::bind("0.0.0.0".to_string() + &port).expect("Failed to bind port");
        println!("Server listening on {}", port);

        for stream in listener.incoming() {
            let mut stream = stream.unwrap();
            let mut buf = Vec::new();
            stream.read_to_end(&mut buf).unwrap();
            let text = String::from_utf8_lossy(&buf).to_string();
            tx.send(text).unwrap();
        }
    });

    rx
}
