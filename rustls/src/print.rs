use libc::{ioctl, winsize, STDOUT_FILENO, TIOCGWINSZ};
use std::{mem, path::PathBuf, vec::Vec};

fn window_size() -> Option<usize> {
    let fd = STDOUT_FILENO;

    let mut ws: winsize = unsafe { mem::zeroed() };
    match unsafe { ioctl(fd, TIOCGWINSZ, &mut ws) } == -1 {
        false => Some(ws.ws_col.into()),
        true => None,
    }
}

fn transpose(dir_pathbufs: &[PathBuf], width: &usize, hight: &usize) -> Vec<PathBuf> {
    let mut input: Vec<PathBuf> = dir_pathbufs.to_owned();
    let mut output: Vec<PathBuf> = vec![PathBuf::default(); width * hight];
    // HELP: please more efficient algorithm.
    'outer: for i in 0..*width {
        for j in 0..*hight {
            output[(j * width) + i] = input[0].clone();
            input.remove(0);
            if input.is_empty() {
                break 'outer;
            }
        }
    }
    output
}

fn file_name_by_pathbuf(dir_pathbuf: &PathBuf) -> String {
    match dir_pathbuf.file_name() {
        Some(file_name) => file_name.to_os_string().into_string().unwrap_or("".to_string()),
        None => "".to_string(), 
    }
}
#[test]
fn test_file_name_by_pathbuf() {
    let pathbuf = PathBuf::default();
    assert_eq!(file_name_by_pathbuf(&pathbuf), "".to_string());
    let mut pathbuf = PathBuf::from("./");
    pathbuf.set_file_name("test.txt");
    assert_eq!(file_name_by_pathbuf(&pathbuf), "test.txt".to_string());
}

pub fn printcol(dir_pathbufs: &[PathBuf]) {
    if dir_pathbufs.is_empty() {
        return;
    }

    let maxsize: usize = window_size().unwrap_or(0);

    let mut colwidth = 0;
    for dir_pathbuf in dir_pathbufs {
        let file_name_len = (file_name_by_pathbuf(dir_pathbuf) + "\t").len();
        if colwidth < file_name_len {
            colwidth = file_name_len;
        }
    }

    let numcols = maxsize / colwidth - 1;
    let mut output = dir_pathbufs.to_owned();
    output = transpose(
        &output,
        &numcols,
        &((dir_pathbufs.len() + (numcols - 1)) / numcols),
    );

    for (idx, dir_pathbuf) in output.iter().enumerate() {
        let file_name = file_name_by_pathbuf(dir_pathbuf);
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
