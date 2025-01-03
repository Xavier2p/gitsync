//! *A tool to earn time at each `git` actions.*
//!
//! ## Description
//!
//! A simple tool to use `git` with ease. At each call, the tool performs:
//!
//! ```console
//! $ git add -A
//! # or --updated if you use the `-u` option
//!
//! $ git commit -m "<MESSAGE>"
//! # we can add the option `-s` to use commit signature
//!
//! $ git tag -a <TAG_NAME> -m "<MESSAGE>"
//! # if you use `-t <NAME>`
//!
//! $ git push
//! # only if you don't use `-l`, follows tags if selected
//! ```
//!
//! ## Usage
//!
//! ```console
//! $ gsync help
//! A simple tool to use `git` with ease.
//! Written in Rust.
//!
//! Usage: gsync [OPTIONS] <COMMAND>
//!
//! Commands:
//!   build     Changes that affect the build system or external dependencies
//!   chore     Updating grunt tasks etc; no production code change
//!   ci        Changes on the CI/CD pipeline or other DevOps tools
//!   docs      Changes to the documentation
//!   feat      New feature for the user, not a new feature for build script
//!   fix       Bug fix for the user, not a fix to a build script
//!   perf      A new performance improvement
//!   refactor  Refactoring production code, eg. renaming a variable
//!   style     Formatting, missing semi colons, etc; no production code change
//!   test      Adding missing tests, refactoring tests; no production code change
//!   help      Print this message or the help of the given subcommand(s)
//!
//! Options:
//!   -t, --tag <TAG>  Tag to assign at the commit
//!   -l, --local      If used, the commit won't be pushed, only committed
//!   -v, --verbose    Enable verbose mode
//!   -u, --updated    Add only the updated files
//!   -s, --sign       Sign the commit
//!   -h, --help       Print help
//!   -V, --version    Print version
//! ```
//!
//! ## Examples
//!
//! ```bash
//! gsync "My commit message"
//! gsync -uvlt v1.0.0 "My commit message"
//! ```
//!
//! # Installation
//! ```bash
//! git clone https://github.com/Xavier2p/gitsync.git && cd gitsync
//! cargo install --path .
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
//! *Fun fact, this project is deployed with `gsync` itself.*

mod args;
mod git;
use args::Commands;
use clap::Parser;
use std::process;

/// Parse the command and return the commit message.
///
/// ## Arguments
/// * `command` - The command to parse.
///
/// ## Example
/// ```
/// use main::parse_command;
/// let message: String = parse_command(Commands::Feat { message: "My commit message".to_string(),
/// scope: Some("scope".to_string()) });
/// assert_eq!(message, "feat(scope): My commit message");
/// ```
///
/// ## Returns
/// The commit message.
fn parse_command(command: args::Commands) -> String {
    match command {
        Commands::Build { message, scope } => {
            create_commit_message("build".to_string(), message, scope)
        }
        Commands::Ci { message, scope } => create_commit_message("ci".to_string(), message, scope),
        Commands::Chore { message, scope } => {
            create_commit_message("chore".to_string(), message, scope)
        }
        Commands::Docs { message, scope } => {
            create_commit_message("docs".to_string(), message, scope)
        }
        Commands::Feat { message, scope } => {
            create_commit_message("feat".to_string(), message, scope)
        }
        Commands::Fix { message, scope } => {
            create_commit_message("fix".to_string(), message, scope)
        }
        Commands::Perf { message, scope } => {
            create_commit_message("perf".to_string(), message, scope)
        }
        Commands::Refactor { message, scope } => {
            create_commit_message("refactor".to_string(), message, scope)
        }
        Commands::Style { message, scope } => {
            create_commit_message("style".to_string(), message, scope)
        }
        Commands::Test { message, scope } => {
            create_commit_message("test".to_string(), message, scope)
        }
    }
}

/// Create a commit message with the given type, message and scope.
///
/// ## Arguments
/// * `ctype` - The type of the commit.
/// * `message` - The message of the commit.
/// * `scope` - The scope of the commit.
///
/// ## Example
/// ```
/// use main::create_commit_message;
/// let message: String = create_commit_message("feat".to_string(), "My commit message".to_string(), Some("scope".to_string()));
/// assert_eq!(message, "feat(scope): My commit message");
/// ```
/// ## Returns
/// The commit message.
fn create_commit_message(ctype: String, message: String, scope: Option<String>) -> String {
    if scope.is_some() {
        format!("{}({}): {}", ctype, scope.unwrap(), message)
    } else {
        format!("{}: {}", ctype, message)
    }
}

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

    let commit_message: String = parse_command(args.command);

    if args.verbose {
        println!("Commit message: {}", commit_message);
    }

    git::commit(&commit_message, args.sign);

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
