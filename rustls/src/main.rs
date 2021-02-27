use libc::{ioctl, winsize, STDOUT_FILENO, TIOCGWINSZ};
use std::{env, fs, path::Path, path::PathBuf, vec::Vec, mem};

fn main() {
    let target_path = match env::args().nth(1) {
        Some(path) => path,
        None => "./".to_string(),
    };

    let mut dir_pathbufs = read_dir_sorted(target_path);

    dir_pathbufs = filter_invisible(dir_pathbufs);
    printcol(&dir_pathbufs);
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

fn window_size() -> Option<winsize> {
    let fd = STDOUT_FILENO;

    let mut ws: winsize = unsafe { mem::zeroed() };
    if unsafe { ioctl(fd, TIOCGWINSZ, &mut ws) } == -1 {
        None
    } else {
        Some(ws)
    }
}

fn printcol(dir_pathbufs: &Vec<PathBuf>) {
    if dir_pathbufs.len() == 0 {
        return;
    }
    let maxsize: usize = match window_size() {
        Some(ws) => ws.ws_col.into(),
        None => 0,
    };

    let mut colwidth = 0;
    for dir_pathbuf in dir_pathbufs {
        let file_name_len = (dir_pathbuf
            .file_name()
            .unwrap()
            .to_os_string()
            .into_string()
            .unwrap()
            + "\t")
            .len();
        if colwidth < file_name_len {
            colwidth = file_name_len;
        }
    }

    let numcols = maxsize / colwidth - 1;

    for (idx, dir_pathbuf) in dir_pathbufs.iter().enumerate() {
        let file_name = match dir_pathbuf.file_name() {
            Some(file_name) => file_name.to_os_string().into_string().unwrap(),
            None => "".to_string(),
        };
        let file_name_len = file_name.len();

        if file_name_len != 0 {
            print!("{}", file_name);
            for _ in 0..(colwidth - file_name_len) {
                print!(" ");
            }
        }
        if idx != 0 && (idx + 1) % numcols == 0 {
            print!("\n");
        } else {
            print!("\t");
        }
    }

    if numcols > dir_pathbufs.len() {
        println!("");
    }
}
