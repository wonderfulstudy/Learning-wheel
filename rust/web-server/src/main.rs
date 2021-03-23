use std::io::prelude::*; // 引入用到的库文件
use std::net::TcpStream;
use std::net::TcpListener;

fn main() { // 程序入口
    let listener = TcpListener::bind("0.0.0.0:8080").unwrap(); // 定义tcp监听实例，并于本地8080端口绑定

    for stream in listener.incoming() { // 循环监听端口是否有数据输入
        match stream { // 模式匹配检测输入数据是否有错误发生
            Ok(stream) => { // 输入正确
                handle_connection(stream); // 将输入的数据交由子函数打印以及echo处理
            }
            Err(e) => panic!("encountered IO error: {}", e), // 发生错误，打印错误日志
        }
    }
}

fn handle_connection(mut stream: TcpStream) { // echo处理函数
    let mut buffer = [0; 1024]; // 数据接收缓冲
    let response = "OK!\n"; // 成功有返回远程的字符

    stream.read(&mut buffer).unwrap(); // 冲输入流中读取字符
    println!("Request: {}", String::from_utf8_lossy(&buffer[..])); // 打印读取到的字符
    stream.write(response.as_bytes()).unwrap(); // 将返回远程的字符，写入流
    stream.flush().unwrap(); // 发送远程数据
}
