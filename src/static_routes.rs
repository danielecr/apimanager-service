use axum::{http::{HeaderMap, HeaderValue}, response::IntoResponse, routing::get, Router};
use crate::assets::{STATIC_FILEMAP, STATIC_FILEMAP_MIME};
use reqwest::{header, StatusCode};

pub fn static_routes() -> Router {
    let prefix = "/";
    let mut static_pages = Router::new();
    for (k, v) in STATIC_FILEMAP.entries() {
        let mime = STATIC_FILEMAP_MIME.get(k).unwrap_or(&"application/octet-stream");
        let k = format!("{}{}", prefix, k);
        if k == format!("{}index.html", prefix) {
            let k = format!("{}", prefix);
            let v2 = v.to_string();
            static_pages = static_pages.clone().route(&k,
                get(move || async move {
                let mut headers = HeaderMap::new();
                headers.insert(header::CONTENT_TYPE, HeaderValue::from_static("text/html"));
                (headers, v2.clone()).into_response()
            }));
        }
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

async fn fallback() -> (HeaderMap, StatusCode) {
    let mut headers = HeaderMap::new();
    headers.insert(header::CONTENT_TYPE, HeaderValue::from_static("text/plain"));
    (headers, StatusCode::NOT_FOUND)
}