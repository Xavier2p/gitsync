//! # Git Module
//!
//! This module contains functions for interacting with git.
//! 
//! The functions are:
//! * `commit` - Creates a commit with the given message.
//! * `tag` - Tag the commit with the given tag.
//! * `push` - Push the changes to the remote.
//! * `add` - Stages the files.
//! * `status` - Gets the status of the repository.
//! * `log` - Gets the 10 last logs of the repository.
//! 
//! ## Example
//! ```
//! use git::{commit, tag, push, add, status, log};
//! commit("My commit message");
//! tag("v1.0.0");
//! push();
//! add(true);
//! status();
//! log();
//! ```
use std::process::Command;

/// Creates a commit with the given message.
/// ## Arguments
/// * `message` - The message of the commit.
/// ## Example
/// ```
/// use git::commit;
/// commit("My commit message");
/// ```
pub fn commit(message: &str) {
    Command::new("git")
        .arg("commit")
        .arg("-m")
        .arg(message)
        .status()
        .expect("Failed to commit.");
}

/// Tag the commit with the given tag.
/// ## Arguments
/// * `tag` - The tag of the commit.
/// ## Example
/// ```
/// use git::tag;
/// tag("v1.0.0");
/// ```
pub fn tag(tag: &str) {
    Command::new("git")
        .arg("tag")
        .arg(tag)
        .status()
        .expect("Failed to tag.");
}

/// Push the changes to the remote.
/// ## Arguments
/// * `have_tag` - If the commit have a tag.
/// ## Example
/// ```
/// use git::push;
/// push(true); // If the commit have a tag.
/// push(false); // If the commit don't have a tag.
/// ```
pub fn push() {
    Command::new("git")
        .arg("push")
        .arg("--tags")
        .status()
        .expect("Failed to push.");
}

/// Stages the files.
/// ## Arguments
/// * `updated` - If only the updated files should be added.
/// ## Example
/// ```
/// use git::add;
/// add(true); // If only the updated files should be added.
/// add(false); // If all the files should be added.
/// ```
pub fn add(updated: bool) {
    Command::new("git")
        .arg("add")
        .arg(if updated { "-u" } else { "-A" })
        .status()
        .expect("Failed to add.");
}

/// Gets the status of the repository.
/// ## Example
/// ```
/// use git::status;
/// status();
/// ```
pub fn status() {
    Command::new("git")
        .arg("status")
        .status()
        .expect("Failed to get status.");
}

/// Gets the 10 last logs of the repository.
/// Displays with the graph view.
/// ## Example
/// ```
/// use git::log;
/// log();
/// ```
pub fn log() {
    Command::new("git")
        .arg("log")
        .arg("--oneline")
        .arg("--graph")
        .arg("--decorate")
        .arg("--all")
        .arg("-n 10")
        .status()
        .expect("Failed to get log.");
}
