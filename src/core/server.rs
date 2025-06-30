use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::{TcpListener, TcpStream},
};

use anyhow::Result;

pub async fn start_server() -> Result<()> {
    let listener = TcpListener::bind("127.0.0.1:7878").await?;

    loop {
        let (stream, _) = listener.accept().await?;

        tokio::spawn(handle_connection(stream));
    }
}

async fn handle_connection(mut stream: TcpStream) -> Result<()> {
    let (stream_reader, mut stream_writer) = stream.split();
    let mut stream_buf_reader = BufReader::new(stream_reader);

    let mut request_line = String::new();
    loop {
        request_line.clear();
        stream_buf_reader.read_line(&mut request_line).await?;
        stream_writer.write_all(request_line.as_bytes()).await?;
    }
}
