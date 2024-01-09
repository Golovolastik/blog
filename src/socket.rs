use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use super::thread_pool::ThreadPool;
use std::fs;
use std::io::prelude::*;
use std::thread;
use std::time::Duration;
use crate::db::PostgresUserRepository;
use crate::user::UserRepository;
use std::collections::HashMap;
use rand::{thread_rng, Rng};
use chrono;


// Структура для хранения сессий
#[derive(Clone, Debug)]
struct SessionManager {
    sessions: HashMap<String, String>, // Map<session_id, username>
}

impl SessionManager {
    fn new() -> Self {
        SessionManager {
            sessions: HashMap::new(),
        }
    }

    fn create_session(&mut self, username: &str) -> String {
        let session_id = generate_unique_session_id(); // Генерация уникального идентификатора сессии
        self.sessions.insert(session_id.clone(), username.to_string());
        session_id
    }

    fn get_username(&self, session_id: &str) -> Option<&String> {
        self.sessions.get(session_id)
    }
}
fn generate_unique_session_id() -> String {
    let mut rng = thread_rng();
    let timestamp = chrono::Utc::now().timestamp_nanos(); // Используем временную метку

    // Создаем уникальный идентификатор сессии путем комбинирования временной метки и случайного числа
    let session_id: String = format!("{}{}", timestamp, rng.gen::<u64>());
    session_id
}

fn read_file(filename: &str) -> std::io::Result<String> {
    fs::read_to_string(filename)
}
fn handle_options_request(stream: &mut TcpStream) -> std::io::Result<()> {
    let response = "HTTP/1.1 200 OK\r\n\
                    Access-Control-Allow-Origin: *\r\n\
                    Access-Control-Allow-Methods: POST\r\n\
                    Access-Control-Allow-Headers: content-type\r\n\
                    Content-Length: 0\r\n\r\n";
    stream.write_all(response.as_bytes())?;
    stream.flush()?;
    Ok(())
}
fn handle_get_request(stream: &mut TcpStream, buffer: &[u8]) -> std::io::Result<()> {
    let contents = std::fs::read_to_string("login.html")?;
    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
        contents.len(),
        contents
    );
    stream.write_all(response.as_bytes())?;
    stream.flush()?;
    Ok(())
}
fn handle_post_login_request(stream: &mut TcpStream, buffer: &[u8], db: &mut PostgresUserRepository, session_manager: &mut SessionManager) -> std::io::Result<crate::user::UserLogin> {
    let request = String::from_utf8_lossy(&buffer[..]);
    let body_start = request.find("\r\n\r\n").unwrap_or(0) + 4;
    let body = &request[body_start..].trim_end_matches('\0');
    let user = serde_json::from_str::<crate::user::UserLogin>(&body);
    match user {
        Err(ref err) => eprintln!("{:?}", err),
        Ok(ref usr) => println!("{:?}", usr),
    }
    match db.check_user_availability(&user.as_ref().unwrap().username) {
        Ok(true) => {
            println!("User doesn't register");
            let contents = "User doesn't register";
            let response = format!(
                "HTTP/1.1 401 Unauthorized\r\nContent-Length: {}\r\n\r\n{}",
                contents.len(),
                contents
            );
            stream.write_all(response.as_bytes()).unwrap();
        },
        Ok(false) => {
            println!("User is already registered");
            match db.check_pass(&user.as_ref().unwrap().username, &user.as_ref().unwrap().password) {
                true => {
                    println!("Password is correct");
                    let mut content = "session_id=".to_string();
                    let session_id = session_manager.create_session(&user.as_ref().unwrap().username);
                    content.push_str(&session_id);
                    content.push_str(";");
                    let response = format!(
                        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
                        content.len(),
                        content
                    );
                    println!("{}", response);
                    stream.write_all(response.as_bytes()).unwrap();
                },
                false => {
                    println!("Password is incorrect");
                    let contents = "incorrect password";
                    let response = format!(
                        "HTTP/1.1 401 Unauthorized\r\nContent-Length: {}\r\n\r\n{}",
                        contents.len(),
                        contents
                    );
                    stream.write_all(response.as_bytes()).unwrap();
                }
            }
        },
        Err(_) => println!("Undefined behavior"),
    }
    Ok(user.unwrap())
}
fn handle_image_request(stream: &mut TcpStream) -> std::io::Result<()> {
    let image_file_path = "images/logo.jpg";
    if let Ok(image_data) = std::fs::read(image_file_path) {
        let response = format!(
            "HTTP/1.1 200 OK\r\nContent-Type: image/jpeg\r\nContent-Length: {}\r\n\r\n",
            image_data.len(),
        );
        stream.write_all(response.as_bytes())?;
        stream.write_all(&image_data)?;
        stream.flush()?;
    } else {
        let contents = std::fs::read_to_string("404.html")?;
        let response = format!(
            "HTTP/1.1 404 NOT FOUND\r\nContent-Length: {}\r\n\r\n{}",
            contents.len(),
            contents
        );
        stream.write_all(response.as_bytes())?;
        stream.flush()?;
    }
    Ok(())
}
fn handle_sleep(stream: &mut TcpStream) -> std::io::Result<()> {
    thread::sleep(Duration::from_secs(5));
    let contents = fs::read_to_string("../hello.txt").unwrap();
    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
        contents.len(),
        contents
    );
    stream.write_all(response.as_bytes())?;
    stream.flush()?;
    Ok(())
}
fn handle_registration(stream: &mut TcpStream) -> std::io::Result<()> {
    let contents = fs::read_to_string("registration.html").unwrap();
    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
        contents.len(),
        contents
    );
    stream.write_all(response.as_bytes())?;
    stream.flush()?;
    Ok(())
}
fn extract_username(request: &[u8]) -> Option<String> {
    let request_str = String::from_utf8_lossy(request);
    let home_str = "GET /home";
    if let Some(start_idx) = request_str.find(home_str) {
        println!("inside the loop");
        let start = start_idx + home_str.len(); // Находим начало имени пользователя после "/home="
        let end = request_str[start_idx + home_str.len()..].find(' ')
            .map(|i| i + start_idx + home_str.len())
            .unwrap_or(request_str.len()); // Находим конец имени пользователя
        let username = &request_str[start..end];
        return Some(username.to_string());
    }
    None
}
fn handle_session(stream: &mut TcpStream, session_id: String, db: &mut PostgresUserRepository) -> std::io::Result<()> {
    let contents = fs::read_to_string("registration.html").unwrap();
    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
        contents.len(),
        contents
    );
    stream.write_all(response.as_bytes())?;
    stream.flush()?;
    Ok(())
}

