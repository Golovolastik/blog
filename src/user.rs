use postgres;
use crate::Error;
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct User {
    pub id: i16,
    pub login: String,
    pub password_hash: String,
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
    fn get_user(&mut self, name: &str, password: &str) -> Result<User, Error::MyError>;
    fn get_user_posts(&mut self, user: crate::user::User) -> Result<Vec<crate::post::Post>, Error::MyError>;
    fn add_post(&mut self, user: crate::user::User, post: crate::post::Post) -> Result<(), Error::MyError>;
}

pub fn calculate_password_hash(pass: &str) -> String {
    // let data = pass.as_bytes();
    // let result = digest(Algorithm::SHA256, data);
    //
    // 42
    let data = pass.as_bytes();
    let hash = crypto_hash::hex_digest(crypto_hash::Algorithm::SHA256, data);
    hash
}