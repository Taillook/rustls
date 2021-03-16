use std::{env, fs, path::Path, path::PathBuf, vec::Vec};

mod print;

fn main() {
    let target_path_name = match env::args().nth(1) {
        Some(path) => path,
        None => "./".to_string(),
    };

    let target_path = Path::new(&target_path_name);
    if target_path.is_dir() {
        let mut dir_pathbufs = read_dir_sorted(target_path);
        dir_pathbufs = filter_invisible(dir_pathbufs);
        print::printcol(&dir_pathbufs);
    } else {
        println!("{}", target_path.to_str().unwrap());
    }
}

fn read_dir(target_path: &Path) -> Vec<PathBuf> {
    fs::read_dir(&target_path)
        .unwrap()
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.path())
        .collect::<Vec<PathBuf>>()
}

fn read_dir_sorted(target_path: &Path) -> Vec<PathBuf> {
    let mut dir_pathbufs = read_dir(target_path);

    dir_pathbufs.sort_by(|a, b| {
        a.as_path()
            .file_name()
            .unwrap()
            .to_os_string()
            .cmp(&b.as_path().file_name().unwrap().to_os_string())
    });

    dir_pathbufs
}

fn filter_invisible(dir_pathbufs: Vec<PathBuf>) -> Vec<PathBuf> {
    let mut output: Vec<PathBuf> = vec![];

    for dir_pathbuf in dir_pathbufs.iter() {
        if !dir_pathbuf
            .as_path()
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .starts_with('.')
        {
            output.push(dir_pathbuf.clone());
        }
    }

    output
}
