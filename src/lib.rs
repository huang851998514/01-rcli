mod cli;
mod process;
mod utils;

pub use cli::{
    Base64DecodeOptions, Base64EncodeOptions, Base64Format, Base64SubCommand, Cli, CsvOptions,
    GenPassOptions, HttpServeOptions, HttpSubCommand, KeyGeneratorOptions, SubCommand,
    TextSginOptions, TextSignFormat, TextSubCommand, TextVerifyOptions,
};
use enum_dispatch::enum_dispatch;
pub use process::{
    process_csv, process_decode, process_encode, process_genpass, process_http_serve,
    process_text_generate, process_text_sign, process_text_verify,
};

#[enum_dispatch]
#[allow(async_fn_in_trait)]
pub trait CmdExector {
    async fn execute(self) -> anyhow::Result<()>;
}
