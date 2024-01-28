use git2::Repository;
use std::{env, path::PathBuf};

use crate::{AddArgs, LinkArgs, RemoveArgs};

pub fn handle_init() {
    let home_dir = env::var("HOME").unwrap();
    let repo_path = PathBuf::from(home_dir).join(".dottler");

    let repo = match Repository::init(repo_path) {
        Ok(repo) => repo,
        Err(_) => panic!("Failed to initialize repository"),
    };
    println!(
        r"Initialized empty Git repository in {}.",
        repo.workdir().unwrap().to_string_lossy()
    );
}

pub fn handle_link(_args: LinkArgs) {}

pub fn handle_add(_args: AddArgs) {}

pub fn handle_remove(_args: RemoveArgs) {}

pub fn handle_sync() {}

pub fn handle_status() {}
