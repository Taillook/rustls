use libc::{ioctl, winsize, STDOUT_FILENO, TIOCGWINSZ};
use std::{fs, mem, ptr, path::PathBuf, vec::Vec, os::unix::fs::MetadataExt, ffi::CStr};

mod filesystem;
use filesystem::FileSystem;

fn window_size() -> Option<usize> {
    let fd = STDOUT_FILENO;

    let mut ws: winsize = unsafe { mem::zeroed() };
    match unsafe { ioctl(fd, TIOCGWINSZ, &mut ws) } == -1 {
        false => Some(ws.ws_col.into()),
        true => None,
    }
}

fn transpose(pathbufs: &[PathBuf], width: &usize, hight: &usize) -> Vec<PathBuf> {
    let mut input: Vec<PathBuf> = pathbufs.to_owned();
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

fn file_name_by_pathbuf(pathbuf: &PathBuf) -> String {
    match pathbuf.file_name() {
        Some(file_name) => file_name
            .to_os_string()
            .into_string()
            .unwrap_or_else(|_| "".to_string()),
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

pub fn printcol(pathbufs: &[PathBuf]) {
    if pathbufs.is_empty() {
        return;
    }

    let maxsize: usize = window_size().unwrap_or(0);

    let mut colwidth = 0;
    for pathbuf in pathbufs {
        let file_name_len = (file_name_by_pathbuf(pathbuf) + "\t").len();
        if colwidth < file_name_len {
            colwidth = file_name_len;
        }
    }

    let numcols = maxsize / colwidth - 1;
    let mut output = pathbufs.to_owned();
    output = transpose(
        &output,
        &numcols,
        &((pathbufs.len() + (numcols - 1)) / numcols),
    );

    for (idx, pathbuf) in output.iter().enumerate() {
        let file_name = file_name_by_pathbuf(pathbuf);
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

pub fn printscol(pathbufs: &[PathBuf]) {
    if pathbufs.is_empty() {
        return;
    }

    for pathbuf in pathbufs.iter() {
        let file_name = file_name_by_pathbuf(pathbuf);
        println!("{}", file_name);
    }
}

pub fn printstream(pathbufs: &[PathBuf]) {
    if pathbufs.is_empty() {
        return;
    }

    for (idx, pathbuf) in pathbufs.iter().enumerate() {
        let file_name = file_name_by_pathbuf(pathbuf);
        match pathbufs.len() == idx + 1 {
            false => print!("{}, ", file_name),
            true => print!("{}", file_name),
        }
    }
    println!();
}

pub fn printlong(pathbufs: &[PathBuf]) {
    if pathbufs.is_empty() {
        return;
    }

    for pathbuf in pathbufs.iter() {
        let path = pathbuf.as_path();
        let file_name = file_name_by_pathbuf(pathbuf);
        let metadata = path.metadata().expect("metadata call failed");
        let len = metadata.len();
        let permission = pathbuf.permission();
        println!("{}{} {} {} {} {}", type_str(metadata.file_type()), permission, get_unix_username(metadata.uid()).unwrap(), metadata.gid(), len, file_name);
    }
}

fn type_str(file_type: fs::FileType) -> &'static str {
    if file_type.is_dir() {
        "d"
    } else if file_type.is_file() {
        "-"
    } else {
        "l"
    }
}

fn get_unix_username(uid: u32) -> Option<String> {

    unsafe {
        let mut result = ptr::null_mut();
        let amt = match libc::sysconf(libc::_SC_GETPW_R_SIZE_MAX) {
            n if n < 0 => 512 as usize,
            n => n as usize,
        };
        let mut buf = Vec::with_capacity(amt);
        let mut passwd: libc::passwd = mem::zeroed();

        match libc::getpwuid_r(uid, &mut passwd, buf.as_mut_ptr(),
                              buf.capacity() as libc::size_t,
                              &mut result) {
           0 if !result.is_null() => {
               let ptr = passwd.pw_name as *const _;
               let username = CStr::from_ptr(ptr).to_str().unwrap().to_owned();
               Some(username)
           },
           _ => None
        }
    }

}
