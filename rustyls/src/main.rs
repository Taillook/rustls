use clap::{app_from_crate, crate_authors, crate_description, crate_name, crate_version, Arg};
use std::{env, fs, path::Path, path::PathBuf, process, vec::Vec};

mod print;

fn main() {
    let app = app_from_crate!()
        .arg(
            Arg::with_name("columns")
                .help("list entries by columns")
                .short("C"),
        )
        .arg(
            Arg::with_name("one")
                .help("list one file per line")
                .short("1"),
        )
        .arg(
            Arg::with_name("stream")
                .help("fill width with a comma separated list of entries")
                .short("m"),
        )
        .arg(Arg::with_name("file").help("FILE").index(1));
    let matches = app.get_matches();

    if matches.is_present("columns") {
        colmuns(matches.value_of("file"));
    } else if matches.is_present("one") {
        one(matches.value_of("file"));
    }  else if matches.is_present("stream") {
        stream(matches.value_of("file"));
    } else {
        colmuns(matches.value_of("file"));
    }
}

fn colmuns(file_name: Option<&str>) {
    let target_path_name = match file_name {
        Some(path) => path.to_string(),
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
        let pathbufs = filter_invisible(&read_dir_sorted(target_path));
        print::printcol(&pathbufs.unwrap());
    } else {
        println!("{}", target_path.to_str().unwrap());
    }
}

fn one(file_name: Option<&str>) {
    let target_path_name = match file_name {
        Some(path) => path.to_string(),
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
        let pathbufs = filter_invisible(&read_dir_sorted(target_path));
        print::printscol(&pathbufs.unwrap());
    } else {
        println!("{}", target_path.to_str().unwrap());
    }
}

fn stream(file_name: Option<&str>) {
    let target_path_name = match file_name {
        Some(path) => path.to_string(),
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
        let pathbufs = filter_invisible(&read_dir_sorted(target_path));
        print::printstream(&pathbufs.unwrap());
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
    let mut pathbufs = read_dir(target_path);

    pathbufs.sort_by(|a, b| {
        a.as_path()
            .file_name()
            .unwrap()
            .to_os_string()
            .cmp(&b.as_path().file_name().unwrap().to_os_string())
    });

    pathbufs
}

fn filter_invisible(pathbufs: &[PathBuf]) -> Option<Vec<PathBuf>> {
    let mut output: Vec<PathBuf> = vec![];

    for pathbuf in pathbufs.iter() {
        let name_str = pathbuf.as_path().file_name()?.to_str()?;
        if !name_str.starts_with('.') {
            output.push(pathbuf.clone());
        }
    }

    Some(output)
}
