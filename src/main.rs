use clap::Parser;
use rcli::{process_csv, process_genpass, Cli, SubCommand, Base64SubCommand};

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
        SubCommand::GenPass(option) => {
            process_genpass(
                option.length,
                option.no_uppercase,
                option.no_lowercase,
                option.no_number,
                option.no_symbol,
            )?;
        },
        SubCommand::Base64(sub_command) => match sub_command {
            Base64SubCommand::Encode(option) => {
                println!("encode: {:?}", option);
            }
            Base64SubCommand::Decode(option) => {
                println!("decode: {:?}", option);
            }
        },
    }
    Ok(())
}
