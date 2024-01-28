use std::{env, path::PathBuf};

use crate::{git, utils};
use crate::{AddArgs, CloneArgs, LinkArgs, RemoveArgs};

pub fn handle_init() {
    let repo = git::init_bare(get_dottler_path());

    repo.set_workdir(get_home_path().as_path(), true).unwrap();
    let mut config = repo.config().unwrap();
    config.set_str("status.showUntrackedFiles", "no").unwrap();

    println!(
        "Initialized empty dottler repository in {}",
        repo.path().display()
    );
}

pub fn handle_link(args: LinkArgs) {
    let repo = git::open_bare(get_dottler_path());
    git::add_remote(repo, &args.url);
}

pub fn handle_clone(args: CloneArgs) {
    let _repo = git::clone_bare(&args.url, get_dottler_path());
}

pub fn handle_add(args: AddArgs) {
    let repo = git::open_bare(get_dottler_path());

    let paths = utils::expand_and_normalize_paths(
        args.files.clone(),
        get_home_path(),
        env::current_dir().expect("Failed to get current directory"),
    );
    repo.set_workdir(get_home_path().as_path(), true).unwrap();

    match git::add_to_index(repo, paths) {
        Ok(_) => println!("Added files to dottler index"),
        Err(e) => {
            eprintln!(
                "Failed to add files to dottler index!\nMore Info: {}",
                e.message()
            );

            // print name of all possible errors
            print_git_error_code(e.code());

            std::process::exit(exitcode::IOERR);
        }
    }
}

pub fn handle_remove(_args: RemoveArgs) {}

pub fn handle_sync() {
    let repo = git::open_bare(get_dottler_path());

    match git::update_tracked_files(repo) {
        Ok(_) => println!("Updated tracked files"),
        Err(e) => {
            eprintln!(
                "Failed to update tracked files!\nMore Info: {}",
                e.message()
            );
            std::process::exit(exitcode::IOERR);
        }
    };
}

pub fn handle_status() {}

fn get_dottler_path() -> PathBuf {
    get_home_path().join(".dottler")
}

fn get_home_path() -> PathBuf {
    PathBuf::from(env::var("HOME").unwrap())
}

fn print_git_error_code(error_code: git2::ErrorCode) {
    match error_code {
        git2::ErrorCode::GenericError => {
            eprintln!("Error: GenericError");
        }
        git2::ErrorCode::BufSize => {
            eprintln!("Error: BufSize");
        }
        git2::ErrorCode::HashsumMismatch => {
            eprintln!("Error: HashsumMismatch");
        }
        git2::ErrorCode::IndexDirty => {
            eprintln!("Error: IndexDirty");
        }
        git2::ErrorCode::Owner => {
            eprintln!("Error: Owner");
        }
        git2::ErrorCode::ApplyFail => {
            eprintln!("Error: ApplyFail");
        }
        git2::ErrorCode::NotFound => {
            eprintln!("Error: NotFound");
        }
        git2::ErrorCode::Invalid => {
            eprintln!("Error: Invalid");
        }
        git2::ErrorCode::Exists => {
            eprintln!("Error: Exists");
        }
        git2::ErrorCode::Ambiguous => {
            eprintln!("Error: Ambiguous");
        }
        git2::ErrorCode::User => {
            eprintln!("Error: User");
        }
        git2::ErrorCode::BareRepo => {
            eprintln!("Error: Barerepo");
        }
        git2::ErrorCode::UnbornBranch => {
            eprintln!("Error: UnbornBranch");
        }
        git2::ErrorCode::Unmerged => {
            eprintln!("Error: Unmerged");
        }
        git2::ErrorCode::NotFastForward => {
            eprintln!("Error: Nonfastforward");
        }
        git2::ErrorCode::InvalidSpec => {
            eprintln!("Error: InvalidSpec");
        }
        git2::ErrorCode::Conflict => {
            eprintln!("Error: Conflict");
        }
        git2::ErrorCode::Locked => {
            eprintln!("Error: Locked");
        }
        git2::ErrorCode::Modified => {
            eprintln!("Error: Modified");
        }
        git2::ErrorCode::Auth => {
            eprintln!("Error: Auth");
        }
        git2::ErrorCode::Certificate => {
            eprintln!("Error: Certificate");
        }
        git2::ErrorCode::Applied => {
            eprintln!("Error: Applied");
        }
        git2::ErrorCode::Peel => {
            eprintln!("Error: Peel");
        }
        git2::ErrorCode::Eof => {
            eprintln!("Error: EOF");
        }
        git2::ErrorCode::Uncommitted => {
            eprintln!("Error: Uncommitted");
        }
        git2::ErrorCode::Directory => {
            eprintln!("Error: Directory");
        }
        git2::ErrorCode::MergeConflict => {
            eprintln!("Error: MergeConflict");
        }
    }
}
