# Rust-Chat-Server

This is a Rust Chat Server written asynchronously using tokio module. The server allows multiple connections at the same time and includes broadcasting to emit messages on the server.

To run the applicaiton:

1- Clone the repo
2- Run the applicaiton using "cargo run"
3- Connect a telnet client to the localhost (Port: 8080) and chat on the server using multiple clients.

The application is a simple helper tutorial to unserstand asynchronous routines in Rust using Tokio, as well as learn spawning for enabling multiple users connect to a TCP server. Running Asynchronous processes concurrently using tokio::select can help with maintaining state correctly and enable a better experience.

