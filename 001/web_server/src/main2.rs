use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::fs;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}


/// 编写请求的相应
fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    // 将请求信息读到 buffer 中
    stream.read(&mut buffer).unwrap();

    let contents = fs::read_to_string("index.html").unwrap();

    // 组装返回的内容。
    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
        contents.len(),
        contents
    );

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
