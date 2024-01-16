use postgres;
use crate::Error;
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct User {
    pub id: i16,
    pub login: String,
    pub password_hash: String,
}
impl User {
    pub fn new() -> Self {
        User {
            id: 0,
            login: "".to_string(),
            password_hash: "".to_string(),
        }
    }
}
#[derive(Serialize, Deserialize, Debug)]
pub struct UserLogin {
    pub username: String,
    pub password: String,
}

// Интерфейс для работы с пользователями в базе данных
pub trait UserRepository {
    fn check_user_availability(&mut self, name: &str) -> Result<bool, Error::MyError>;
    fn check_pass(&mut self, name: &str, pass: &str) -> bool;
    fn add_user(&mut self, name: &str, password_hash: &str) -> Result<(), Error::MyError>;
    // Другие методы для работы с пользователями могут быть добавлены здесь
    fn get_user(&mut self, name: &str) -> Result<User, Error::MyError>;
    fn get_user_posts(&mut self, user: &str) -> Result<Vec<crate::post::Post>, Error::MyError>;
    fn add_post(&mut self, user: &str, post: crate::post::PostForm) -> Result<(), Error::MyError>;
}

pub fn calculate_password_hash(pass: &str) -> String {
    let data = pass.as_bytes();
    let hash = crypto_hash::hex_digest(crypto_hash::Algorithm::SHA256, data);
    hash
}