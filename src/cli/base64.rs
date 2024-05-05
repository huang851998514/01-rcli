use super::verify_input_file;
use clap::Parser;

#[derive(Parser, Debug)]
pub enum Base64SubCommand {
    #[command(name = "encode", about = "加密")]
    Encode(Base64EncodeOptions),
    #[command(name = "decode", about = "解密")]
    Decode(Base64DecodeOptions),
}

#[derive(Parser, Debug)]
pub struct Base64EncodeOptions {
    #[arg(short, long, value_parser = verify_input_file, default_value = "-")]
    pub input: String,
    #[arg(short, long)]
    pub output: Option<String>,
}

#[derive(Parser, Debug)]
pub struct Base64DecodeOptions {
    #[arg(short, long, value_parser = verify_input_file, default_value = "-")]
    pub input: String,
    #[arg(short, long)]
    pub output: Option<String>,
}
