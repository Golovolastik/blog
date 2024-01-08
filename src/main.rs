use course_project::socket;

fn main() {
    socket::listen();
    //course_project::db::connect();
    //let result = course_project::db::check_user_availability("Golovolastik");
    //println!("{:?}", result);

    //course_project::db::test_func();

    // if let Err(e) = course_project::db::connect() {
    //     eprintln!("Ошибка при подключении к базе данных: {}", e);
    // }
}
