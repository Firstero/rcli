mod cli;
mod process;
use clap::Parser;

use cli::{Base64SubCommand, Opts, SubCommand};
use process::{process_b64decode, process_b64encode, process_csv, process_genpass};

// usage:
// 使用 rcli -- csv --input input.csv --output output.csv --format json
// 使用 rcli -- genpass 进行密码生成
// 使用 rcli -- base64 --encode/--decode --format 进行base64编码/解码
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
        // Todo: implement base64 subcommand
        SubCommand::Base64(base64_opts) => match base64_opts.subcmd {
            Base64SubCommand::Encode(opts) => {
                process_b64encode(&opts.input, opts.format)?;
            }
            Base64SubCommand::Decode(opts) => {
                process_b64decode(&opts.input, opts.format)?;
            }
        },
    }
    Ok(())
}
