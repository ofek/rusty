use anyhow::Result;
use clap::Args;
use std::process::{exit, Command};

/// Execute an arbitrary command
///
/// All arguments are passed through unless --help is first
#[derive(Args, Debug)]
#[command()]
pub struct Cli {
    #[arg(required = true, trailing_var_arg = true, allow_hyphen_values = true)]
    args: Vec<String>,
}

impl Cli {
    pub fn exec(&self) -> Result<()> {
        let mut command = Command::new(&self.args[0]);
        if self.args.len() > 1 {
            command.args(&self.args[1..]);
        }

        let status = command.status()?;
        exit(status.code().unwrap_or(1));
    }
}
