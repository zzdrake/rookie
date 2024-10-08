use std::io::{self, prelude::*, BufReader, Write};
use std::net::TcpStream;
use std::str;

fn main() -> io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:8080")?;
    for _ in 0..10 {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read");
        stream.write(input.as_bytes()).expect("Failed to write");

        let mut reader = BufReader::new(&stream);
        let mut buf: Vec<u8> = Vec::new();
        reader.read_until(b'\n', &mut buf).expect("Failed to read into buf");
        println!("read from server: {}", str::from_utf8(&buf).unwrap());
        println!("\n");
    }
    Ok(())
}
