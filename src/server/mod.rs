use axum::{response::Html, routing::get, Router};
use std::fs;
#[allow(unused_imports)]
use std::path::PathBuf;
#[allow(unused_imports)]
use tokio::io::split;


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