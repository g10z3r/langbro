use mongodb::{bson::doc, options::ClientOptions, sync::Client};

use std::time::Duration;

pub fn connect() -> Option<Client> {
    let mut options = ClientOptions::parse(dotenv!("MONGO_DB_URL")).unwrap();

    // Параметры соединения
    let duration: Duration = Duration::new(60, 0);
    options.app_name = Some("Stuffy Krill".to_string());
    options.connect_timeout = Some(duration);

    // Получение дескриптора кластера
    let client: Result<Client, mongodb::error::Error> = Client::with_options(options);
    match client {
        Ok(c) => {
            let ping = c
                .database(dotenv!("MONGO_DATABASE_NAME"))
                .run_command(doc! {"ping": 1}, None)
                .unwrap();
            println!("{}", ping);
            Some(c)
        }
        Err(_) => Option::None,
    }
}
