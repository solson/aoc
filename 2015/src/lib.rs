use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

pub fn read_file<P: AsRef<Path>>(path: P) -> String {
    let mut contents = String::new();
    File::open(path).unwrap().read_to_string(&mut contents).unwrap();
    contents
}
