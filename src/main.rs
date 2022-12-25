use std::net::TcpStream;
use std::str;
use std::io::{self, BufRead,BufReader,Write};

fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:8888")
    .expect("Could not connect to server");
}
