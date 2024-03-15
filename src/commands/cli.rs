//!
//! # CLI argument handler
//!
//! This module handles the CLI arguments using clap

use clap::{crate_version, value_parser, Arg, ArgAction, Command};
use clap_complete::{generate, Generator, Shell};
use std::io;

/// Function to create the CLI structure using clap
pub(crate) fn build_cli() -> Command {
    Command::new("rigit")
        .name("rigit")
        .author("Væñgír, <vaengir@gmx.de>")
        .version(crate_version!())
        .about("A CLI tool which lets you perform a git action on multiple repositories in one directory")
        .arg_required_else_help(true)
        .subcommand(Command::new("status")
            .about("Run git status on all repos in the following directory")
            .arg_required_else_help(true)
            .arg(Arg::new("path")
                .required(true))
            .arg(Arg::new("verbose")
                .short('v')
                .long("verbose")
                .help("Use verbose output")
                .action(ArgAction::SetTrue)))
        .subcommand(Command::new("fetch")
            .about("Run git fetch on all repos in the following directory")
            .arg_required_else_help(true)
            .arg(Arg::new("path")
                .required(true)))
        .arg(
            Arg::new("generator")
                .long("generate")
                .action(ArgAction::Set)
                .value_parser(value_parser!(Shell)),
        )
}

/// Function to create output for bash completion
pub(crate) fn print_completions<G: Generator>(gen: G, cmd: &mut Command) {
    println!("-----------------------------------------------------------------------------------------------------");
    generate(gen, cmd, cmd.get_name().to_string(), &mut io::stdout());
    println!("-----------------------------------------------------------------------------------------------------");
    println!("Copy everything between the lines into the corresponding dir for the shell you use.");
}
