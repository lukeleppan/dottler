use std::{
    env,
    path::{Path, PathBuf},
};

use git2::{Cred, RemoteCallbacks, Repository};

pub fn init_bare(repo_path: PathBuf) -> Repository {
    match Repository::init_bare(repo_path) {
        Ok(repo) => repo,
        Err(e) => {
            eprintln!(
                "Failed to initialize dottler repository!\nMore Info: {}",
                e.message()
            );

            std::process::exit(exitcode::CANTCREAT);
        }
    }
}

pub fn clone_bare(remote_url: &str, repo_path: PathBuf) -> Repository {
    let mut callbacks = RemoteCallbacks::new();
    callbacks.credentials(|_user, username_from_url, _allowed_types| {
        Cred::ssh_key(
            username_from_url.unwrap(),
            None,
            Path::new(&format!("{}/.ssh/id_rsa", env::var("HOME").unwrap())),
            None,
        )
    });

    let mut fetch_options = git2::FetchOptions::new();
    fetch_options.remote_callbacks(callbacks);

    let mut builder = git2::build::RepoBuilder::new();
    builder.fetch_options(fetch_options);

    builder.bare(true);

    match builder.clone(remote_url, &repo_path) {
        Ok(repo) => repo,
        Err(e) => {
            eprintln!(
                "Failed to clone dottler repository!\nMore Info: {}",
                e.message()
            );
            std::process::exit(exitcode::IOERR);
        }
    }
}

pub fn open_bare(repo_path: PathBuf) -> Repository {
    match Repository::open_bare(repo_path) {
        Ok(repo) => repo,
        Err(_) => panic!("Failed to open repository"),
    }
}

pub fn add_to_index(repo: Repository, spec: Vec<String>) -> Result<(), git2::Error> {
    let mut index = repo.index().expect("Failed to get index");
    for pathspec in spec {
        index.add_path(Path::new(&pathspec))?;
    }

    index.write()?;
    Ok(())
}
