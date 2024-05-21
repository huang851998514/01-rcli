use core::fmt;
use std::{path::PathBuf, str::FromStr};

use anyhow::Ok;
use clap::Parser;
use enum_dispatch::enum_dispatch;

use crate::{process_text_generate, process_text_sign, process_text_verify, CmdExector};

use super::{verify_file, verify_path};

#[enum_dispatch(CmdExector)]
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

// impl CmdExector for TextSubCommand {
//     async fn execute(self) -> anyhow::Result<()> {
//         match self {
//             // 文本签名
//             TextSubCommand::Sign(options) => options.execute().await,
//             // 文本验证签名
//             TextSubCommand::Verify(options) => options.execute().await,
//             // 生成签名用的key
//             TextSubCommand::Generate(options) => options.execute().await,
//         }
//     }
// }

impl CmdExector for TextSginOptions {
    async fn execute(self) -> anyhow::Result<()> {
        let signed = process_text_sign(&self.input, &self.key, self.format)?;
        print!("{}", signed);
        Ok(())
    }
}

impl CmdExector for TextVerifyOptions {
    async fn execute(self) -> anyhow::Result<()> {
        let verify = process_text_verify(&self.input, &self.key, &self.signature, self.format)?;
        println!("{}", verify);
        Ok(())
    }
}

impl CmdExector for KeyGeneratorOptions {
    async fn execute(self) -> anyhow::Result<()> {
        let key = process_text_generate(self.format)?;
        let path = self.output;
        match self.format {
            // 生成blake3签名用的key
            TextSignFormat::Blake3 => {
                let name = path.join("blake3.txt");
                tokio::fs::write(name, &key[0]).await?;
                Ok(())
            }
            // 生成ed25519签名用的key
            TextSignFormat::Ed25519 => {
                tokio::fs::write(path.join("ed25519.sk"), &key[0]).await?;
                tokio::fs::write(path.join("ed25519.pk"), &key[1]).await?;
                Ok(())
            }
        }
    }
}
