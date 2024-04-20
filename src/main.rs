// usage:
// 使用 rcli 进行 csv 的处理, show 或者 转换输出不同的 formats
mod opts;
mod process;

use clap::Parser;
use opts::{Opts, SubCommand};
use process::process_csv;

fn main() -> anyhow::Result<()> {
    let cli = Opts::parse();
    match cli.subcmd {
        SubCommand::Csv(opts) => {
            let output = opts
                .output
                .unwrap_or_else(|| format!("output.{}", opts.format));
            process_csv(&opts.input, output, opts.format)?;
        }
        // Todo implement genpass subcommand
        SubCommand::Genpass(_) => unimplemented!("genpass not implemented"),
    }
    Ok(())
}
