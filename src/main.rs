use std::fs;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

fn main() {
    // ربط الخادم بـ المنفذ 7878 محلياً
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    println!("🚀 HTTP Server running on http://127.0.0.1:7878");

    // حلقة استقبال الاتصالات المستمرة
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                // استخدام خيط منفصل (Thread) لكل اتصال لضمان عدم تعطل الخادم
                thread::spawn(|| {
                    handle_connection(stream);
                });
            }
            Err(e) => {
                eprintln!("Failed to establish connection: {}", e);
            }
        }
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    // استجابة HTTP احترافية وسريعة
    let status_line = "HTTP/1.1 200 OK";
    let contents = "<html><body><h1>Hello from Custom Rust HTTP Server!</h1><p>High performance systems programming.</p></body></html>";
    let length = contents.len();

    let response = format!(
        "{}\r\nContent-Length: {}\r\nContent-Type: text/html; charset=UTF-8\r\n\r\n{}",
        status_line, length, contents
    );

    stream.write_all(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
