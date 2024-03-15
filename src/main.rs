//!
//! # rigit
//!
//! A CLI tool which let's you perform a git action on multiple repositories in one directory

pub(crate) mod commands;
pub(crate) mod util;

use crate::commands::cli::{Cli, Command};
use crate::commands::fetch::run_fetch;
use crate::commands::status::run_status;
use crate::util::repos::Repos;
use anyhow::Result;
use clap::Parser;
use std::path::Path;

fn main() -> Result<()> {
    let args = Cli::parse();

    match args.command {
        Command::Status { path, verbose } => {
            let path = Path::new(&path).canonicalize()?;
            let repos = Repos::get_repos(path)?;
            run_status(repos, verbose)?;
        }
        Command::Fetch { path } => {
            let path = Path::new(&path).canonicalize()?;
            let repos = Repos::get_repos(path)?;
            run_fetch(repos);
        }
    }
    Ok(())
}
