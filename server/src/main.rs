use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::time::{Duration, Instant};

fn handle_client(mut stream: TcpStream, tcp_start: Instant) -> std::io::Result<()> {
    println!("Connection from {}", stream.peer_addr()?);

    loop {

        let mut buffer = [0; 2048];
        let read_len = stream.read(&mut buffer)?;
        
        println!("Received: {:?}\n Length: {}", buffer, read_len);
        
        stream.write(b"Hello Client!")?;

        let tcp_time = tcp_start.elapsed();

        println!("Time elapsed: {:?}", tcp_time);
    }

    Ok(())
}

fn main() -> std::io::Result<()> {
    println!("Listening...");
    let listener = TcpListener::bind("127.0.0.1:34254")?;

    // accept connections and process them serially
    for stream in listener.incoming() {
        let tcp_start = Instant::now();

        handle_client(stream?, tcp_start).unwrap();
    }
    Ok(())
}
