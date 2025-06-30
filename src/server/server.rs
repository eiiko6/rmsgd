use std::net::SocketAddr;

use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::{TcpListener, TcpStream},
    sync::broadcast::{self, Receiver, Sender},
};

use anyhow::Result;

pub async fn start_server() -> Result<()> {
    let listener = TcpListener::bind("127.0.0.1:7878").await?;
    let (tx, rx) = broadcast::channel::<(String, SocketAddr)>(32);

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
    tx: Sender<(String, SocketAddr)>,
    mut rx: Receiver<(String, SocketAddr)>,
) -> Result<()> {
    let (stream_reader, mut stream_writer) = stream.split();
    let mut stream_buf_reader = BufReader::new(stream_reader);
    let mut client_input = String::new();

    loop {
        tokio::select! {
            _ = stream_buf_reader.read_line(&mut client_input) => {
                tx.send((client_input.clone(), addr))?;
                client_input.clear();
            }

            Ok((message, client)) = rx.recv() => {
                if client != addr {
                    let message = format!("{client} - {message}");
                    stream_writer.write_all(message.as_bytes()).await?;
                }
            }
        }
    }
}
