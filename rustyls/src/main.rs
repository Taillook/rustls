use clap::{app_from_crate, crate_authors, crate_description, crate_name, crate_version, Arg};
use std::{env, fs, path::Path, path::PathBuf, process, vec::Vec};

mod print;

/// struct of flag for print.
struct CommandFlagForPrint {
    colmuns: bool,
}

fn main() {
    let app = app_from_crate!()
        .arg(
            Arg::with_name("columns")
                .help("list entries by columns")
                .short("C"),
        )
        .arg(Arg::with_name("file").help("FILE").index(1));
    let matches = app.get_matches();

    let command_flag = CommandFlagForPrint {
        colmuns: matches.is_present("columns"),
    };

    let flag_called = command_flag.colmuns;

    if command_flag.colmuns {
        colmuns(matches.value_of("file"));
    }

    if !flag_called {
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
