use std::env;
use dotenv;

// a service listing http on port 8080
// using axum
use axum::{routing::get, Router, extract::{Path,State}};
use apimanager_service::assets::static_files::{INDEX_BUNDLE_JS, INDEX_BUNDLE_JS_MAP, INDEX_HTML};

#[derive(Clone)]
struct Appstate {
    service_manager_service: String,
}

async fn get_services_handler(State(state): State<Appstate>) -> String {
    let uri_part = "/api/services";
    let services = reqwest::get(format!("{}{}", &state.service_manager_service, uri_part))
        .await.unwrap()
        .text()
        .await.unwrap();
    services
}

async fn get_resources_handler(State(state): State<Appstate>) -> String {
    let uri_part = "/api/resources";
    let services = reqwest::get(format!("{}{}", &state.service_manager_service, uri_part))
        .await.unwrap()
        .text()
        .await.unwrap();
    services
}

async fn get_servname_handler(State(state): State<Appstate>, Path(name): Path<String>) -> String {
    let uri_part = "/api/service/";
    let services = reqwest::get(format!("{}{}{}", &state.service_manager_service, uri_part, &name))
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
    let state = Appstate {
        service_manager_service,
    };
    let app = Router::new().route("/index.html", get(|| async { INDEX_HTML }))
    .route("/index.bundle.js", get(|| async { INDEX_BUNDLE_JS}))
    .route("/index.bundle.js.map", get(|| async { INDEX_BUNDLE_JS_MAP}))
    .route("/api/services", get(get_services_handler))
    .route("/api/service/{name}", get(get_servname_handler))
    .route("/api/resources", get(get_resources_handler))
    .with_state(state);


    // run it
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
    println!("{:?}", INDEX_HTML);
    println!("Hello, world!");
}
