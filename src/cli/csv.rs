use crate::{process_csv, CmdExector};

use super::verify_file;
use std::{fmt, str::FromStr};

use anyhow::Ok;
use clap::Parser;

#[derive(Debug, Clone, Copy)]
pub enum OutputFormat {
    Json,
    Yaml,
}

#[derive(Parser, Debug)]
pub struct CsvOptions {
    #[arg(short, long, value_parser=verify_file)]
    pub input: String,
    #[arg(short, long)]
    pub output: Option<String>,
    #[arg(short, long, default_value_t = ',')]
    pub delimiter: char,
    #[arg(long, default_value_t = true)]
    pub header: bool,
    #[arg(long,value_parser=parse_format, default_value = "json")]
    pub format: OutputFormat,
}

fn parse_format(format: &str) -> Result<OutputFormat, anyhow::Error> {
    format.parse()
}

impl FromStr for OutputFormat {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "json" => Ok(OutputFormat::Json),
            "yaml" => Ok(OutputFormat::Yaml),
            e => anyhow::bail!("不支持的格式:{}", e),
        }
    }
}

impl From<OutputFormat> for &'static str {
    fn from(value: OutputFormat) -> Self {
        match value {
            OutputFormat::Json => "json",
            OutputFormat::Yaml => "yaml",
        }
    }
}

impl fmt::Display for OutputFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        //write!(f, "{}", *self)
        write!(f, "{}", Into::<&str>::into(*self))
    }
}

impl CmdExector for CsvOptions {
    async fn execute(self) -> anyhow::Result<()> {
        let output = match &self.output {
            Some(output) => output.clone(),
            None => format!("output.{}", self.format),
        };
        process_csv(&self.input, output, self.format)?;
        Ok(())
    }
}
