use clap::Parser;
use std::{fmt, fs, str::FromStr};

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
}

#[derive(Debug, Parser)]
pub struct CsvOpts {
    #[arg(long, default_value_t = true)]
    pub header: bool,
    #[arg(short, long, default_value_t = ',')]
    pub delimiter: char,
    #[arg(short, long, required = true, value_parser(verify_file_exists))]
    pub input: String,
    #[arg(short, long)] // default_value_t = "output.json".into()
    pub output: Option<String>,
    #[arg(long, default_value = "json", value_parser=OutputFormat::from_str)]
    pub format: OutputFormat,
}

#[derive(Debug, Parser)]
pub struct GenpassOpts {
    #[arg(long, default_value_t = false)]
    pub no_upper: bool,
    #[arg(long, default_value_t = false)]
    pub no_lower: bool,
    #[arg(long, default_value_t = false)]
    pub no_number: bool,
    #[arg(long, default_value_t = false)]
    pub no_symbol: bool,
    #[arg(short, long, default_value_t = 16)]
    pub length: u8,
}

#[derive(Debug, Clone, Copy)]
pub enum OutputFormat {
    Json,
    Yaml,
}

fn verify_file_exists(path: &str) -> Result<String, String> {
    if fs::metadata(path).is_ok() {
        Ok(path.to_string())
    } else {
        Err(format!("file not found: {}", path))
    }
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

impl fmt::Display for OutputFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Into::<&str>::into(*self))
    }
}
