use anyhow::Result;
use serde_json::to_string;
use std::net::SocketAddr;
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::{TcpListener, TcpStream},
    sync::broadcast::{self, Receiver, Sender},
};

use rmsgd::Message;

#[tokio::main]
pub async fn main() -> Result<()> {
    let listener = TcpListener::bind("127.0.0.1:7878").await?;
    let (tx, _rx) = broadcast::channel::<Message>(32);

    loop {
        let (stream, client_socket_address) = listener.accept().await?;

        tokio::spawn(handle_connection(
            stream,
            client_socket_address,
            tx.clone(),
            tx.subscribe(),
        ));
    }
}

async fn handle_connection(
    mut stream: TcpStream,
    addr: SocketAddr,
    tx: Sender<Message>,
    mut rx: Receiver<Message>,
) -> Result<()> {
    let (stream_reader, mut stream_writer) = stream.split();
    let mut stream_buf_reader = BufReader::new(stream_reader);
    let mut client_input = String::new();

    loop {
        tokio::select! {
            result = stream_buf_reader.read_line(&mut client_input) => {
                if result? == 0 {
                    // Client disconnected, stop processing
                    return Ok(());
                }

                match serde_json::from_str::<Message>(client_input.trim()) {
                    Ok(msg) => {
                        // Broadcast to others
                        tx.send(msg)?;
                    }
                    Err(e) => {
                        eprintln!("Failed to parse message from {}: {}", addr, e);
                    }
                }
                client_input.clear();
            }

            Ok(msg) = rx.recv() => {
                if msg.client_address != addr {
                    let serialized = to_string(&msg)?;
                    stream_writer.write_all(serialized.as_bytes()).await?;
                    stream_writer.write_all("\n".as_bytes()).await?;
                }
            }
        }
    }
}