// Функция для обработки страницы конкретного пользователя
fn handle_user_page(stream: &mut TcpStream, username: Option<String>, db: &mut PostgresUserRepository) -> std::io::Result<()> {
    if let Some(username) = username {
        // Здесь можно сгенерировать страницу для данного пользователя
        let contents = format!("Hello, {}!", username);
        //let contents = crate::content::generate(&mut db, )
        let response = format!(
            "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
            contents.len(),
            contents
        );
        stream.write_all(response.as_bytes())?;
    } else {
        let contents = "Invalid username";
        let response = format!(
            "HTTP/1.1 400 BAD REQUEST\r\nContent-Length: {}\r\n\r\n{}",
            contents.len(),
            contents
        );
        stream.write_all(response.as_bytes())?;
    }

    stream.flush()?;
    Ok(())
}
fn handle_home_request(stream: &mut TcpStream, username: &str, session_manager: &mut SessionManager) -> std::io::Result<()> {
    if session_manager.sessions.contains_key(username) {
        let contents = format!("Hello, {}!", username);
        let response = format!(
            "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
            contents.len(),
            contents
        );
        stream.write_all(response.as_bytes())?;
    } else {
        let contents = "Unauthorized access";
        let response = format!(
            "HTTP/1.1 401 Unauthorized\r\nContent-Length: {}\r\n\r\n{}",
            contents.len(),
            contents
        );
        stream.write_all(response.as_bytes())?;
    }

    stream.flush()?;
    Ok(())
}
fn handle_connection(mut stream: TcpStream, mut session_manager: SessionManager) {
    let mut db = crate::db::connect().unwrap();
    let mut buffer = [0; 2048];
    stream.read(&mut buffer).unwrap();

    match buffer {
        b if b.starts_with(b"OPTIONS / HTTP/1.1\r\n") => {
            handle_options_request(&mut stream).unwrap();
        }
        b if b.starts_with(b"GET / HTTP/1.1\r\n") => {
            handle_get_request(&mut stream, &buffer).unwrap();
        }
        b if b.starts_with(b"GET /register HTTP/1.1\r\n") => {
            handle_registration(&mut stream).unwrap();
        }
        b if b.starts_with(b"GET /sleep HTTP/1.1\r\n") => {
            handle_sleep(&mut stream).unwrap();
        }
        b if b.starts_with(b"POST / HTTP/1.1\r\n") => {
            handle_post_login_request(&mut stream, &buffer, &mut db, &mut session_manager).unwrap();
        }
        b if b.starts_with(b"GET /images/") => {
            handle_image_request(&mut stream).unwrap();
        }
        b if b.starts_with(b"GET /home") => {
            if let Some(username) = extract_username(&buffer) {
                handle_home_request(&mut stream, &username, &mut session_manager).unwrap();
            }
        }
        b if b.starts_with(b"GET /session=") => {
            // Извлечение идентификатора сессии из запроса
            let session_start = b.iter().position(|&x| x == b'=').unwrap_or(0) + 1;
            let session_end = b.iter().position(|&x| x == b' ').unwrap_or(buffer.len());
            let session_id = std::str::from_utf8(&buffer[session_start..session_end]).unwrap();

            handle_session(&mut stream, session_id.to_string(), &mut db).unwrap();
        }
        _ => {
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
}
pub fn listen() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);
    let mut session_manager = SessionManager::new();

    for stream in listener.incoming().take(20) {
        let stream = stream.unwrap();

        let session_manager_clone = session_manager.clone(); // Клонируем для передачи в замыкание

        pool.execute(|| {
            handle_connection(stream, session_manager_clone);
        });
    }

    println!("Shutting down.");
}
