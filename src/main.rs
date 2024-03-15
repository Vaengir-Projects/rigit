//!
//! # rigit
//!
//! A CLI tool which let's you perform a git action on multiple repositories in one directory

pub(crate) mod commands;
pub(crate) mod util;

use crate::commands::cli::print_completions;
use crate::commands::fetch::run_fetch;
use crate::commands::status::run_status;
use crate::util::repos::Repos;
use anyhow::{anyhow, Context, Result};
use clap_complete::Shell;
use commands::cli::build_cli;
use std::path::Path;

fn main() -> Result<()> {
    let args = build_cli().get_matches();

    if let Some(generator) = args.get_one::<Shell>("generator").copied() {
        let mut cmd = build_cli();
        eprintln!("Generating completion file for {generator}...");
        print_completions(generator, &mut cmd);
    }

    match args.subcommand() {
        Some(("status", sub_matches)) => {
            let path = sub_matches.get_one::<String>("path").expect("required");
            let path = Path::new(&path).canonicalize()?;
            let repos = Repos::get_repos(path)?;
            let verbose = sub_matches.get_flag("verbose");
            run_status(repos, verbose)?;
        }
        Some(("fetch", sub_matches)) => {
            let path = sub_matches
                .get_one::<String>("path")
                .ok_or(anyhow!("Couldn't find path arg"))?;
            let path = Path::new(&path)
                .canonicalize()
                .context("Parsing the path")?;
            let repos = Repos::get_repos(path).context("Getting repos in directory")?;
            println!("Fetching repos...");
            run_fetch(repos);
        }
        None => println!("Generated bash completion script"),
        e => unreachable!("Should be unreachable!: {:?}", e),
    }
    Ok(())
}
