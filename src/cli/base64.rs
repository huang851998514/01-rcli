use core::fmt;
use std::str::FromStr;

use crate::{process_decode, process_encode, CmdExector};

use super::verify_file;
use clap::Parser;
use enum_dispatch::enum_dispatch;

#[enum_dispatch(CmdExector)]
#[derive(Parser, Debug)]
pub enum Base64SubCommand {
    #[command(name = "encode", about = "加密")]
    Encode(Base64EncodeOptions),
    #[command(name = "decode", about = "解密")]
    Decode(Base64DecodeOptions),
}

#[derive(Parser, Debug)]
pub struct Base64EncodeOptions {
    #[arg(short, long, value_parser = verify_file, default_value = "-")]
    pub input: String,
    #[arg(long, value_parser = parse_base64_format ,default_value = "standard")]
    pub format: Base64Format,
}

#[derive(Parser, Debug)]
pub struct Base64DecodeOptions {
    #[arg(short, long, value_parser = verify_file, default_value = "-")]
    pub input: String,
    #[arg(long, value_parser = parse_base64_format ,default_value = "standard")]
    pub format: Base64Format,
}

#[derive(Debug, Clone, Copy)]
pub enum Base64Format {
    Standard,
    UrlSafe,
}

fn parse_base64_format(format: &str) -> Result<Base64Format, anyhow::Error> {
    format.parse()
}

impl FromStr for Base64Format {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "standard" => Ok(Base64Format::Standard),
            "urlsafe" => Ok(Base64Format::UrlSafe),
            _ => Err(anyhow::anyhow!("不支持的base64格式: {}", s)),
        }
    }
}

impl From<Base64Format> for &'static str {
    fn from(format: Base64Format) -> &'static str {
        match format {
            Base64Format::Standard => "standard",
            Base64Format::UrlSafe => "urlsafe",
        }
    }
}

impl fmt::Display for Base64Format {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", Into::<&str>::into(*self))
    }
}

// impl CmdExector for Base64SubCommand {
//     async fn execute(self) -> anyhow::Result<()> {
//         match self {
//             Base64SubCommand::Encode(option) => option.execute().await,
//             Base64SubCommand::Decode(option) => option.execute().await,
//         }
//     }
// }

impl CmdExector for Base64EncodeOptions {
    async fn execute(self) -> anyhow::Result<()> {
        let encode = process_encode(&self.input, self.format)?;
        print!("{}", encode);
        Ok(())
    }
}

impl CmdExector for Base64DecodeOptions {
    async fn execute(self) -> anyhow::Result<()> {
        let decode = process_decode(&self.input, self.format)?;
        //TODO: decode出来的不一定是string，这里先这样处理
        let decode = String::from_utf8(decode)?;
        let decode = decode.trim().to_string();
        print!("{}", decode);
        Ok(())
    }
}
