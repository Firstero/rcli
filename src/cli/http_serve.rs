use clap::Parser;
use enum_dispatch::enum_dispatch;
use std::path::PathBuf;

use crate::{process_http_serve, CmdExcutor};
use anyhow::Result;

#[derive(Debug, Parser)]
pub struct HttpOpts {
    #[command(subcommand)]
    pub subcmd: HttpSubCommand,
}

#[derive(Debug, Parser)]
#[enum_dispatch(CmdExcutor)]
pub enum HttpSubCommand {
    #[command(name = "serve", about = "Serve a directory via HTTP")]
    Serve(HttpServeOpts),
}

#[derive(Debug, Parser)]
pub struct HttpServeOpts {
    #[arg(short, long, default_value = ".", help = "serve directory")]
    pub dir: PathBuf,
    #[arg(short, long, default_value_t = 8080, help = "serve port")]
    pub port: u16,
}

impl CmdExcutor for HttpServeOpts {
    async fn execute(self) -> Result<()> {
        process_http_serve(self.dir, self.port).await
    }
}

impl CmdExcutor for HttpOpts {
    async fn execute(self) -> Result<()> {
        self.subcmd.execute().await
    }
}
