use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream}
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
    let request_line: String = buffer.lines().next().unwrap().unwrap();

    // 处理数据
    if request_line == "GET / HTTP/1.1" {
        // 处理 GET 请求
        let status_line = "HTTP/1.1 200 OK";
        let contents = fs::read_to_string("hello.html").unwrap();
        let length = contents.len();
        let response = format!("{status_line}\r\nContent-Length:{length}\r\n\r\n{contents}");
        // 发送响应
        stream.write_all(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    } else {
        // 处理其他请求
        // some other request
        let status_line = "HTTP/1.1 404 NOT FOUND";
        let contents = fs::read_to_string("404.html").unwrap();
        let length = contents.len();

        let response = format!(
            "{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}"
        );

        stream.write_all(response.as_bytes()).unwrap();
    }
}
