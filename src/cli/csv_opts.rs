use crate::{parse_input_file, process_csv, CmdExcutor};

use anyhow::Result;
use clap::Parser;
use std::{fmt::Display, str::FromStr};

#[derive(Debug, Parser)]
pub struct CsvOpts {
    #[arg(long, default_value_t = true)]
    pub header: bool,
    #[arg(short, long, default_value_t = ',')]
    pub delimiter: char,
    #[arg(short, long, required = true, value_parser=parse_input_file)]
    pub input: String,
    #[arg(short, long)]
    pub output: Option<String>,
    #[arg(long, default_value = "json", value_parser=OutputFormat::from_str)]
    pub format: OutputFormat,
}

#[derive(Debug, Clone, Copy)]
pub enum OutputFormat {
    Json,
    Yaml,
}

impl FromStr for OutputFormat {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "json" => Ok(OutputFormat::Json),
            "yaml" => Ok(OutputFormat::Yaml),
            v => Err(anyhow::anyhow!("invalid output format: {}", v)),
        }
    }
}

impl From<OutputFormat> for &'static str {
    fn from(f: OutputFormat) -> Self {
        match f {
            OutputFormat::Json => "json",
            OutputFormat::Yaml => "yaml",
        }
    }
}

impl Display for OutputFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Into::<&str>::into(*self))
    }
}

impl CmdExcutor for CsvOpts {
    async fn execute(self) -> Result<()> {
        let output = self
            .output
            .unwrap_or_else(|| format!("output.{}", self.format));
        process_csv(&self.input, output, self.format)
    }
}
