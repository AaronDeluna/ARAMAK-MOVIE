

use axum::{response::Html, routing::get, Router};
use tokio_postgres::{Client, Config, NoTls, Error};
use std::io;
use std::fs;
#[allow(unused_imports)]
use std::path::PathBuf;
#[allow(unused_imports)]
use tokio::io::split;



//database configuration



async fn manage_database() -> Result<(), Error> {
    // Параметры подключения к базе данных
    let db_name = "AramakMuvi";
    let user = "postgres";
    let password = "halw8301";
    let host = "127.0.0.1";
    let port = "3000";

    // Подключение к существующей базе данных "postgres"
    let mut config = Config::new();
    config.user(user).password(password).host(host).port(port.parse::<u16>().unwrap());
    let (client, connection) = config.connect(NoTls).await?;

    // Создание новой базы данных
    client.execute(&format!("CREATE DATABASE {}", db_name), &[]).await?;

    // Закрытие соединения с базой данных "postgres"
    connection.await?;

    // Параметры подключения к новой базе данных
    let new_db_url = format!("postgres://{}:{}@{}:{}/{}", user, password, host, port, db_name);
    let mut new_config = Config::new();
    new_config.user(user).password(password).host(host).port(port.parse::<u16>().unwrap());
    let (new_client, new_connection) = new_config.connect(NoTls).await?;

    // Создание таблицы "users" с полями "password" и "email"
    new_client
        .execute(
            "CREATE TABLE users (id SERIAL PRIMARY KEY, password TEXT, email TEXT)",
            &[],
        )
        .await?;

    // Вставка тестовых данных
    new_client
        .execute(
            "INSERT INTO users (password, email) VALUES ('password123', 'test@example.com')",
            &[],
        )
        .await?;

    // Закрытие соединения с новой базой данных
    new_connection.await?;

    Ok(())
}

#[tokio::main]
async fn main() {
    if let Err(e) = manage_database().await {
        eprintln!("Ошибка при работе с базой данных: {}", e);
    }
}



//server connection



async fn home() -> Html<String> {
    let html = fs::read_to_string("src/login.html").unwrap();
    Html(html)
}

fn server_config () {
    #[tokio::main]
    async fn main() {
        let app = Router::new().route("/", get(home));

        axum::Server::bind(&"127.0.0.1:3000".parse().unwrap())
            .serve(app.into_make_service())
            .await
            .unwrap();
        println!("Server is running");
    }
}