use std::{env, fs, path::Path, path::PathBuf, vec::Vec};

fn main() {
    let target_path = match env::args().nth(1) {
        Some(path) => path,
        None => "./".to_string(),
    };

    let paths = read_dir(target_path);
    for path in paths {
        println!("{}", path.as_path().display())
    }
}

fn read_dir(target_path: String) -> Vec<PathBuf> {
    return fs::read_dir(Path::new(&target_path))
        .unwrap()
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.path())
        .collect::<Vec<PathBuf>>();
}
