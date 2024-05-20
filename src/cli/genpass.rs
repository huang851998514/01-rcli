use crate::{process_genpass, CmdExector};
use anyhow::Ok;
use clap::Parser;
use zxcvbn::zxcvbn;

#[derive(Parser, Debug)]
pub struct GenPassOptions {
    #[arg(short, long, default_value_t = 16)]
    pub length: u8,

    #[arg(long, default_value_t = false)]
    pub no_uppercase: bool,

    #[arg(long, default_value_t = false)]
    pub no_lowercase: bool,

    #[arg(long, default_value_t = false)]
    pub no_number: bool,

    #[arg(long, default_value_t = false)]
    pub no_symbol: bool,
}

impl CmdExector for GenPassOptions {
    async fn execute(self) -> anyhow::Result<()> {
        let pass = process_genpass(
            self.length,
            self.no_uppercase,
            self.no_lowercase,
            self.no_number,
            self.no_symbol,
        )?;
        print!("{}", pass);

        //密码强度检测
        let result = zxcvbn(&pass, &[])?;
        eprintln!("密码强度：{}", result.score());
        Ok(())
    }
}
