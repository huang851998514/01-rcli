mod base64;
mod csv;
mod genpass;
mod http;
mod text;

use std::path::Path;
use std::path::PathBuf;

use clap::Parser;

pub use self::base64::Base64Format;
pub use self::base64::Base64SubCommand;
pub use self::csv::OutputFormat;
pub use self::http::HttpSubCommand;
pub use self::text::{TextSignFormat, TextSubCommand};
use self::{csv::CsvOptions, genpass::GenPassOptions};

#[derive(Parser, Debug)]
#[command(name = "rcli", version, author, about,long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub cmd: SubCommand,
}

#[derive(Parser, Debug)]
pub enum SubCommand {
    #[command(about = "将csv文件转换为其他类型文件")]
    Csv(CsvOptions),
    #[command(name = "genpass", about = "生成随机密码")]
    GenPass(GenPassOptions),
    #[command(subcommand)]
    Base64(Base64SubCommand),
    #[command(subcommand)]
    Text(TextSubCommand),
    #[command(subcommand)]
    Http(HttpSubCommand),
}

fn verify_file(filename: &str) -> Result<String, &'static str> {
    if "-" == filename || Path::new(filename).exists() {
        Ok(filename.into())
    } else {
        Err("文件不存在")
    }
}

fn verify_path(path: &str) -> Result<PathBuf, &'static str> {
    let p = Path::new(path);
    if p.exists() && p.is_dir() {
        Ok(p.into())
    } else {
        Err("路径不存在或者不是文件夹")
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_verify_input_file() {
        assert_eq!(super::verify_file("Cargo.toml"), Ok("Cargo.toml".into()));
        assert_eq!(super::verify_file("-"), Ok("-".into()));
        assert_eq!(super::verify_file("not_exist.csv"), Err("文件不存在"));
    }
}
