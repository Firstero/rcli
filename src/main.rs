use anyhow::Result;
use clap::Parser;

use rcli::{CmdExcutor, Opts};

// usage:
// 使用 rcli -- csv --input input.csv --output output.csv --format json 进行csv转换
// 使用 rcli -- genpass -l 32 --no-lower --no-lower --no-symbol 进行密码生成
// 使用 rcli -- base64 encode/decode --format nopadding/urlsafe/standard 进行base64编码/解码
// 使用 rcli -- text sign/verify/generate --format blake3/ed25519 进行文本签名/验证
// 使用 rcli -- http serve . --port 8080 进行http服务
#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let cli = Opts::parse();
    cli.subcmd.execute().await?;
    Ok(())
}
