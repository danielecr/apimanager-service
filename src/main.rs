use std::env;
use dotenv;

// a service listing http on port 8080
// using axum
use axum::{extract::{Path,State}, http::{HeaderMap, HeaderValue}, response::IntoResponse, routing::get, Router};
use apimanager_service::assets::{static_files::{INDEX_BUNDLE_JS, INDEX_BUNDLE_JS_MAP, INDEX_HTML}, STATIC_FILEMAP_MIME};
use apimanager_service::assets::STATIC_FILEMAP;
use reqwest::{header, StatusCode};

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

fn static_routes() -> Router {
    let prefix = "/";
    let mut static_pages = Router::new();
    for (k, v) in STATIC_FILEMAP.entries() {
        let mime = STATIC_FILEMAP_MIME.get(k).unwrap_or(&"application/octet-stream");
        let k = format!("{}{}", prefix, k);
        static_pages = static_pages.clone().route(&k,
            get(move || async move {
            let mut headers = HeaderMap::new();
            headers.insert(header::CONTENT_TYPE, HeaderValue::from_static(*mime));
            (headers, *v).into_response()
        }));
    }
    static_pages = static_pages.fallback(fallback);
    static_pages
} 

#[tokio::main]
async fn main() {
    let _ = dotenv::dotenv();
    let service_manager_service = env::var("SERVICE_MANAGER_SERVICE").expect("SERVICE_MANAGER_SERVICE must be set");
    let static_pages = static_routes();
 
    let state = Appstate {
        service_manager_service,
    };

    let app_routes = Router::new()
    .route("/api/services", get(get_services_handler))
    .route("/api/service/{name}", get(get_servname_handler))
    .route("/api/resources", get(get_resources_handler))
    .with_state(state);
 
    let app = Router::new()
    .merge(static_pages)
    .merge(app_routes);
    // run it
    let port: u16 = env::var("PORT").unwrap_or_else(|_| "3000".to_string()).parse().unwrap();

    let addr = format!("0.0.0.0:{}", port);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
    println!("{:?}", INDEX_HTML);
    println!("Hello, world!");
}

async fn fallback() -> (StatusCode, String) {
    (StatusCode::NOT_FOUND, format!("Cannot find "))
}
