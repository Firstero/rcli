mod cli;
mod process;
mod utils;

use anyhow::Result;
pub use cli::{
    parse_input_file, verify_dir, Base64SubCommand, HttpSubCommand, Opts, SubCommand,
    TextSignFormat, TextSubCommand,
};
pub use process::{
    process_b64decode, process_b64encode, process_csv, process_decrypt, process_encrypt,
    process_generate, process_genpass, process_http_serve, process_sign, process_verify,
};
pub use utils::{get_content, get_reader};

#[allow(async_fn_in_trait)]
pub trait CmdExcutor {
    async fn execute(self) -> Result<()>;
}
