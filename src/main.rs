use clap::Parser;
use rcli::{process_csv, Cli, SubCommand};

fn main() -> anyhow::Result<()> {
    let cli_parse = Cli::parse();
    match cli_parse.cmd {
        SubCommand::Csv(option) => {
            let output = match &option.output {
                Some(output) => output.clone(),
                None => format!("output.{}", option.format),
            };
            process_csv(&option.input, output, option.format)?;
        }
    }
    Ok(())
}
