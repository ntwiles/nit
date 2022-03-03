use std::fs;
use std::io::prelude::*;

pub fn init(mut _args: impl Iterator<Item = String>) {
    fs::remove_dir_all(".nit").ok();
    fs::create_dir(".nit").unwrap();

    let mut head_file = fs::File::create(".nit/HEAD").unwrap();
    head_file
        .write_all("ref: refs/heads/main".as_bytes())
        .unwrap();

    let mut index_file = fs::File::create(".nit/index").unwrap();
    index_file.write_all("".as_bytes()).unwrap();
    index_file.flush().unwrap();

    fs::create_dir(".nit/objects").unwrap();
    fs::create_dir(".nit/refs").unwrap();
    fs::create_dir(".nit/refs/heads").unwrap();
}
