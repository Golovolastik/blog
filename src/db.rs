use std::io::Read;
use postgres;
use postgres::{Client, NoTls};
use crate::Error;
use crate::user::UserRepository;


pub struct PostgresUserRepository {
    client: Client,
}

impl PostgresUserRepository {
    pub fn new(client: Client) -> Self {
        PostgresUserRepository { client }
    }
}

impl crate::user::UserRepository for PostgresUserRepository {
    fn check_user_availability(&mut self, name: &str) -> Result<bool, Error::MyError> {
        match self.client.query("SELECT * FROM blog_user WHERE nick_name = $1", &[&name]) {
            Ok(rows) => {
                for row in rows {
                    if let Some(data) = Some(row.get::<usize, &str>(1)) {
                        return Ok(false);
                    }
                }
                Ok(true)
            }
            Err(err) => {
                println!("{:?}", err);
                Err(Error::MyError::UserAlreadyExists)
            } // Возвращаем ваш тип ошибки Error::MyError
        }
    }

    fn add_user(&mut self, name: &str, password: &str) -> Result<(), Error::MyError> {
        if let Ok(false) = crate::user::UserRepository::check_user_availability(self, name) {
            return Err(Error::MyError::UserAlreadyExists);
        }
        let hash = crate::user::calculate_password_hash(password);
        match self.client.execute(
            "INSERT INTO blog_user (nick_name, pass_hash) VALUES ($1, $2)",
            &[&name, &hash],
        ) {
            Ok(row) => println!("{} rows inserted", row),
            Err(err) => {
                println!("{:?}", err);
                return Err(Error::MyError::UserAlreadyExists)
            }
        }
        Ok(())
    }
    // Добавьте реализацию других методов работы с пользователями по необходимости
    fn get_user(&mut self, name: &str, password: &str) -> Result<crate::user::User, Error::MyError> {
        if let Ok(true) = crate::user::UserRepository::check_user_availability(self, name) {
            return Err(Error::MyError::UserNotExists);
        }
        let hash = crate::user::calculate_password_hash(password);
        let result =  self.client.query(
            "SELECT * FROM blog_user WHERE nick_name = $1 AND pass_hash = $2",
            &[&name, &hash]
        );
        match result {
            Ok(ref row) => {
                if row.len() > 1 {
                    return Err(Error::MyError::UserAlreadyExists)
                }
            },
            Err(_) => return Err(Error::MyError::UserAlreadyExists)
        }
        let user: crate::user::User = {
            let mut user_data: Option<crate::user::User> = None;
            for row in result.unwrap() {
                user_data = Some(crate::user::User {
                    id: row.get(0),
                    login: row.get(1),
                    password_hash: row.get(2),
                });
            }

            // Возвращаем последнее значение user_data
            user_data.unwrap()
        };
        Ok(user)
    }
}

pub fn connect() -> Result<PostgresUserRepository, postgres::Error> {
    // Подключение к базе данных
    let mut client = Client::connect("postgresql://postgres:example@localhost:5432/blog", NoTls)?;
    let mut admin = PostgresUserRepository::new(client);
    Ok(admin)


    // // Попытка выполнить простой запрос, например, SHOW VERSION
    // let version_query = client.query("SHOW SERVER_VERSION", &[])?;
    //
    // if let Some(row) = version_query.get(0) {
    //     let version: String = row.get(0);
    //     println!("Успешное подключение! Версия сервера: {}", version);
    // } else {
    //     println!("Не удалось получить версию сервера.");
    // }

//     client.batch_execute("
//     CREATE TABLE person (
//         id      SERIAL PRIMARY KEY,
//         name    TEXT NOT NULL,
//         data    BYTEA
//     )
// ")?;
//
//     let name = "Ferris";
//     let data = None::<&[u8]>;
//     client.execute(
//         "INSERT INTO person (name, data) VALUES ($1, $2)",
//         &[&name, &data],
//     )?;

//     !!!!!!!!!!!!!!!
//         !!!!!!!!!!!
// !!!!!!!!!!!!!!!!!!!!
//     for row in client.query("SELECT content FROM post", &[])? {
//         // let id: i32 = row.get(0);
//         // let name: &str = row.get(1);
//         let data: &str = row.get(0);
//
//         println!("found post: {:?}", data);
//     }
//     Ok(())
}

// pub fn add_user_to_db(name: &str, pass_hash: &str) -> Result<(), Error> {
//     check_user_availability(&name);
//     //add_to_db(&name, &pass_hash);
//     Ok(())
// }
// pub fn check_user_availability(name: &str) -> Result<bool, Error> {
//     let mut client = Client::connect("postgresql://postgres:example@localhost:5432/blog", NoTls)?;
//
//     for row in client.query("SELECT * FROM blog_user WHERE nick_name = $1", &[&name])? {
//         if let data = row.get::<usize, &str>(1) {
//             return Ok(false)
//         }
//
//         //println!("found nickname: {:?}", data);
//     }
//     Ok(true)
//     //"SELECT COUNT(*) FROM blog_user WHERE nick_name = $1"
// }

// fn insert_hash(user_id: i32, password: &str) -> Result<(), Error> {
//     let data = password.as_bytes();
//     let hash = crypto_hash::hex_digest(crypto_hash::Algorithm::SHA256, data);
//
//     let mut client = Client::connect("postgresql://postgres:example@localhost:5432/blog", NoTls)?;
//
//     let stmt = client.prepare("INSERT INTO auth (user_id, pass_hash) VALUES ($1, $2)")?;
//     client.execute(&stmt, &[&user_id, &hash])?;
//
//     Ok(())
// }

pub fn test_func() {
    //println!("{:?}",insert_hash(1, "mob5651008"));
    let mut connection = connect();
    let mut db: PostgresUserRepository;
    if let Err(_) = connection {
        panic!("Can't connect to database");
    } else {
        db = connection.unwrap();
    }
    //db.add_user("ololo", "safqfcvqe").unwrap();
    //println!("{:?}", db.get_user("Golovolastik", "mob5651008"));
}