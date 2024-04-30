use clap::Parser;
use rcli::{process_csv, Cli, SubCommand};

fn main() -> anyhow::Result<()> {
    let cli_parse = Cli::parse();
    match cli_parse.cmd {
        SubCommand::Csv(csv) => {
            process_csv(&csv.input, &csv.output)?;
        }
    }
    Ok(())
}
