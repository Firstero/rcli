mod base64_opts;
mod csv_opts;
mod genpass_opts;
mod text;

use std::fs;

pub use self::base64_opts::{Base64Format, Base64Opts, Base64SubCommand};
pub use self::csv_opts::{CsvOpts, OutputFormat};
pub use self::genpass_opts::GenpassOpts;
pub use self::text::{TextOpts, TextSignFormat, TextSubCommand};
use clap::Parser;

#[derive(Debug, Parser)]
#[command(name = "rcli", version, about, author, long_about=None)]
pub struct Opts {
    #[command(subcommand)]
    pub subcmd: SubCommand,
}

#[derive(Debug, Parser)]
pub enum SubCommand {
    // rcli csv --header xx -delimiter , -input /tmp/1.csv -output output.json
    #[command(name = "csv", about = "csv file processor")]
    Csv(CsvOpts),
    // rcli genpass --upper xx --lower --symbol --number --length
    #[command(name = "genpass", about = "generate password")]
    Genpass(GenpassOpts),
    // rcli base64 --encode/decode --output
    #[command(name = "base64", about = "base64 encode/decode")]
    Base64(Base64Opts),
    #[command(name = "text", about = "text sign/verify")]
    Text(TextOpts),
}

// 模块级别的函数，共享input file的解析逻辑
fn parse_input_file(path: &str) -> Result<String, String> {
    if path == "-" || fs::metadata(path).is_ok() {
        Ok(path.to_string())
    } else {
        Err(format!("file not found: {}", path))
    }
}

// 单元测试
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input_file() {
        assert_eq!(parse_input_file("-"), Ok("-".to_string()));
        assert_eq!(parse_input_file("*"), Err("file not found: *".to_string()));
        assert_eq!(parse_input_file("Cargo.toml"), Ok("Cargo.toml".to_string()));
    }
}