use std::io::prelude::*;
use std::net::TcpStream;
use std::{thread, time};

fn main() -> std::io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:34254")?;

    loop {
        stream.write(b"Hello Server!")?;
        let mut buffer = [0; 20];
        let read_len = stream.read(&mut buffer)?;
        println!("Received: {:#?}\n Length: {}", buffer, read_len);

        let millis = time::Duration::from_millis(1000);

        thread::sleep(millis);
    }

    Ok(())
} // the stream is closed here
