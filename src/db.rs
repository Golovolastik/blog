use std::io::Read;
use postgres;
use postgres::{Client, NoTls};
use crate::Error;
use crate::Error::MyError;
use crate::user::{User, UserRepository};


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

    fn check_pass(&mut self, name: &str, pass: &str) -> bool {
        let hash = crate::user::calculate_password_hash(pass);
        let db_hash = self.client.query(
            "SELECT pass_hash FROM blog_user WHERE nick_name = $1",
            &[&name],
        );
        match db_hash {
            Err(err) => println!("Something went wrong!"),
            Ok(result) => {
                for row in result {
                    match row.get::<usize, String>(0) {
                        query => {
                            if query == hash {
                                return true;
                            }
                        }
                        _ => println!("Something went wrong!")
                    }
                }
            }
        }
        false
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
    fn get_user(&mut self, name: &str) -> Result<crate::user::User, Error::MyError> {
        if let Ok(true) = crate::user::UserRepository::check_user_availability(self, name) {
            return Err(Error::MyError::UserNotExists);
        }
        //let hash = crate::user::calculate_password_hash(password);
        let result =  self.client.query(
            "SELECT * FROM blog_user WHERE nick_name = $1",
            &[&name]
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

    fn get_user_posts(&mut self, user: &str) -> Result<Vec<crate::post::Post>, MyError> {
        let result = self.client.query(
            "SELECT * FROM post JOIN blog_user USING(user_id) WHERE nick_name = $1",
            &[&user]
        );
        if let Err(err) = result {
            return Err(Error::MyError::UserNotExists)}
        let mut posts: Vec<crate::post::Post> = Vec::new();
        for row in result.unwrap() {
            let record = crate::post::Post {
                author_id: row.get(0),
                //post_id: row.get(1),
                header: row.get(2),
                content: row.get(3),
            };
            &posts.push(record);
        }
        Ok(posts)
    }

    fn add_post(&mut self, user: &str, post: crate::post::PostForm) -> Result<(), MyError> {
        println!("Hello from database");
        println!("User {user}");
        println!("{:?}", post);
        match self.client.execute(
            "INSERT INTO post (user_id, title, content) \
                    VALUES ((SELECT user_id FROM blog_user WHERE nick_name = $1), $2, $3);",
            &[&user, &post.title, &post.content],
        ) {
            Ok(1) => println!("Post added"),
            Ok(num) => println!("Undefined behavior"),
            Err(err) => {
                eprintln!("{:?}", err);
                return Err(MyError::UserNotExists)
            }
        }
        Ok(())
    }
}

pub fn connect() -> Result<PostgresUserRepository, postgres::Error> {
    // Подключение к базе данных
    let mut client = Client::connect("postgresql://postgres:example@localhost:5432/blog", NoTls)?;
    let mut admin = PostgresUserRepository::new(client);
    Ok(admin)

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
    let admin = db.get_user("Golovolastik");
    //println!("{:?}", db.get_user_posts(admin.unwrap()));
    // for post in db.get_user_posts(admin.unwrap()) {
    //     println!("{:?}", post);
    // }
    // let post = crate::post::Post {
    //     //post_id: 0,
    //     author_id: admin.as_ref().unwrap().id,
    //     header: "Tilimilitryamdia".to_string(),
    //     content: "Story about fairy country...".to_string(),
    // };
    // match db.add_post(admin.unwrap(), post) {
    //     Ok(()) => println!("Success"),
    //     Err(err) => println!("Something wrong: {:?}", err),
    // };
    println!("{:?}", db.check_pass("Golovolastik", "mob5008"));
}