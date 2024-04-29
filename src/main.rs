use anyhow::Result;
use clap::Parser;

use rcli::{CmdExcutor, Opts};

/// usage:
/// - rcli csv
///     - ```rcli csv --input in.csv --output out.json --format json```
///     - ```rcli csv --input in.csv --output out.yaml --format yaml```
///     - ```rcli csv --header --delimiter , --input in.csv --output out.yaml --format yaml```
/// - rcli genpass
///     - ```rcli genpass -l 32 --no-lower --no-lower --no-symbol --no-number```
/// - rcli base64
///     - ```rcli base64 encode --format nopadding/standard/urlsafe --input textfile```
///     - ```rcli base64 decode --format nopadding/standard/urlsafe --input textfile```
/// - rcli text
///     - ```rcli text sign --format blake3 --key keyfile --input textfile```
///     - ```rcli text verify --format blake3 --key keyfile --input textfile --sig signature```
///     - ```rcli text generate --format blake3 --output keyfile```
///     - ```rcli text encrypt --key keyfile --input textfile --nonce noncefile```
///     - ```rcli text decrypt --key keyfile --input textfile --nonce noncefile```
/// - rcli http serve(default dir is current dir, default port is 8080)
///     - ```rcli http serve```
///     - ```rcli http serve --dir /tmp --port 8080```
#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let cli = Opts::parse();
    cli.subcmd.execute().await?;
    Ok(())
}
