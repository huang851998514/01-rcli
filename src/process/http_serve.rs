use std::path::Path;

use tracing::info;

#[allow(dead_code)]
fn process_htt_serve(path: &Path, port: u16) {
    info!("http服务启动成功，serving:{:?} on port:{}", path, port);
}
