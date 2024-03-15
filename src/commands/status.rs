//!
//! # Status argument
//!
//! This module handles the status functionality

use crate::util::repos::{Dir, Repos};
use anyhow::{Context, Result};
use colored::Colorize;
use std::process::Command;

/// Type to handle results of status commands
#[derive(Debug)]
pub(crate) struct Statuses {
    statuses: Vec<Status>,
}

/// Type to handle result of one status
#[derive(Debug)]
pub(crate) struct Status {
    pub(crate) directory: Dir,
    pub(crate) status: Result<Vec<String>, String>,
}

/// Expose this module to be used in main.rs
pub(crate) fn run_status(repos: Repos, verbose: bool) -> Result<()> {
    let status = get_status(repos, verbose)?;
    // dbg!(&status);
    status.print(verbose);
    Ok(())
}

impl Statuses {
    /// Print function to display results clearly
    fn print(&self, verbose: bool) {
        for status in &self.statuses {
            match &status.status {
                Ok(r) => {
                    println!("\n{}: {}", status.directory.name, " ".red());
                    if verbose {
                        let output = r[0].split('\n');
                        for line in output {
                            println!("  {}", line);
                        }
                    } else {
                        for line in r {
                            println!("  {}", line);
                        }
                    }
                }
                Err(_) => {
                    println!("\n{}: {}", status.directory.name, " ".green());
                }
            };
        }
    }
}

/// Function to run status command and return results
fn get_status(repos: Repos, verbose: bool) -> Result<Statuses> {
    let mut statuses: Vec<Status> = Vec::new();
    for dir in repos.repos {
        let status = Command::new("git")
            .arg("status")
            .current_dir(&dir.path)
            .output()
            .with_context(|| format!("Failed to get status of dir: {:?}", dir))?;

        let status_output = String::from_utf8_lossy(&status.stdout).to_string();
        let mut ok_vec: Vec<String> = Vec::new();
        let status: Result<Vec<String>, String>;
        if !status_output.contains("up to date") {
            if verbose {
                ok_vec.push(status_output.clone());
            } else {
                ok_vec.push(
                    "Local repository is not synchronized with the remote repository.".to_string(),
                );
            }
        }
        if status_output.contains("modified") {
            if verbose {
                ok_vec.push(status_output.clone());
            } else {
                ok_vec.push("You have uncommited changes in your local repository.".to_string());
            }
        }
        if status_output.contains("untracked") {
            if verbose {
                ok_vec.push(status_output.clone());
            } else {
                ok_vec.push("You have untracked files in your repository.".to_string());
            }
        }

        if ok_vec.is_empty() {
            if verbose {
                status = Err(status_output);
            } else {
                status = Err("The repository is clean".to_string());
            }
        } else {
            status = Ok(ok_vec);
        }

        statuses.push(Status {
            directory: dir,
            status,
        });
    }

    statuses.sort_by_key(|item| match item.status {
        Ok(_) => 0,
        Err(_) => 1,
    });

    Ok(Statuses { statuses })
}
