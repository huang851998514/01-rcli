use core::fmt;
use std::{path::PathBuf, str::FromStr};

use anyhow::Ok;
use clap::Parser;

use super::{verify_file, verify_path};

#[derive(Debug, Parser)]
pub enum TextSubCommand {
    #[command(about = "为文本签名")]
    Sign(TextSginOptions),
    #[command(about = "为文本验证签名")]
    Verify(TextVerifyOptions),
    #[command(about = "生成签名用的key")]
    Generate(KeyGeneratorOptions),
}

#[derive(Debug, Parser)]
pub struct TextSginOptions {
    #[arg(short, long, value_parser = verify_file, default_value = "-")]
    pub input: String,
    #[arg(short, long, value_parser = verify_file)]
    pub key: String,
    #[arg(long, default_value = "blake3", value_parser = sign_parse_format)]
    pub format: TextSignFormat,
}

#[derive(Debug, Parser)]
pub struct TextVerifyOptions {
    #[arg(short, long, value_parser = verify_file, default_value = "-")]
    pub input: String,
    #[arg(short, long, value_parser = verify_file)]
    pub key: String,
    #[arg(short, long)]
    pub signature: String,
    #[arg(long, default_value = "blake3", value_parser = sign_parse_format)]
    pub format: TextSignFormat,
}

#[derive(Debug, Parser)]
pub struct KeyGeneratorOptions {
    #[arg(long, default_value = "blake3", value_parser = sign_parse_format)]
    pub format: TextSignFormat,
    #[arg(short, long, value_parser = verify_path)]
    pub output: PathBuf,
}

#[derive(Debug, Clone, Copy)]
pub enum TextSignFormat {
    Blake3,
    Ed25519,
}

fn sign_parse_format(s: &str) -> anyhow::Result<TextSignFormat> {
    s.parse()
}

impl FromStr for TextSignFormat {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "blake3" => Ok(Self::Blake3),
            "ed25519" => Ok(Self::Ed25519),
            _ => Err(anyhow::anyhow!("不支持的签名格式")),
        }
    }
}

impl From<TextSignFormat> for &'static str {
    fn from(format: TextSignFormat) -> Self {
        match format {
            TextSignFormat::Blake3 => "blake3",
            TextSignFormat::Ed25519 => "ed25519",
        }
    }
}

impl fmt::Display for TextSignFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", Into::<&'static str>::into(*self))
    }
}
