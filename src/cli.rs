use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "rigit")]
#[command(author = "Weiberle17 <maxweiberle63@gmail.com>")]
#[command(version)]
#[command(about = "Cli-tool which lets you perform a git action on multiple repositories", long_about = None)]
pub struct Cli {
  #[command(subcommand)]
  pub command: Command,
}

// FIX: check path variable for availability
#[derive(Subcommand, Debug)]
pub enum Command {
  /// Run git status on all repos in the following directory
  Status {
    path: String,
    #[clap(short, long)]
    verbose: bool,
  },
  /// Run git fetch on all repos in the following directory
  Fetch {
    path: String,
  }
}
