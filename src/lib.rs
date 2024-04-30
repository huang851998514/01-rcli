mod options;
mod process;

pub use options::{Cli, SubCommand};
pub use process::{process_csv, process_genpass};
