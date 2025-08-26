use std::io::Read;
use std::io::Write;
use std::net::TcpListener;
use std::net::TcpStream;

pub fn send(text: &str, ip_port: &str) -> std::io::Result<()> {
    let mut stream = TcpStream::connect(ip_port)?;
    stream.write_all(text.as_bytes())?;
    Ok(())
}

pub fn rec(rec: String) {
    let listener = TcpListener::bind("0.0.0.0".to_owned() + &rec).unwrap();
    println!("{}", rec.clone());

    for stream in listener.incoming() {
        let mut stream = stream.unwrap();
        let mut buf = Vec::new();
        stream.read_to_end(&mut buf).unwrap();
        let text = String::from_utf8_lossy(&buf);
    }
}
