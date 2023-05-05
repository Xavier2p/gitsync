use clap::Parser;

#[derive(Debug, Parser, Clone)]
#[clap(about, version, author)]
pub struct Args {
    #[clap(forbid_empty_values = true, default_value = "[autocommit]: Work in Progress")]
    /// The message of the commit.
    pub message: String,

    #[clap(short, long)]
    /// The tag of the commit.
    pub tag: Option<String>,

    #[clap(short, long)]
    /// If used, the commit won't be pushed.
    pub local: bool,

    #[clap(short, long)]
    /// Enable verbose mode.
    pub verbose: bool,

    #[clap(short, long)]
    /// Add only the updated files.
    pub updated: bool,
}