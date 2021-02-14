use std::path::Path;
use std::env;
use std::fs;

fn main() {
    let target_path = match env::args().nth(1) {
        Some(path) => path,
        None => "./".to_string(),
    };

    let paths = fs::read_dir(Path::new(&target_path)).unwrap();
    for path in paths {
        println!("{}", path.unwrap().path().display())
    }
}
