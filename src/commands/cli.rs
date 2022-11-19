use anyhow::Result;
use clap::{Parser, Subcommand};
use clap_verbosity_flag::{InfoLevel, Verbosity};

/// Rusty example app
#[derive(Parser, Debug)]
#[command(version, bin_name = "rusty", disable_help_subcommand = true)]
pub struct Cli {
    #[clap(flatten)]
    pub verbose: Verbosity<InfoLevel>,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Complete(super::complete::Cli),
    Config(super::config::cli::Cli),
    Exec(super::exec::Cli),
    Test(super::test::Cli),
}

impl Cli {
    pub fn exec(&self) -> Result<()> {
        match &self.command {
            Commands::Complete(cli) => cli.exec(),
            Commands::Config(cli) => cli.exec(),
            Commands::Exec(cli) => cli.exec(),
            Commands::Test(cli) => cli.exec(),
        }
    }
}
