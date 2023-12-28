use super::thread_pool::ThreadPool;
use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::path::PathBuf;
use std::thread;
use std::time::Duration;
use serde_json::Value;

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 2048];
    stream.read(&mut buffer).unwrap();

    let request = String::from_utf8_lossy(&buffer[..]);
    //println!("{}", request);

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";
    let registration = b"GET /register HTTP/1.1\r\n";
    let post = b"POST / HTTP/1.1\r\n";
    let options = b"OPTIONS / HTTP/1.1\r\n";
    let image_path = b"GET /images/";

    if buffer.starts_with(options) {
        let response = "HTTP/1.1 200 OK\r\n\
                        Access-Control-Allow-Origin: *\r\n\
                        Access-Control-Allow-Methods: POST\r\n\
                        Access-Control-Allow-Headers: content-type\r\n\
                        Content-Length: 0\r\n\r\n";
        stream.write_all(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    } else if buffer.starts_with(get) {
        let contents = fs::read_to_string("login.html").unwrap();
        let response = format!(
            "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
            contents.len(),
            contents
        );
        stream.write_all(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    } else if buffer.starts_with(registration) {
        let contents = crate::content::generate();
        //let contents = fs::read_to_string("registration.html").unwrap();
        let response = format!(
            "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
            contents.len(),
            contents
        );
        stream.write_all(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    } else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(5));
        let contents = fs::read_to_string("hello.html").unwrap();
        let response = format!(
            "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
            contents.len(),
            contents
        );
        stream.write_all(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    } else if buffer.starts_with(post) {
        let request = String::from_utf8_lossy(&buffer[..]);
        let body_start = request.find("\r\n\r\n").unwrap_or(0) + 4;
        let body = &request[body_start..];

        println!("Получены данные из формы: {}", body);
        let contents = fs::read_to_string("hello.html").unwrap();
        let response = format!(
            "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
            contents.len(),
            contents
        );
        //let redirect_response = "HTTP/1.1 303 See Other\r\nLocation: http://localhost:7878\r\n\r\n";

        //let response = "HTTP/1.1 200 OK\r\n\r\nData received";
        stream.write_all(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }else if buffer.starts_with(image_path) {
        // Путь к изображению на сервере (здесь используйте реальный путь к вашему изображению)
        let image_file_path = "images/logo.jpg";

        // Попытка чтения содержимого изображения
        if let Ok(image_data) = fs::read(image_file_path) {
            let response = format!(
                "HTTP/1.1 200 OK\r\nContent-Type: image/jpeg\r\nContent-Length: {}\r\n\r\n",
                image_data.len(),
            );

            // Отправка заголовка и содержимого изображения
            stream.write_all(response.as_bytes()).unwrap();
            stream.write_all(&image_data).unwrap();
            stream.flush().unwrap();
        } else {
            // Если изображение не найдено, отправляем статус ошибки 404
            let contents = fs::read_to_string("404.html").unwrap();
            let response = format!(
                "HTTP/1.1 404 NOT FOUND\r\nContent-Length: {}\r\n\r\n{}",
                contents.len(),
                contents
            );
            stream.write_all(response.as_bytes()).unwrap();
            stream.flush().unwrap();
        }
    } else {
        let contents = fs::read_to_string("404.html").unwrap();
        let response = format!(
            "HTTP/1.1 404 NOT FOUND\r\nContent-Length: {}\r\n\r\n{}",
            contents.len(),
            contents
        );
        stream.write_all(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }
}
pub fn listen() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming().take(20) {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }

    println!("Shutting down.");
}
