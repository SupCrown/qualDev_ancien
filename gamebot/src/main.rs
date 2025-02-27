mod bot;

use std::io::Write;
use std::net::TcpStream;

fn main() {

    if let Ok(mut stream) = TcpStream::connect("172.17.0.1:6969") {
        println!("Connected to the server!");
        stream.write(b"GunTrig=1.0");
    } else {
        println!("Couldn't connect to server...");
    }
}