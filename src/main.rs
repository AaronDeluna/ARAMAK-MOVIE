mod database;
mod server;


use axum::{response::Html, routing::get, Router};
use postgres::{Client, NoTls};
use std::io;
use std::fs;
#[allow(unused_imports)]
use std::path::PathBuf;
#[allow(unused_imports)]
use tokio::io::split;




fn main() {

}