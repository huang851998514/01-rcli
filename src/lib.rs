mod cli;
mod process;

pub use cli::{Base64SubCommand, Cli, SubCommand};
pub use process::{process_csv, process_genpass};
