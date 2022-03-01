use std::fs::File;
use std::io::prelude::*;

use crate::hash::sha_256_hex;

pub fn write_object(contents: &str) -> String {
    let hash = sha_256_hex(&contents);

    let object_file_name = format!(".nit/objects/{}", hash);
    let mut file = File::create(object_file_name).expect("Could not create object file.");

    file.write_all(contents.as_bytes())
        .expect("Could not write to object file.");

    hash
}
