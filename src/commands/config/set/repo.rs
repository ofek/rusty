use anyhow::Result;
use clap::Args;

use crate::{app, config, platform};

/// Set the path to the repository
#[derive(Args, Debug)]
#[command()]
pub struct Cli {
    path: String,
}

impl Cli {
    pub fn exec(&self) -> Result<()> {
        let path = platform::canonicalize_path(&self.path);
        debug!("Setting repository path to: {}", &path);

        let mut config = app::config().clone();
        config.repo = path;
        config::save(config)?;

        Ok(())
    }
}
