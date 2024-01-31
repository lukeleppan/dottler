use std::path::{Path, PathBuf};

use walkdir::WalkDir;

pub fn expand_and_normalize_paths(
    paths: Vec<String>,
    homedir: PathBuf,
    workdir: PathBuf,
) -> Vec<String> {
    let mut new_paths: Vec<String> = Vec::new();

    for path in paths {
        let mut path = PathBuf::from(path);
        if path.is_relative() {
            path = workdir.clone().join(path);
        }

        let path = path.canonicalize().expect("Invalid file path");
        if path.is_dir() {
            for entry in WalkDir::new(&path) {
                let entry = entry.unwrap();
                let path_str = entry.path().to_str().unwrap();
                if path_str.contains(".git") {
                    continue;
                }
                if entry.file_type().is_file() {
                    let relative_path = convert_to_relative_path(entry.path(), &homedir);
                    println!("{:?}", relative_path);
                    new_paths.push(relative_path);
                }
            }
        } else {
            let relative_path = convert_to_relative_path(&path, &homedir);
            println!("{:?}", relative_path);
            new_paths.push(relative_path);
        }
    }

    new_paths
}

fn convert_to_relative_path(path: &Path, homedir: &PathBuf) -> String {
    let path = path.strip_prefix(homedir).unwrap();
    path.to_str().unwrap().to_string()
}
