use anyhow::Result;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::get,
    Router,
};
use std::{net::SocketAddr, path::PathBuf, sync::Arc};
use tower_http::services::ServeDir;
use tracing::{info, warn};

#[derive(Debug)]
struct HttpServeState {
    path: PathBuf,
}

pub async fn process_http_serve(path: PathBuf, port: u16) -> Result<()> {
    let state = HttpServeState { path: path.clone() };
    let dir_service = ServeDir::new(path.clone())
        .append_index_html_on_directories(true)
        .precompressed_gzip()
        .precompressed_br()
        .precompressed_deflate()
        .precompressed_zstd();
    let router = Router::new()
        .nest_service("/tower", dir_service)
        .route("/*path", get(file_handler))
        .with_state(Arc::new(state));
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    info!("http服务启动成功，serving:{:?} on port:{}", path, port);
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, router).await?;
    Ok(())
}

async fn file_handler(
    State(state): State<Arc<HttpServeState>>,
    Path(path): Path<String>,
) -> (StatusCode, String) {
    let p = std::path::Path::new(&state.path).join(path);
    if !p.exists() {
        (
            StatusCode::NOT_FOUND,
            format!("文件不存在：{}", p.display()),
        )
    } else {
        match tokio::fs::read_to_string(&p).await {
            Ok(content) => {
                info!("文件读取成功：{}", p.display());
                (StatusCode::OK, content)
            }
            Err(e) => {
                warn!("文件读取失败：{}，错误：{}", p.display(), e);
                (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
            }
        }
    }
}
