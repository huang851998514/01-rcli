use std::path::PathBuf;

use clap::Parser;

use crate::{process_http_serve, CmdExector};

use super::verify_path;

#[derive(Parser, Debug)]
pub enum HttpSubCommand {
    #[command(about = "http文件服务")]
    Serve(HttpServeOptions),
}

#[derive(Parser, Debug)]
pub struct HttpServeOptions {
    #[arg(short, long,value_parser=verify_path, default_value = ".")]
    pub dir: PathBuf,
    #[arg(short, long, default_value = "8080")]
    pub port: u16,
}

impl CmdExector for HttpSubCommand {
    async fn execute(self) -> anyhow::Result<()> {
        match self {
            HttpSubCommand::Serve(options) => options.execute().await,
        }
    }
}

impl CmdExector for HttpServeOptions {
    async fn execute(self) -> anyhow::Result<()> {
        // 启动http文件服务
        process_http_serve(self.dir, self.port).await
    }
}
