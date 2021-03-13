use std::{env, fs, path::Path, path::PathBuf, vec::Vec};

use rustls::print::print;

fn main() {
    let target_path = match env::args().nth(1) {
        Some(path) => path,
        None => "./".to_string(),
    };

    let mut dir_pathbufs = read_dir_sorted(target_path);

    dir_pathbufs = filter_invisible(dir_pathbufs);
    print::printcol(&dir_pathbufs);
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

fn filter_invisible(dir_pathbufs: Vec<PathBuf>) -> Vec<PathBuf> {
    let mut output: Vec<PathBuf> = vec![];

    for dir_pathbuf in dir_pathbufs.iter() {
        // HELP: write more shorter 
        if dir_pathbuf
            .as_path()
            .file_name()
            .unwrap()
            .to_os_string()
            .into_string()
            .unwrap()
            .chars()
            .next()
            .unwrap()
            != '.'
        {
            output.push(dir_pathbuf.clone());
        }
    }

    return output;
}
