use libc::{ioctl, winsize, STDOUT_FILENO, TIOCGWINSZ};
use std::{mem, path::PathBuf, vec::Vec};

fn window_size() -> Option<winsize> {
    let fd = STDOUT_FILENO;

    let mut ws: winsize = unsafe { mem::zeroed() };
    if unsafe { ioctl(fd, TIOCGWINSZ, &mut ws) } == -1 {
        None
    } else {
        Some(ws)
    }
}

fn transpose(dir_pathbufs: Vec<PathBuf>, width: usize, hight: usize) -> Vec<PathBuf> {
    let mut input: Vec<PathBuf> = dir_pathbufs.clone();
    let mut output: Vec<PathBuf> = vec![PathBuf::default(); width * hight];
    // HELP: please more efficient algorithm.
    'outer: for i in 0..width {
        for j in 0..hight {
            output[(j * width) + i] = input[0].clone();
            input.remove(0);
            if input.len() == 0 {
                break 'outer;
            }
        }
    }
    output
}

pub fn printcol(dir_pathbufs: &Vec<PathBuf>) {
    if dir_pathbufs.is_empty() {
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
    let mut output = dir_pathbufs.clone();
    output = transpose(
        output,
        numcols,
        (dir_pathbufs.len() + (numcols - 1)) / numcols,
    );

    for (idx, dir_pathbuf) in output.iter().enumerate() {
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
            println!();
        } else {
            print!("\t");
        }
    }
}
