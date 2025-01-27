use std::env;
use dotenv;

// a service listing http on port 8080
// using axum
use axum::{routing::get, Router};
use apimanager_service::assets::static_files::{INDEX_BUNDLE_JS, INDEX_BUNDLE_JS_MAP, INDEX_HTML};


async fn get_services_handler(service_manager_service: String) -> String {
    let services = reqwest::get(format!("{}/api/services", service_manager_service))
        .await.unwrap()
        .text()
        .await.unwrap();
    services
}

#[tokio::main]
async fn main() {
    let _ = dotenv::dotenv();
    let service_manager_service = env::var("SERVICE_MANAGER_SERVICE").expect("SERVICE_MANAGER_SERVICE must be set");
    //let get_services_handler = create_get_services_handler(service_manager_service);

    let app = Router::new().route("/index.html", get(|| async { INDEX_HTML }))
    .route("/index.bundle.js", get(|| async { INDEX_BUNDLE_JS}))
    .route("/index.bundle.js.map", get(|| async { INDEX_BUNDLE_JS_MAP}))
    .route("/api/services", get(move || {
        let service_manager_service = service_manager_service.clone();
        async move { get_services_handler(service_manager_service).await }
    }));


    // run it
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
    println!("{:?}", INDEX_HTML);
    println!("Hello, world!");
}
