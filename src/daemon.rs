const SOCKET: &'static str = "127.0.0.1:34254";
mod network;
mod query;
use std::{
    collections::HashMap,
    io::{Read, Write},
};
use tokio::{
    io,
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpListener,
    time::{timeout, Duration},
};

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
                if let Ok(connection) = timeout(Duration::from_millis(100), socket.accept()).await {
                    let (tcp_stream, _) = connection.unwrap();
                    request(tcp_stream, &mut command_hashmap).await?;
                }
                for value in command_hashmap.values_mut() {
                    value.update().await?;
                }
            }
        })
}
async fn init_hashmap() -> Result<HashMap<&'static str, Box<dyn BarQuery>>, tokio::io::Error> {
    use query::Command;
    //Command::new("date", vec!["%a.%m-%d-%Y--%I:%M.%p"], 0.3).await?,
    let mut command_hashmap: HashMap<&str, Box<dyn BarQuery>> = HashMap::new();
    command_hashmap.insert("time", Box::new(Command::new("date", vec![])));
    command_hashmap.insert(
        "volume",
        Box::new(Command::new("pulseaudio-ctl", vec!["current"])),
    );
    command_hashmap.insert(
        "net-down",
        Box::new(network::Network::new("/sys/class/net/enp6s0/statistics/rx_bytes").await?),
    );
    command_hashmap.insert(
        "net-up",
        Box::new(network::Network::new("/sys/class/net/enp6s0/statistics/tx_bytes").await?),
    );
    Ok(command_hashmap)
}
async fn request(
    mut connection: tokio::net::TcpStream,
    command_hashmap: &mut HashMap<&'static str, Box<dyn BarQuery>>,
) -> std::io::Result<()> {
    let mut message = String::new();
    connection.read_to_string(&mut message).await?;
    println!("requesting {}", message);
    let returned = match command_hashmap.remove_entry(message.as_str()) {
        Some((k, v)) => {
            let mut v = v;
            let s = v.result().await?;
            command_hashmap.insert(k, v);
            s
        }
        None => String::from("no value for request"),
    };
    connection.write(returned.as_bytes()).await?;
    Ok(())
}
#[async_trait::async_trait]
pub trait BarQuery {
    async fn result(&mut self) -> Result<String, io::Error>;
    async fn update(&mut self) -> Result<(), io::Error>;
}
