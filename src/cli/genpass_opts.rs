use anyhow::Result;
use clap::Parser;

use crate::{process_genpass, CmdExcutor};
use zxcvbn::zxcvbn;
#[derive(Debug, Parser)]
pub struct GenpassOpts {
    #[arg(long, default_value_t = false)]
    pub no_upper: bool,
    #[arg(long, default_value_t = false)]
    pub no_lower: bool,
    #[arg(long, default_value_t = false)]
    pub no_number: bool,
    #[arg(long, default_value_t = false)]
    pub no_symbol: bool,
    #[arg(short, long, default_value_t = 16)]
    pub length: u8,
}

impl CmdExcutor for GenpassOpts {
    async fn execute(self) -> Result<()> {
        let ret = process_genpass(
            self.no_upper,
            self.no_lower,
            self.no_number,
            self.no_symbol,
            self.length,
        );
        let ret = String::from_utf8(ret)?;
        println!("{}", ret);
        let estimate = zxcvbn(&ret, &[])?;
        eprintln!("estimate: {:?}", estimate.score());
        Ok(())
    }
}
