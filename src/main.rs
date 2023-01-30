use tokio::io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
use tokio::io::BufReader;
use tokio::sync::broadcast;

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("localhost:8080").await.unwrap();
    let (tx, mut rx) = broadcast::channel(10);
    loop {
        let (mut socket, addr) = listener.accept().await.unwrap();

        let tx = tx.clone();
        let mut rx = tx.subscribe();

        tokio::spawn(async move{
            let (reader, mut writer) = socket.split();
        let mut reader = BufReader::new(reader);
        let mut line = String::new();

        loop {
            tokio::select! {
                result = reader.read_line(&mut line) => {
                    if result.unwrap() == 0 {
                        break;
                      }
                    tx.send((line.clone(), addr)).unwrap();
                    line.clear();
                }
                result = rx.recv() => {
                    let (msg, o_addr) = result.unwrap();
                    if addr != o_addr{
                        writer.write_all(msg.as_bytes()).await.unwrap();
                    }

                }
            }
        }
        });

    }
}
