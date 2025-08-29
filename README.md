Nervo TCP Library

A simple Rust library for sending and receiving TCP messages in a separate thread. Suitable for realtime applications like games.

Crates.io

Installation

Add to your Cargo.toml:
```toml
[dependencies]
nervo = "0.2.0"
```
Usage
Sending messages
```Rust
use nervo::send;

fn main() -> std::io::Result<()> {
    send("Hello", "127.0.0.1:8080")?;
    Ok(())
}
```

The send function connects to the specified TCP address and sends a single message. Each message should end with a newline (\n) for proper reception.

Receiving messages
use nervo::rec;
```Rust
fn main() {
    // Start TCP server and get a receiver for incoming messages
    let rx = rec(":8080");

    // Receive messages in real-time without blocking the main thread
    loop {
        if let Ok(msg) = rx.try_recv() {
            println!("Received: {}", msg);
        }

        // Other logic (game loop, GUI updates, etc.) can run here
    }
}
```

Pass the port as ":PORT" (e.g., ":8080").

The rec function runs the server in a separate thread and sends incoming messages via an mpsc channel.

For realtime applications, use try_recv() to avoid blocking your main thread.

Notes

Each TCP connection spawns a new thread, so multiple clients can connect simultaneously.

Make sure messages are newline-delimited (\n) for correct reception.

Suitable for realtime games, GUI apps, or simple TCP messaging systems.
