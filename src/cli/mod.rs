mod base64_opts;
mod csv_opts;
mod genpass_opts;
mod http_serve;
mod text;

use anyhow::Result;
use enum_dispatch::enum_dispatch;
use std::fs;
use std::path::{Path, PathBuf};

pub use self::base64_opts::{Base64Format, Base64Opts};
pub use self::csv_opts::{CsvOpts, OutputFormat};
pub use self::genpass_opts::GenpassOpts;
pub use self::http_serve::{HttpOpts, HttpServeOpts, HttpSubCommand};
pub use self::text::{
    TextDecryptOpts, TextEncryptOpts, TextKeyGenerateOpts, TextOpts, TextSignFormat, TextSignOpts,
    TextSubCommand, TextVerifyOpts,
};

use clap::Parser;

#[derive(Debug, Parser)]
#[command(name = "rcli", version="0.1.0", about="A Rust Command Line Tool", author="Firstero", long_about=None)]
pub struct Opts {
    #[command(subcommand)]
    pub subcmd: SubCommand,
}

#[derive(Debug, Parser)]
#[enum_dispatch(CmdExcutor)]
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
    // rcli text sign/verify --input --key --format
    #[command(name = "text", about = "text sign/verify")]
    Text(TextOpts),
    // rcli http serve . --port 8080
    #[command(name = "http", about = "http server")]
    HttpServe(HttpOpts),
}

pub fn parse_input_file(path: &str) -> Result<String, String> {
    if path == "-" || fs::metadata(path).is_ok() {
        Ok(path.to_string())
    } else {
        Err(format!("file not found: {}", path))
    }
}

pub fn verify_dir(path: &str) -> Result<PathBuf, &'static str> {
    let path = Path::new(path);
    if path.exists() && path.is_dir() {
        Ok(path.into())
    } else {
        Err("directory not found")
    }
}

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
