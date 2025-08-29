use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;

// posílání zprávy
pub fn send(text: &str, ip_port: &str) -> std::io::Result<()> {
    let mut stream = TcpStream::connect(ip_port)?;
    stream.write_all(text.as_bytes())?;
    stream.write_all(b"\n")?; // oddělovač zpráv
    Ok(())
}

// přijímání zpráv realtime
pub fn rec(port: &str) -> Receiver<String> {
    let (tx, rx): (Sender<String>, Receiver<String>) = channel();
    let port = port.to_string();

    thread::spawn(move || {
        let listener =
            TcpListener::bind("0.0.0.0".to_string() + &port).expect("Failed to bind port");
        println!("Server listening on {}", port);

        for stream in listener.incoming() {
            let stream = stream.unwrap();
            let tx = tx.clone();

            thread::spawn(move || {
                let reader = BufReader::new(stream);
                for line in reader.lines() {
                    if let Ok(msg) = line {
                        tx.send(msg).unwrap();
                    }
                }
            });
        }
    });

    rx
}
