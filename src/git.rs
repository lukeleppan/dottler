use std::{
    env,
    path::{Path, PathBuf},
};

use git2::{Cred, Index, RemoteCallbacks, Repository};

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

pub fn add_remote(repo: Repository, remote_url: &str) {
    match repo.remote("origin", remote_url) {
        Ok(_) => {}
        Err(e) => {
            eprintln!(
                "Failed to add remote to dottler repository!\nMore Info: {}",
                e.message()
            );
            std::process::exit(exitcode::IOERR);
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

pub fn commit_to_repo(
    repo: Repository,
    index: &mut Index,
    message: &str,
) -> Result<(), git2::Error> {
    index.write()?;

    let signature = repo.signature()?;
    let tree_id = index.write_tree()?;
    let tree = repo.find_tree(tree_id)?;

    let parents = if let Some(head) = repo.head().ok().and_then(|h| h.target()) {
        vec![repo.find_commit(head)?]
    } else {
        Vec::new()
    };

    let parents = parents.iter().collect::<Vec<_>>();

    repo.commit(
        Some("HEAD"),
        &signature,
        &signature,
        format!("{} {}", chrono::Local::now().timestamp_millis(), message,).as_str(),
        &tree,
        &parents,
    )?;

    Ok(())
}

pub fn add_to_index(repo: Repository, spec: Vec<String>) -> Result<(), git2::Error> {
    let mut index = repo.index().expect("Failed to get index");
    for pathspec in spec {
        index.add_path(Path::new(&pathspec))?;
    }

    commit_to_repo(repo, &mut index, "Add new files to Dottler")?;

    Ok(())
}

pub fn remove_tracked_files(repo: Repository, paths: Vec<String>) -> Result<(), git2::Error> {
    let mut index = repo.index()?;

    for path in paths {
        index.remove_path(Path::new(&path))?;
    }

    commit_to_repo(repo, &mut index, "Remove files from Dottler")?;

    Ok(())
}

pub fn update_tracked_files(repo: Repository) -> Result<(), git2::Error> {
    let mut index = repo.index()?;

    for entry in repo.index()?.iter() {
        let path = Path::new(std::str::from_utf8(&entry.path).unwrap());
        if path.exists() {
            index.add_path(path)?;
        }
    }

    commit_to_repo(repo, &mut index, "Update Dottler files")?;

    Ok(())
}

// pub fn pull_remote(repo: Repository) -> Result<(), git2::Error> {
//     repo.find_remote("origin")?
//         .fetch(&["master"], None, None)
//         .unwrap();
//
//     Ok(())
// }

pub fn push_remote(repo: Repository) -> Result<(), git2::Error> {
    let mut remote = repo.find_remote("origin")?;
    let mut callbacks = RemoteCallbacks::new();
    callbacks.credentials(|_user, username_from_url, _allowed_types| {
        Cred::ssh_key_from_agent(username_from_url.unwrap())
    });
    let mut push_options = git2::PushOptions::new();
    push_options.remote_callbacks(callbacks);
    remote.push(&["refs/heads/master"], Some(&mut push_options))?;
    Ok(())
}
