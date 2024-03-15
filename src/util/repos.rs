//!
//! # Repo handler
//!
//! This module finds all repositories under specified directory

use anyhow::{Context, Result};
use std::path::PathBuf;

/// Type to handle found repositories
#[derive(Debug)]
pub(crate) struct Repos {
    pub(crate) repos: Vec<Dir>,
}

/// Type to handle found directories
#[derive(Debug)]
pub(crate) struct Dir {
    pub(crate) name: String,
    pub(crate) path: PathBuf,
}

impl Repos {
    /// Function to find all repositories in the specified directory and return the results
    pub(crate) fn get_repos(path: PathBuf) -> Result<Repos> {
        let child_directories = path
            .read_dir()
            .with_context(|| format!("Couldn't get the child directories of {:?}", &path))?;

        let mut status = Vec::new();
        for dir in child_directories {
            let dir = dir.context("Child directory has an error")?;
            let mut repo = PathBuf::new();
            repo.push(dir.path());
            repo.push(".git");
            if repo.is_dir() {
                status.push(Dir {
                    name: dir.file_name().to_string_lossy().to_string(),
                    path: dir.path(),
                });
            }
        }
        Ok(Repos { repos: status })
    }
}
