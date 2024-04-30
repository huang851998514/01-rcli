use clap::Parser;
use std::path::Path;

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
}

#[derive(Parser, Debug)]
pub struct CsvOptions {
    #[arg(short, long, value_parser=verify_input_file)]
    pub input: String,
    #[arg(short, long, default_value = "output.json")]
    pub output: String,
    #[arg(short, long, default_value_t = ',')]
    pub delimiter: char,
    #[arg(long, default_value_t = true)]
    pub header: bool,
}

fn verify_input_file(filename: &str) -> Result<String, &'static str> {
    if Path::new(filename).exists() {
        Ok(filename.into())
    } else {
        Err("文件不存在")
    }
}
