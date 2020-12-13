use nanofilt::*;
use std::env;
use std::process::exit;

fn main() {
    let path = match env::args().nth(1) {
        Some(path) => path,
        None => {
            eprintln!("Usage: nanofilt <input>");
            exit(1);
        }
    };
    if let Err(e) = foo(&path) {
        eprintln!("Error parsing file: {}\n{}", &path, e);
    };
}
