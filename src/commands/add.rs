use std::fs::{read_to_string, File};
use std::io::prelude::*;

use crate::hash::sha_256_hex;
use crate::index::{load_index, write_index};

pub fn add(mut args: impl Iterator<Item = String>) {
    let file_name = args.next().expect("Must specify which file(s) to add.");

    let contents = read_to_string(&file_name).expect("Could not load file contents.");
    let hash = sha_256_hex(&contents);

    let mut index = load_index();

    if let Some(existing_hash) = index.get(&file_name) {
        if &hash == existing_hash {
            return;
        }
    }

    let object_file_name = format!(".nv/objects/{}", hash);
    let mut file = File::create(object_file_name).expect("Could not create object file.");

    file.write_all(contents.as_bytes())
        .expect("Could not write to object file.");

    index.remove(&file_name).unwrap();
    index.insert(file_name, hash);

    write_index(&index);
}
