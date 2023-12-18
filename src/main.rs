use course_project::socket;

fn main() {
    socket::listen();

    // if let Err(e) = course_project::db::connect() {
    //     eprintln!("Ошибка при подключении к базе данных: {}", e);
    // }
}
