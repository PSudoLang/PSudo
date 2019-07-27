use std::fs::File;
use std::io::Read;
use std::path::Path;

pub fn read_file(path: &Path) -> String {
    let mut f = File::open(path).unwrap();
    let mut s = String::new();
    f.read_to_string(&mut s).unwrap();

    s
}
