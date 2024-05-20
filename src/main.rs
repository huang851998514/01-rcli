use clap::Parser;
use rcli::{Cli, CmdExector};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    let cli_parse = Cli::parse();
    cli_parse.cmd.execute().await
}
