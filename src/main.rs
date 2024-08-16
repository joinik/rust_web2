use std::{
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
    println!("Request: {:#?}", http_request);
}
