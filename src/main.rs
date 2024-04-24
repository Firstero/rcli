use anyhow::Result;
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
use clap::Parser;
use zxcvbn::zxcvbn;

use rcli::{
    get_content, get_reader, process_b64decode, process_b64encode, process_csv, process_genpass,
    process_sign, process_verify, Base64SubCommand, Opts, SubCommand, TextSubCommand,
};

// usage:
// 使用 rcli -- csv --input input.csv --output output.csv --format json 进行csv转换
// 使用 rcli -- genpass -l --no-lower --no-lower --no-symbol 进行密码生成
// 使用 rcli -- base64 --encode/--decode --format nopadding/urlsafe/standard 进行base64编码/解码
// 使用 rcli -- text --sign/--verify --format blake3/ed25519 进行文本签名/验证
fn main() -> Result<()> {
    let cli = Opts::parse();
    match cli.subcmd {
        SubCommand::Csv(opts) => {
            let output = opts
                .output
                .unwrap_or_else(|| format!("output.{}", opts.format));
            process_csv(&opts.input, output, opts.format)?;
        }
        SubCommand::Genpass(opts) => {
            let ret = process_genpass(
                opts.no_upper,
                opts.no_lower,
                opts.no_number,
                opts.no_symbol,
                opts.length,
            );
            let ret = String::from_utf8(ret)?;
            println!("{:?}", ret);
            let estimate = zxcvbn(&ret, &[])?;
            eprintln!("estimate: {:?}", estimate.score());
        }
        SubCommand::Base64(base64_opts) => match base64_opts.subcmd {
            Base64SubCommand::Encode(opts) => {
                let ret = process_b64encode(&opts.input, opts.format)?;
                println!("{}", ret);
            }
            Base64SubCommand::Decode(opts) => {
                let ret = process_b64decode(&opts.input, opts.format)?;
                println!("{}", ret);
            }
        },
        // Todo: implement text subcommand
        SubCommand::Text(text_opts) => match text_opts.subcmd {
            TextSubCommand::Sign(opts) => {
                let mut reader = get_reader(&opts.input)?;
                let key = get_content(&opts.key)?;
                let signed = process_sign(&mut reader, &key, opts.format)?;
                let encode = URL_SAFE_NO_PAD.encode(signed);
                println!("{}", encode);
            }
            TextSubCommand::Verify(opts) => {
                let mut reader = get_reader(&opts.input)?;
                let key = get_content(&opts.key)?;
                let sig = URL_SAFE_NO_PAD.decode(&opts.sig)?;
                let verified = process_verify(&mut reader, &key, &sig, opts.format)?;
                if verified {
                    println!("✓ Signature verified");
                } else {
                    println!("⚠ Signature not verified");
                }
            }
        },
    }
    Ok(())
}
