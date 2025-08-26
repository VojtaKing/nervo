# Nervo TCP Library

A simple Rust library for sending and receiving TCP messages.

(https://crates.io/crates/nervo)

## Installation

Add to your `Cargo.toml`:

```
[dependencies]
nervo = "0.2.0"
```
Usage
```Rust
use nervo::send;

fn main() -> std::io::Result<()> {
    send("Hello", "127.0.0.1:8080")?;
    Ok(())
}
```
Receiving messages
```Rust
use nervo::rec;

fn main() {
    // Start TCP server and get a receiver for incoming messages
    let rx = rec(":8080");

    // Receive one message
    if let Ok(msg) = rx.recv() {
        println!("Received: {}", msg);
    }
}
```
Pass the port as ":PORT" (e.g., ":8080").

The rec function runs the server in a separate thread and sends incoming messages via a channel.

This way, you can process received messages in your main thread.
