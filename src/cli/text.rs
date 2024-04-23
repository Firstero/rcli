use clap::Parser;
use std::{fmt::Display, str::FromStr};

use super::parse_input_file;
#[derive(Debug, Parser)]
pub struct TextOpts {
    #[command(subcommand)]
    pub subcmd: TextSubCommand,
}

#[derive(Debug, Parser)]
pub enum TextSubCommand {
    #[command(name = "sign", about = "Sign text with private/shared key.")]
    Sign(TextSignOpts),
    #[command(name = "verify", about = "Verify text with public/shared key.")]
    Verify(TextVerifyOpts),
}

#[derive(Debug, Parser)]
pub struct TextSignOpts {
    #[arg(short, long, value_parser=parse_input_file, default_value="-", help = "input file path, or '-' for stdin")]
    pub input: String,
    #[arg(short, long, value_parser=parse_input_file, help = "key file path, or '-' for stdin")]
    pub key: String,
    #[arg(long, value_parser=TextSignFormat::from_str, default_value="blake3", help = "key file path, or '-' for stdin")]
    pub format: TextSignFormat,
}

#[derive(Debug, Parser)]
pub struct TextVerifyOpts {
    #[arg(short, long, default_value = "-", value_parser=parse_input_file, help = "input file path, or '-' for stdin")]
    pub input: String,
    #[arg(short, long, value_parser=parse_input_file, help = "key file path, or '-' for stdin")]
    pub key: String,
    #[arg(short, long, required = true, help = "signature")]
    pub sig: String,
    #[arg(long, value_parser=TextSignFormat::from_str, default_value="blake3", help = "key file path, or '-' for stdin")]
    pub format: TextSignFormat,
}

#[derive(Debug, Clone, Copy)]
pub enum TextSignFormat {
    Blake3,
    Ed25519,
}

impl FromStr for TextSignFormat {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "blake3" => Ok(TextSignFormat::Blake3),
            "ed25519" => Ok(TextSignFormat::Ed25519),
            v => Err(anyhow::anyhow!("Invalid TextSignFormat: {}", v)),
        }
    }
}

impl From<TextSignFormat> for &'static str {
    fn from(f: TextSignFormat) -> Self {
        match f {
            TextSignFormat::Blake3 => "Blake3",
            TextSignFormat::Ed25519 => "Ed25519",
        }
    }
}

impl Display for TextSignFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Into::<&str>::into(*self))
    }
}
