const SOCKET: &'static str = "127.0.0.1:34254";
mod bar;
use std::collections::HashMap;
use std::io::{Read, Write};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

use tokio::time::Duration;

fn main() -> std::io::Result<()> {
    tokio::runtime::Builder::new_current_thread()
        .enable_io()
        .enable_time()
        .build()?
        .block_on(async {
            let mut command_hashmap = init_hashmap().await?;
            println!("running on {}", SOCKET);
            let socket = TcpListener::bind(SOCKET).await?;
            loop {
                for command in command_hashmap.values_mut() {
                    command.update().await?;
                }
                if let Ok(connection) =
                    tokio::time::timeout(Duration::from_millis(100), socket.accept()).await
                {
                    let (tcp_stream, _) = connection.unwrap();
                    request(tcp_stream, &command_hashmap).await?;
                }
            }
        })
}
async fn init_hashmap() -> Result<HashMap<&'static str, bar::Command>, tokio::io::Error> {
    use bar::Command;
    let mut command_hashmap = HashMap::new();
    command_hashmap.insert(
        "time",
        //Command::new("date", vec!["%a.%m-%d-%Y--%I:%M.%p"], 0.3).await?,
        Command::new("date", vec![], 0.3).await?,
    );
    command_hashmap.insert(
        "volume",
        Command::new("pulseaudio-ctl", vec!["current"], 0.1).await?,
    );
    Ok(command_hashmap)
}
async fn request(
    mut connection: tokio::net::TcpStream,
    command_hashmap: &HashMap<&'static str, bar::Command>,
) -> std::io::Result<()> {
    let mut message = String::new();
    connection.read_to_string(&mut message).await?;
    println!("requesting {}", message);
    let returned = match command_hashmap.get(&message.as_str()) {
        Some(c) => {
            let last_result = c.last_result.clone();
            last_result.unwrap_or(" ".to_owned())
        }
        None => " ".to_owned(),
    };
    connection.write(returned.as_bytes()).await?;
    Ok(())
}
