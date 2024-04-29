use anyhow::Result;
use clap::Parser;
use std::{fmt::Display, str::FromStr};

use crate::{parse_input_file, process_b64decode, process_b64encode, CmdExcutor};
#[derive(Debug, Parser)]
pub struct Base64Opts {
    #[command(subcommand)]
    pub subcmd: Base64SubCommand,
}

#[derive(Debug, Parser)]
pub enum Base64SubCommand {
    #[command(name = "encode", about = "base64 encode")]
    Encode(Base64EncodeOpts),
    #[command(name = "decode", about = "base64 decode")]
    Decode(Base64DecodeOpts),
}

#[derive(Debug, Parser)]
pub struct Base64EncodeOpts {
    #[arg(short, long, value_parser=parse_input_file, default_value="-", help = "input file path, or '-' for stdin")]
    pub input: String,
    #[arg(long, default_value = "standard", value_parser=Base64Format::from_str, help = "base64 format: [standard, urlsafe, nopadding]")]
    pub format: Base64Format,
}

#[derive(Debug, Parser)]
pub struct Base64DecodeOpts {
    #[arg(short, long, value_parser=parse_input_file, default_value="-", help = "input file path, or '-' for stdin")]
    pub input: String,
    #[arg(long, value_parser=Base64Format::from_str, default_value = "standard", help = "base64 format: [standard, urlsafe, nopadding]")]
    pub format: Base64Format,
}

#[derive(Debug, Clone, Copy)]
pub enum Base64Format {
    Standard,
    UrlSafe,
    NoPadding,
}

impl FromStr for Base64Format {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "standard" => Ok(Base64Format::Standard),
            "urlsafe" => Ok(Base64Format::UrlSafe),
            "nopadding" => Ok(Base64Format::NoPadding),
            v => Err(anyhow::anyhow!("invalid base64 format: {}", v)),
        }
    }
}

impl From<Base64Format> for &'static str {
    fn from(f: Base64Format) -> Self {
        match f {
            Base64Format::Standard => "standard",
            Base64Format::UrlSafe => "urlsafe",
            Base64Format::NoPadding => "nopadding",
        }
    }
}

impl Display for Base64Format {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Into::<&str>::into(*self))
    }
}

impl CmdExcutor for Base64EncodeOpts {
    async fn execute(self) -> Result<()> {
        let ret = process_b64encode(&self.input, self.format)?;
        println!("{}", ret);
        Ok(())
    }
}

impl CmdExcutor for Base64DecodeOpts {
    async fn execute(self) -> Result<()> {
        let ret = process_b64decode(&self.input, self.format)?;
        println!("{}", ret);
        Ok(())
    }
}

impl CmdExcutor for Base64Opts {
    async fn execute(self) -> Result<()> {
        match self.subcmd {
            Base64SubCommand::Encode(opts) => opts.execute().await,
            Base64SubCommand::Decode(opts) => opts.execute().await,
        }
    }
}
