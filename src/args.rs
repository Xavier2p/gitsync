//! # Args Module
//!
//! This module contains the arguments parser.
//!
//! I use `clap` for parsing the arguments.
//!
//! ## Example
//! ```
//! use args::Args;
//! let args: Args = Args::parse();
//! ```
use clap::Parser;

/// The arguments parser.
/// ## Example
/// ```
/// use args::Args;
/// let args: Args = Args::parse();
/// ```
#[derive(Parser)]
#[command(about, version, author)]
pub struct Args {
    #[arg(default_value = "[gsync]: Work in Progress")]
    /// Message to assign to the commit.
    pub message: String,

    #[arg(short, long)]
    /// Tag to assign at the commit.
    pub tag: Option<String>,

    #[arg(short, long)]
    /// If used, the commit won't be pushed, only committed.
    pub local: bool,

    #[arg(short, long)]
    /// Enable verbose mode.
    pub verbose: bool,

    #[arg(short, long)]
    /// Add only the updated files.
    pub updated: bool,

    #[arg(short, long)]
    /// Sign the commit.
    pub sign: bool,
}
