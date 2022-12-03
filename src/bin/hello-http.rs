use std::{net::{TcpListener, TcpStream}, io::Read};
fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}
fn handle_connection(mut stream : TcpStream) {
    let mut buf = [0; 512];
    stream.read(&mut buf).unwrap();
    println!("Request : {}", String::from_utf8_lossy(&buf));
}
