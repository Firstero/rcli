use clap::Parser;
use std::fs;

#[derive(Debug, Parser)]
#[command(name = "rcli", version, about, author, long_about=None)]
pub struct Opts {
    #[command(subcommand)]
    pub subcmd: SubCommand,
}

#[derive(Debug, Parser)]
pub enum SubCommand {
    #[command(name = "csv", about = "csv file processor")]
    Csv(CsvOpts),
}

#[derive(Debug, Parser)]
pub struct CsvOpts {
    #[arg(long, default_value_t = true)]
    pub header: bool,
    #[arg(short, long, default_value_t = ',')]
    pub delimiter: char,
    #[arg(short, long, required = true, value_parser(verify_file_exists))]
    pub input: String,
    #[arg(short, long, default_value = "output.json")] // default_value_t = "output.json".into()
    pub output: String,
}

fn verify_file_exists(path: &str) -> Result<String, String> {
    if fs::metadata(path).is_ok() {
        Ok(path.to_string())
    } else {
        Err(format!("file not found: {}", path))
    }
}
