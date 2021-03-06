use std::net::{TcpListener, TcpStream};
use std::io::Read;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    stream.read(&mut buffer).unwrap();

    println!("请求信息【\r\nRequest: {} \r\n】", String::from_utf8_lossy(&buffer[..]));
}
