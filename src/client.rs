use std::io::{Read, Write};
const SOCKET: &'static str = "127.0.0.1:34254";
use std::net::{Shutdown, TcpStream};
fn main() -> std::io::Result<()> {
    let request = std::env::args()
        .skip(1)
        .next()
        .expect("supply and argument");
    let mut socket = TcpStream::connect(SOCKET)?;
    socket.write(request.as_bytes())?;
    socket.shutdown(Shutdown::Write)?;

    let mut result = String::new();
    socket.read_to_string(&mut result)?;
    println!("{}", result);
    Ok(())
}
