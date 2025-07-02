use anyhow::Result;
use chrono::Utc;
use owo_colors::OwoColorize;
use serde_json::from_str;
use std::io::{self, Write};
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader, stdin},
    net::TcpStream,
};

use rmsgd::Message;

fn get_username() -> Result<String> {
    // let mut path = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
    // path.push(".chat_username");

    // if path.exists() {
    //     Ok(fs::read_to_string(path)?.trim().to_string())
    // } else {
    //     print!("Enter a username: ");
    //     io::stdout().flush()?;
    //     let mut name = String::new();
    //     io::stdin().read_line(&mut name)?;
    //     let name = name.trim().to_string();
    //
    //     fs::write(path, &name)?;
    //     Ok(name)
    // }

    print!("Enter a username: ");
    io::stdout().flush()?;
    let mut name = String::new();
    io::stdin().read_line(&mut name)?;
    let name = name.trim().to_string();

    Ok(name)
}

#[tokio::main]
async fn main() -> Result<()> {
    let client_username = get_username()?;

    let stream = TcpStream::connect("127.0.0.1:7878").await?;
    let local_addr = stream.local_addr()?;
    let (reader, writer) = stream.into_split();

    let mut stream_buf_reader = BufReader::new(reader);

    tokio::spawn(async move {
        let mut client_input = String::new();
        let mut writer = writer;
        let mut stdin_reader = BufReader::new(stdin());

        loop {
            client_input.clear();

            // print!("➜ ");
            io::stdout().flush().unwrap();

            let bytes_read = stdin_reader.read_line(&mut client_input).await.unwrap_or(0);
            if bytes_read == 0 {
                break;
            }

            let trimmed = client_input.trim();
            if !trimmed.is_empty() {
                let msg = Message {
                    client_address: local_addr,
                    username: client_username.clone(),
                    content: trimmed.to_string(),
                    time: Utc::now(),
                };

                if let Ok(json_msg) = serde_json::to_string(&msg) {
                    if let Err(e) = writer.write_all(json_msg.as_bytes()).await {
                        eprintln!("Error sending to server: {e}");
                        break;
                    }
                    if let Err(e) = writer.write_all(b"\n").await {
                        eprintln!("Error sending newline to server: {e}");
                        break;
                    }
                }
            }
        }
    });

    // Read and display messages from server
    let mut server_output = String::new();
    loop {
        server_output.clear();
        let n = stream_buf_reader.read_line(&mut server_output).await?;
        if n == 0 {
            break; // server closed connection
        }

        let message: Message = from_str(&server_output)?;

        println!(
            "[{}] {}: {}",
            message.time.format("%Y-%m-%d %H:%M:%S").dimmed(),
            message.username.blue(),
            message.content
        );
    }

    Ok(())
}
