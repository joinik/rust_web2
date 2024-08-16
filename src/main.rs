use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

fn main() {
    // 监听地址：127.0.0.1:7878
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    println!("Listening on IP_ADDRESS:7878");

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}

// 处理连接
fn handle_connection(mut stream: TcpStream) {
    // 读取客户端发送的数据
    let buffer = BufReader::new(&mut stream);
    let http_request: Vec<_> = buffer
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();
    // 处理数据
    let status_line = "HTTP/1.1 200 OK";
    let contents = fs::read_to_string("hello.html").unwrap();
    let length = contents.len();
    let response = format!("{status_line}\r\nContent-Length:{length}\r\n\r\n{contents}");
    // 发送响应
    stream.write_all(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
