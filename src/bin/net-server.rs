use std::{
    io::{Error, Read},
    net::{Ipv4Addr, SocketAddrV4, TcpListener},
};

// 监听未使用 TCP/IP 端口
fn main() -> Result<(), Error> {
    let loopback = Ipv4Addr::new(127, 0, 0, 1); // 本地回环地址
    let socket = SocketAddrV4::new(loopback, 12345); // 端口设置为0，自动分配一个随机端口
    let listener = TcpListener::bind(socket)?;

    let port = listener.local_addr()?.port(); // 获取分配的端口
    println!("Listening on port: {}", port);

    let (mut tcp_stream, addr) = listener.accept()?; // 接受连接
    println!("Accepted connection from: {}", addr);

    let mut input = String::new();
    tcp_stream.read_to_string(&mut input)?; // 读取数据
    println!("Received data from {:?}: {}", addr, input);

    Ok(())
}

// 打开一个终端运行服务：cargo run --bin net-server
// 打开另一个终端执行测试：cargo test --bin net-server

#[cfg(test)]
mod test {
    use std::io::{Error, Write};
    use std::net::TcpStream;
    #[test]
    fn test_tcp_server() -> Result<(), Error> {
        // 测试 TCP 服务器
        let port = 12345; // 替换为服务器显示的端口号
        let mut stream = TcpStream::connect(format!("127.0.0.1:{}", port))?;

        stream.write_all(b"Hello from Rust client!")?;
        println!("Message sent to server");

        Ok(())
    }
}
