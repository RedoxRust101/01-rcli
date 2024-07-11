use axum::{
    extract::{Path, State},
    http::{StatusCode, Uri},
    response::IntoResponse,
    routing::get,
    Router,
};
use std::{net::SocketAddr, path::PathBuf, sync::Arc};
use tokio::fs;
use tokio::net::TcpListener;
use tower_http::services::ServeDir;
use tracing::info;
#[derive(Debug)]
pub struct HttpServeState {
    pub path: PathBuf,
}

pub async fn process_http_serve(path: PathBuf, port: u16) -> anyhow::Result<()> {
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    info!("Serving directory {:?} on {}", path, addr);
    let listener = TcpListener::bind(addr).await?;
    let state = HttpServeState { path: path.clone() };
    let dir_service = ServeDir::new(path)
        .append_index_html_on_directories(true)
        .precompressed_gzip()
        .precompressed_br()
        .precompressed_deflate()
        .precompressed_zstd();
    // axum router
    let router = Router::new()
        .route("/*path", get(file_handler))
        .nest_service("/tower", dir_service)
        .with_state(Arc::new(state));
    axum::serve(listener, router).await?;
    anyhow::Ok(())
}
async fn file_handler(
    uri: Uri,
    State(state): State<Arc<HttpServeState>>,
    Path(path): Path<String>,
) -> impl IntoResponse {
    let p: PathBuf = std::path::Path::new(&state.path).join(path);
    info!("Reading file {:?}", p);
    let base_uri_str = uri.to_string();
    if p.is_dir() {
        serve_directory(p, &base_uri_str).await
    } else {
        serve_file(p).await
    }
}

async fn serve_directory(
    path: PathBuf,
    base_uri_str: &str,
) -> (StatusCode, [(String, String); 1], String) {
    let mut entries = fs::read_dir(path).await.unwrap();
    let mut body = "<html><body><ul>".to_string();
    while let Some(entry) = entries.next_entry().await.unwrap() {
        let file_name = entry.file_name();
        let file_name_str = file_name.to_str().unwrap();
        body.push_str(&format!(
            "<li><a href=\"{}/{}\">{}</a></li>",
            base_uri_str, file_name_str, file_name_str
        ));
    }

    body.push_str("</ul></body></html>");
    (
        StatusCode::OK,
        [("Content-Type".into(), "text/html; charset=utf-8".into())],
        body,
    )
}

async fn serve_file(path: PathBuf) -> (StatusCode, [(String, String); 1], String) {
    match tokio::fs::read_to_string(path).await {
        Ok(content) => (
            StatusCode::OK,
            [("Content-Type".into(), "text/plain; charset=utf-8".into())],
            content,
        ),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            [("Content-Type".into(), "text/plain; charset=utf-8".into())],
            e.to_string(),
        ),
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_serve_directory() {
        let path = PathBuf::from("src/process");
        let base_uri_str = "http://localhost:8080/src/process";
        let (status, headers, body) = serve_directory(path, base_uri_str).await;
        assert_eq!(status, StatusCode::OK);
        assert_eq!(headers[0].0, "Content-Type");
        assert_eq!(headers[0].1, "text/html; charset=utf-8");
        assert!(body.contains("http_serve.rs"));
    }

    #[tokio::test]
    async fn test_serve_file() {
        let path = PathBuf::from("Cargo.toml");
        let (status, headers, body) = serve_file(path).await;
        assert_eq!(status, StatusCode::OK);
        assert_eq!(headers[0].0, "Content-Type");
        assert_eq!(headers[0].1, "text/plain; charset=utf-8");
        assert!(body.contains("axum"));
    }

    #[tokio::test]
    async fn test_file_handler() {
        let uri = Uri::from_static("http://localhost:8080/src/process/http_serve.rs");
        let state = Arc::new(HttpServeState {
            path: PathBuf::from("src/process"),
        });
        let path = Path("http_serve.rs".to_string());
        let response = file_handler(uri, State(state), path).await;
        let response = response.into_response();
        assert_eq!(response.status(), StatusCode::OK);
    }
}
