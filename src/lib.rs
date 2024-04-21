mod cli;
mod process;
pub use cli::{Base64SubCommand, Opts, SubCommand};
pub use process::{process_b64decode, process_b64encode, process_csv, process_genpass};
