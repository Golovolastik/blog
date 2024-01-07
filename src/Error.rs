#[derive(Debug)]
pub enum MyError {
    UserAlreadyExists,
    UserNotExists,
    DatabaseError(postgres::Error),
}

impl std::fmt::Display for MyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            MyError::UserAlreadyExists => write!(f, "User already exists"),
            MyError::UserNotExists => write!(f, "User not exists"),
            MyError::DatabaseError(ref err) => write!(f, "Database error: {}", err),
        }
    }
}

impl std::error::Error for MyError {}