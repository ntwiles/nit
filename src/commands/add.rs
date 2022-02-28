use std::fs::{read_to_string, File};
use std::io::prelude::*;

use crate::hash::sha_256_hex;
use crate::index::{load_index, write_index};

use super::error::Error;

pub fn add(mut args: impl Iterator<Item = String>) -> Result<(), Error> {
    let file_name = args.next().expect("Must specify which file(s) to add.");

    let contents = read_to_string(&file_name)?;
    let hash = sha_256_hex(&contents);

    let mut index = load_index();

    if let Some(existing_hash) = index.get(&file_name) {
        if &hash == existing_hash {
            return Ok(());
        }
    }

    let object_file_name = format!(".nv/objects/{}", hash);
    let mut file = File::create(object_file_name)?;
    file.write_all(contents.as_bytes())?;

    index.remove(&file_name).unwrap();
    index.insert(file_name, hash);

    write_index(&index);

    Ok(())
}
