mod base64;
mod csv;
mod genpass;
use std::path::Path;

use clap::Parser;

pub use self::base64::Base64SubCommand;
pub use self::csv::OutputFormat;
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
}

fn verify_input_file(filename: &str) -> Result<String, &'static str> {
    
    if "-" == filename || Path::new(filename).exists() {
        Ok(filename.into())
    } else {
        Err("文件不存在")
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_verify_input_file() {
        assert_eq!(super::verify_input_file("Cargo.toml"), Ok("Cargo.toml".into()));
        assert_eq!(super::verify_input_file("-"), Ok("-".into()));
        assert_eq!(super::verify_input_file("not_exist.csv"), Err("文件不存在"));
    }
}