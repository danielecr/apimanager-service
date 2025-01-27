// a service listing http on port 8080
// using axum
use axum::{routing::get, Router};
use std::net::SocketAddr;
use apimanager_service::assets::static_files::{INDEX_BUNDLE_JS, INDEX_BUNDLE_JS_MAP, INDEX_HTML};

#[tokio::main]
async fn main() {
    //let app ("/", get(|| async { "Hello, World!" }));
    let app = Router::new().route("/index.html", get(|| async { INDEX_HTML }))
    .route("/index.bundle.js", get(|| async { INDEX_BUNDLE_JS}))
    .route("/index.bundle.js.map", get(|| async { INDEX_BUNDLE_JS_MAP}));


    // run it
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
    println!("{:?}", INDEX_HTML);
    println!("Hello, world!");
}
