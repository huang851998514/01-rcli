use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "rcli", version, author, about,long_about = None)]
struct Cli {
    #[command(subcommand)]
    cmd: SubCommand,
}

#[derive(Parser, Debug)]
enum SubCommand {
    #[command(about = "将csv文件转换为其他类型文件")]
    Csv(CsvOptions),
}

#[derive(Parser, Debug)]
struct CsvOptions {
    #[arg(short, long)]
    input: String,
    #[arg(short, long, default_value = "output.json")]
    output: String,
    #[arg(short, long, default_value_t = ',')]
    delimiter: char,
    #[arg(long, default_value_t = true)]
    header: bool,
}

fn main() {
    let cli_parse = Cli::parse();
    println!("{:?}", cli_parse);
}
