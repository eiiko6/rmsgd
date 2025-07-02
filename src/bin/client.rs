use anyhow::Result;
use chrono::{DateTime, Local, Utc};
use owo_colors::OwoColorize;
use rustyline::{DefaultEditor, ExternalPrinter};
use serde_json::from_str;
use std::io::{self, Write};
use std::sync::{Arc, Mutex};
use tokio::sync::mpsc;
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::TcpStream,
};

use rmsgd::{Message, User};

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
    let (reader, mut writer) = stream.into_split();

    let mut stream_buf_reader = BufReader::new(reader);

    let (line_tx, mut line_rx) = mpsc::channel::<String>(100);

    let rl = Arc::new(Mutex::new(DefaultEditor::new()?));
    let printer = Arc::new(Mutex::new(rl.lock().unwrap().create_external_printer()?));

    tokio::spawn(async move {
        let rl = Arc::clone(&rl);
        let line_tx = line_tx.clone();
        tokio::spawn(async move {
            loop {
                let readline = tokio::task::spawn_blocking({
                    let rl = Arc::clone(&rl);
                    move || {
                        let mut rl = rl.lock().unwrap();
                        rl.readline("➜ ")
                    }
                })
                .await;

                match readline {
                    Ok(Ok(line)) => {
                        if line.trim().is_empty() {
                            continue;
                        }
                        if line_tx.send(line).await.is_err() {
                            break;
                        }
                    }
                    _ => break,
                }
            }
        });
    });

    // async task to receive lines and send them over TCP
    tokio::spawn(async move {
        while let Some(line) = line_rx.recv().await {
            let msg = Message {
                user: User {
                    client_address: local_addr,
                    username: client_username.clone(),
                },
                content: line,
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

        let mut printer = printer.lock().unwrap();
        printer.print(format!(
            "[{}] {}: {}",
            DateTime::<Local>::from(message.time)
                .format("%Y-%m-%d %H:%M:%S")
                .dimmed(),
            message.user.username.blue(),
            message.content
        ))?;
    }

    Ok(())
}
