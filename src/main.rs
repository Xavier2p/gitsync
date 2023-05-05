//! > **A tool to earn time at each `git` actions.**
//!
//! # Description
//! A simple tool to use `git` with ease.
//!
//! Written in Rust.
//!
//! /!\ This project is a test for me to learn Rust.
//!
//! # Usage
//! ```bash
//! gsync [OPTIONS] [MESSAGE]
//! ```
//!
//! # Arguments
//! * `<MESSAGE>`    - The message of the commit
//!
//! # Options
//! * `-h`, `--help`         - Print help information
//! * `-l`, `--local`        - If used, the commit won't be pushed
//! * `-t`, `--tag <TAG>`    - The tag of the commit
//! * `-u`, `--updated`      - Add only the updated files
//! * `-v`, `--verbose`      - Enable verbose mode
//! * `-V`, `--version`      - Print version information
//!
//! # Examples
//! ```bash
//! gsync "My commit message"
//! gsync -uvlt v1.0.0 "My commit message"
//! ```
//!
//! # Installation
//! ```bash
//! git clone https://github.com/Xavier2p/gitsync.git && cd gitsync
//! cargo build --release
//! ```
//!
//! # Add to PATH
//! ```bash
//! sudo cp target/release/gsync /usr/local/bin
//! ```
//!
//! # License
//! This project is under the MIT License.
//!
//! # Author
//! **gsync** © [Xavier2p](https://github.com/Xavier2p)
//!
//! Authored and maintained by Xavier2p.
//!
//! >Fun fact, this project is deployed with `gsync` itself.

mod args;
mod git;
use clap::Parser;
use std::process;

/// The main function.
fn main() {
    let args: args::Args = args::Args::parse();
    let mut has_tag: bool = false;

    if args.verbose {
        git::status();
    }

    git::add(args.updated);

    if args.verbose {
        git::status();
    }

    git::commit(&args.message);

    if args.tag.is_some() {
        has_tag = true;
        git::tag(&args.tag.unwrap());
    }

    if !args.local {
        if has_tag {
            git::push_tag();
        }
        git::push();
    }

    if args.verbose {
        git::log();
    }

    process::exit(0);
}
