use std::io::prelude::*;
use std::net::{TcpStream, Shutdown};
use std::thread;
use std::time::{Duration, Instant};

fn main() -> std::io::Result<()> {
    let tcp_start = Instant::now();
    let mut stream = TcpStream::connect("127.0.0.1:34254")?;

    loop {
        let w = [65; 2048];
        stream.write(&w)?;
        //stream.flush();
        let mut buffer = [0; 20];
        let read_len = stream.read(&mut buffer)?;
        println!("Received: {:?}\n Length: {}", buffer, read_len);
        let tcp_time = tcp_start.elapsed();

        println!("Time elapsed: {:?}", tcp_time);

        //break;
        let millis = Duration::from_millis(2000);
        
	    stream.shutdown(Shutdown::Both);
        thread::sleep(millis);
    }

    Ok(())
} // the stream is closed here
