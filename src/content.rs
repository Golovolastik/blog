use handlebars::Handlebars;
use std::collections::BTreeMap;
use std::fs;
use crate::db::PostgresUserRepository;
use crate::user::UserRepository;

pub fn generate(db: &mut PostgresUserRepository, user: Option<String>) -> String {
    // Создаем экземпляр Handlebars
    let mut handlebars = Handlebars::new();
    //db.get_user_posts()
    // заголовок и текст
    let mut data = BTreeMap::new();
    let posts = db.get_user_posts(user.unwrap()).unwrap();
    // data.insert("posts", vec![
    //     ("Post 1", "Lorem ipsum dolor sit amet, consectetur adipiscing elit."),
    //     ("Post 2", "Quisque ullamcorper placerat ipsum. Cras nibh."),
    // ]);
    data.insert("posts", posts);

    // Чтение шаблона из файла
    let template = fs::read_to_string("content_page.hbs").expect("Не удалось прочитать шаблон");

    // Компилируем шаблон
    handlebars
        .register_template_string("blog_template", template)
        .expect("Не удалось зарегистрировать шаблон");

    // Рендерим HTML с данными
    let rendered = handlebars.render("blog_template", &data).unwrap();
    rendered

    // Отправляем HTML страницу по запросу
    // Здесь нужно использовать ваш код сервера для отправки содержимого rendered по запросу
    // Например:
    // server.send_response(rendered);
}
