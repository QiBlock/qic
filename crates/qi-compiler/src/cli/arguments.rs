use std::collections::BTreeSet;
use std::path::Path;
use std::path::PathBuf;

use clap::Parser;
use path_slash::PathExt;

#[derive(Debug, Parser)]
#[command(name = "The Qi compiler", arg_required_else_help = true)]
pub struct Arguments {
    /// Print the version and exit.
    #[arg(long = "version")]
    pub version: bool,
}

impl Arguments {
    /// Validate the arguments.
    pub fn validate(&self) -> anyhow::Result<()> {
        if self.version && std::env::args().count() > 2 {
            anyhow::bail!("No other options are allowed while getting the compiler version.");
        }

        Ok(())
    }
}
