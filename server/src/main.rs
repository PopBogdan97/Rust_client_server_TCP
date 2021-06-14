use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};

fn handle_client(mut stream: TcpStream) -> std::io::Result<()> {
    println!("Connection from {}", stream.peer_addr()?);

    let mut buffer = [0; 170];
    let read_len = stream.read(&mut buffer)?;

    println!("Received: {:#?}\n Length: {}", buffer, read_len);

    stream.write(b"Hello Client!")?;

    Ok(())
}

fn main() -> std::io::Result<()> {
    println!("Listening...");
    let listener = TcpListener::bind("127.0.0.1:34254")?;

    // accept connections and process them serially
    for stream in listener.incoming() {
        handle_client(stream?).unwrap();
    }
    Ok(())
}
