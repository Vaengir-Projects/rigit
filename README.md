[![Crates.io](https://img.shields.io/crates/v/faktory.svg)](https://crates.io/crates/rigit)
[![Documentation](https://docs.rs/faktory/badge.svg)](https://docs.rs/rigit/)
[![Codecov](https://codecov.io/github/jonhoo/faktory-rs/coverage.svg?branch=master)](https://app.codecov.io/gh/Vaengir-Projects/rigit)
[![dependency status](https://deps.rs/repo/github/Vaengir-Projects/rigit/status.svg)](https://deps.rs/repo/github/Vaengir-Projects/rigit)

# rigit
Tool which let's you perform a git actions on multiple repositories.

## Installation
```bash
cargo install rigit
```

## Usage
```
rigit [OPTIONS] [COMMAND]

Commands:
  status  Run git status on all repos in the following directory
      -v, --verbose  Print complete git status
  fetch   Run git fetch on all repos in the following directory
  help    Print this message or the help of the given subcommand(s)

Options:
  -G, --generate <generator>  [possible values: bash, elvish, fish, powershell, zsh]
  -h, --help                  Print help
  -V, --version               Print version
```

### WIP (Could take some time because the implemented commands are the main ones I wanted this tool for)
- pull: pull the remote changes of all repositories under the specified directory and merge/rebase them.

### Planned features
- push: push the local changes of all repositories under the specified directory to the remote repositories.
