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
#[clap(about, version, author)]
pub struct Args {
    #[clap(
        forbid_empty_values = true,
        default_value = "[autocommit]: Work in Progress"
    )]
    /// Message to assign at the commit.
    pub message: String,

    #[clap(short, long)]
    /// Tag to assign at the commit.
    pub tag: Option<String>,

    #[clap(short, long)]
    /// If used, the commit won't be pushed, only committed.
    pub local: bool,

    #[clap(short, long)]
    /// Enables verbose mode.
    pub verbose: bool,

    #[clap(short, long)]
    /// Add only the updated files.
    pub updated: bool,
}
