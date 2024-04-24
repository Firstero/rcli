mod cli;
mod process;
mod utils;
pub use cli::{Base64SubCommand, Opts, SubCommand, TextSignFormat, TextSubCommand};
pub use process::{
    process_b64decode, process_b64encode, process_csv, process_genpass, process_sign,
    process_verify,
};
pub use utils::{get_content, get_reader};
