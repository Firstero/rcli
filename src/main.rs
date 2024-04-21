// usage:
// 使用 rcli -- csv --input input.csv --output output.csv --format json
// 使用 rcli -- genpass 进行密码生成
mod opts;
mod process;

use clap::Parser;
use opts::{Opts, SubCommand};
use process::{process_csv, process_genpass};

fn main() -> anyhow::Result<()> {
    let cli = Opts::parse();
    match cli.subcmd {
        SubCommand::Csv(opts) => {
            let output = opts
                .output
                .unwrap_or_else(|| format!("output.{}", opts.format));
            process_csv(&opts.input, output, opts.format)?;
        }
        SubCommand::Genpass(opts) => {
            process_genpass(
                opts.no_upper,
                opts.no_lower,
                opts.no_number,
                opts.no_symbol,
                opts.length,
            )?;
        }
    }
    Ok(())
}
