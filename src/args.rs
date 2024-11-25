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
use clap::{Parser, Subcommand};

/// The arguments parser.
/// ## Example
/// ```
/// use args::Args;
/// let args: Args = Args::parse();
/// ```
#[derive(Parser)]
#[command(about, version, author)]
#[command(arg_required_else_help = true, subcommand_required = true)]
pub struct Args {
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

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Changes that affect the build system or external dependencies
    #[command(arg_required_else_help = true)]
    Build {
        /// Message used as description for the commit.
        #[arg(value_parser = check_message)]
        message: String,

        /// Scope
        #[arg(short, long)]
        scope: Option<String>,
    },
    /// Updating grunt tasks etc; no production code change
    #[command(arg_required_else_help = true)]
    Chore {
        /// Message used as description for the commit.
        #[arg(value_parser = check_message)]
        message: String,

        /// Scope
        #[arg(short, long)]
        scope: Option<String>,
    },
    /// Changes on the CI/CD pipeline or other DevOps tools
    #[command(arg_required_else_help = true)]
    Ci {
        /// Message used as description for the commit.
        #[arg(value_parser = check_message)]
        message: String,

        /// Scope
        #[arg(short, long)]
        scope: Option<String>,
    },
    /// Changes to the documentation
    #[command(arg_required_else_help = true)]
    Docs {
        /// Message used as description for the commit.
        #[arg(value_parser = check_message)]
        message: String,

        /// Scope
        #[arg(short, long)]
        scope: Option<String>,
    },
    /// New feature for the user, not a new feature for build script
    #[command(arg_required_else_help = true)]
    Feat {
        /// Message used as description for the commit.
        #[arg(value_parser = check_message)]
        message: String,

        /// Scope
        #[arg(short, long)]
        scope: Option<String>,
    },
    /// Bug fix for the user, not a fix to a build script
    #[command(arg_required_else_help = true)]
    Fix {
        /// Message used as description for the commit.
        #[arg(value_parser = check_message)]
        message: String,

        /// Scope
        #[arg(short, long)]
        scope: Option<String>,
    },
    /// A new performance improvement
    #[command(arg_required_else_help = true)]
    Perf {
        /// Message used as description for the commit.
        #[arg(value_parser = check_message)]
        message: String,

        /// Scope
        #[arg(short, long)]
        scope: Option<String>,
    },
    /// Refactoring production code, eg. renaming a variable
    #[command(arg_required_else_help = true)]
    Refactor {
        /// Message used as description for the commit.
        #[arg(value_parser = check_message)]
        message: String,

        /// Scope
        #[arg(short, long)]
        scope: Option<String>,
    },
    /// Formatting, missing semi colons, etc; no production code change
    #[command(arg_required_else_help = true)]
    Style {
        /// Message used as description for the commit.
        #[arg(value_parser = check_message)]
        message: String,

        /// Scope
        #[arg(short, long)]
        scope: Option<String>,
    },
    /// Adding missing tests, refactoring tests; no production code change
    #[command(arg_required_else_help = true)]
    Test {
        /// Message used as description for the commit.
        #[arg(value_parser = check_message)]
        message: String,

        /// Scope
        #[arg(short, long)]
        scope: Option<String>,
    },
}

fn check_message(input: &str) -> Result<String, String> {
    if input.is_empty() {
        Err("The message can't be empty.".to_string())
    } else {
        Ok(input.to_string())
    }
}
