mod error;

use std::fs::{read_to_string, remove_file, File};
use std::io::prelude::*;

use crate::difference::get_diff;
use crate::hash::sha_256_hex;
use crate::index::{load_index, write_index};

use error::Error;

pub fn diff() -> Result<(), Error> {
    let file_a = read_to_string(".nv/trackMe.txt")?;
    let file_b = read_to_string("data/trackMe.txt")?;

    let deltas = get_diff(file_a.lines(), file_b.lines());

    for delta in deltas {
        println!("{delta}");
    }

    Ok(())
}

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

    let existing_hash = index.remove(&file_name).unwrap();
    remove_file(format!(".nv/objects/{}", existing_hash))?;

    let object_file_name = format!(".nv/objects/{}", hash);
    let mut file = File::create(object_file_name)?;
    file.write_all(contents.as_bytes())?;

    index.insert(file_name, hash);

    write_index(&index);

    Ok(())
}
