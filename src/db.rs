use std::io::Read;
use postgres::{Client, NoTls, Error};

pub fn connect() -> Result<(), Error> {
    // Подключение к базе данных
    let mut client = Client::connect("postgresql://postgres:example@localhost:5432/test", NoTls)?;

    // Попытка выполнить простой запрос, например, SHOW VERSION
    let version_query = client.query("SHOW SERVER_VERSION", &[])?;

    if let Some(row) = version_query.get(0) {
        let version: String = row.get(0);
        println!("Успешное подключение! Версия сервера: {}", version);
    } else {
        println!("Не удалось получить версию сервера.");
    }

    client.batch_execute("
    CREATE TABLE person (
        id      SERIAL PRIMARY KEY,
        name    TEXT NOT NULL,
        data    BYTEA
    )
")?;

    let name = "Ferris";
    let data = None::<&[u8]>;
    client.execute(
        "INSERT INTO person (name, data) VALUES ($1, $2)",
        &[&name, &data],
    )?;

    for row in client.query("SELECT id, name, data FROM person", &[])? {
        let id: i32 = row.get(0);
        let name: &str = row.get(1);
        let data: Option<&[u8]> = row.get(2);

        println!("found person: {} {} {:?}", id, name, data);
    }
    Ok(())
}