use std::net::TcpListener;


fn main() {
    // 监听地址：127.0.0.1:7878
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    println!("Listening on IP_ADDRESS:7878");

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        println!("Connection from {}", stream.peer_addr().unwrap())
    }
}