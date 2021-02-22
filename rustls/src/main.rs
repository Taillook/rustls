use std::{env, fs, path::Path, path::PathBuf, vec::Vec};

fn main() {
    let target_path = match env::args().nth(1) {
        Some(path) => path,
        None => "./".to_string(),
    };

    let paths = read_dir_sorted(target_path);
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

fn read_dir_sorted(target_path: String) -> Vec<PathBuf> {
    let mut dir_pathbufs = read_dir(target_path);

    dir_pathbufs.sort_by(|a, b| {
        a.as_path()
            .file_name()
            .unwrap()
            .to_os_string()
            .cmp(&b.as_path().file_name().unwrap().to_os_string())
    });

    return dir_pathbufs;
}
