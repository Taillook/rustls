use std::{env, fs, path::Path, path::PathBuf, process, vec::Vec};

mod print;

fn main() {
    let target_path_name = match env::args().nth(1) {
        Some(path) => path,
        None => "./".to_string(),
    };

    let target_path = Path::new(&target_path_name);
    if !target_path.exists() {
        println!(
            "rustls: {}: No such file or directory",
            target_path.to_str().unwrap()
        );
        process::exit(1);
    }
    if target_path.is_dir() {
        let dir_pathbufs = filter_invisible(&read_dir_sorted(target_path));
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

fn filter_invisible(dir_pathbufs: &[PathBuf]) -> Vec<PathBuf> {
    let mut output: Vec<PathBuf> = vec![];

    for dir_pathbuf in dir_pathbufs.iter() {
        let dir_name_str = dir_pathbuf.as_path().file_name().unwrap().to_str().unwrap();
        if !dir_name_str.starts_with('.') {
            output.push(dir_pathbuf.clone());
        }
    }

    output
}
