use anyhow::Result;
use clap::Args;

/// Test conditional output
#[derive(Args, Debug)]
#[command(hide = true)]
pub struct Cli {
    text: String,
}

impl Cli {
    pub fn exec(&self) -> Result<()> {
        trace!("trace {}", self.text);
        debug!("debug {}", self.text);
        info!("info {}", self.text);
        success!("success {}", self.text);
        waiting!("waiting {}", self.text);
        warn!("warn {}", self.text);
        error!("error {}", self.text);
        display!("display {}", self.text);
        critical!("critical {}", self.text);

        Ok(())
    }
}
