# Nervo TCP Library

A simple Rust library for sending and receiving TCP messages.

[![Crates.io](https://crates.io/crates/nervo)

## Installation

Add to your `Cargo.toml`:

```
[dependencies]
nervo = "0.1.0"

```




Usage
```send(text: &str, ip_port: &str) – send a text message to a TCP server```

```rec(port: String)``` – run a blocking TCP server that reads incoming messages
Sending messages:
```
use nervo::send;

fn main() -> std::io::Result<()> {
    send("Hello", "127.0.0.1:8080")?;
    Ok(())
}
```
Receiving messages
```
use nervo::rec;

fn main() {
    rec(":8080".to_string());
}
```
Pass the port as ":PORT" (e.g., ":8080").

The function blocks and accepts incoming connections, reading data into a String.
