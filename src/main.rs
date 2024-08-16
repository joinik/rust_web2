use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    
};
use rust_web::ThreadPool;

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
    let request_line: String = buffer.lines().next().unwrap().unwrap();

    // 处理数据
    let (status_line, filename) = if request_line == "GET / HTTP/1.1" {
        // 处理 GET 请求
        ("HTTP/1.1 200 OK", "hello.html")
    } else {
        // 处理404 请求
        ("HTTP/1.1 200 OK", "404.html")
    };
    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
    // 将所有已写入的数据刷新到目标设备或存储介质中，确保数据被完整地传输或保存
    stream.flush().unwrap();
}
