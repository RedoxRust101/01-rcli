use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::get,
    Router,
};
use std::{net::SocketAddr, path::PathBuf, sync::Arc};
use tokio::net::TcpListener;
use tracing::{info, warn};

#[derive(Debug)]
pub struct HttpServeState {
    pub path: PathBuf,
}

pub async fn process_http_serve(path: PathBuf, port: u16) -> anyhow::Result<()> {
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    info!("Serving directory {:?} on {}", path, addr);
    let listener = TcpListener::bind(addr).await?;
    let state = HttpServeState { path };
    // axum router
    let router = Router::new()
        .route("/*path", get(file_handler))
        .with_state(Arc::new(state));
    axum::serve(listener, router).await?;
    anyhow::Ok(())
}

async fn file_handler(
    State(state): State<Arc<HttpServeState>>,
    Path(path): Path<String>,
) -> (StatusCode, String) {
    let p = std::path::Path::new(&state.path).join(path);
    info!("Reading file {:?}", p);
    if !p.exists() {
        (
            StatusCode::NOT_FOUND,
            format!("File {} not found", p.display()),
        )
    } else {
        match tokio::fs::read_to_string(p).await {
            Ok(content) => {
                info!("Read {} bytes", content.len());
                (StatusCode::OK, content)
            }
            Err(e) => {
                warn!("Error reading file:{:?}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
            }
        }
    }
}