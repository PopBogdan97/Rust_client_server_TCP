use std::io::prelude::*;
use std::net::TcpStream;

fn main() -> std::io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:34254")?;

    stream.write(b"Hello Server!")?;
    let mut buffer = [0; 20];
    let read_len = stream.read(&mut buffer)?;

    println!("Received: {:#?}\n Length: {}", buffer, read_len);

    Ok(())
} // the stream is closed here
