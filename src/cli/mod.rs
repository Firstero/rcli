mod base64_opts;
mod csv_opts;
mod genpass_opts;
mod http_serve;
mod jwt;
mod text;

use anyhow::Result;
use enum_dispatch::enum_dispatch;
use regex::Regex;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::Duration;

pub use self::base64_opts::{
    Base64DecodeOpts, Base64EncodeOpts, Base64Format, Base64Opts, Base64SubCommand,
};
pub use self::csv_opts::{CsvOpts, OutputFormat};
pub use self::genpass_opts::GenpassOpts;
pub use self::http_serve::{HttpOpts, HttpServeOpts, HttpSubCommand};
pub use self::jwt::{JwtOpts, JwtSignOpts, JwtSubCommand, JwtVerifyOpts};
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

    #[command(name = "jwt", about = "jwt token sign/verify")]
    Jwt(JwtOpts),
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

pub fn parse_duration(s: &str) -> Result<u64, &'static str> {
    let re = Regex::new(r"^\d+[smhd]$").unwrap();
    if re.is_match(s) {
        let num = s[..s.len() - 1].parse::<usize>().unwrap();
        let unit = &s[s.len() - 1..];

        let timestamp = match unit {
            "s" => (Duration::from_secs(num as u64)).as_secs(),
            "m" => (Duration::from_secs(num as u64 * 60)).as_secs(),
            "h" => (Duration::from_secs(num as u64 * 60 * 60)).as_secs(),
            "d" => (Duration::from_secs(num as u64 * 60 * 60 * 24)).as_secs(),
            _ => unreachable!(),
        };
        Ok(timestamp)
    } else {
        Err("invalid duration")
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

    #[test]
    fn test_parse_duration() {
        assert_eq!(parse_duration("1s"), Ok(1));
        assert_eq!(parse_duration("1m"), Ok(60));
        assert_eq!(parse_duration("1h"), Ok(60 * 60));
        assert_eq!(parse_duration("1d"), Ok(60 * 60 * 24));
        assert_eq!(parse_duration("1x"), Err("invalid duration"));
    }
}
