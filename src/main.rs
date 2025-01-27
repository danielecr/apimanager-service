// a service listing http on port 8080
// using axum
use axum::{routing::get, Router};
use std::net::SocketAddr;
use apimanager_service::assets::static_files::INDEX_HTML;

#[tokio::main]
async fn main() {
    //let app ("/", get(|| async { "Hello, World!" }));

    println!("{:?}", INDEX_HTML);
    println!("Hello, world!");
}
