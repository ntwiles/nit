use std::fs::read_to_string;

use crate::index::{load_index_as_map, write_index};
use crate::objects::write_object;

pub fn add(mut args: impl Iterator<Item = String>) {
    let file_name = args.next().expect("Must specify which file(s) to add.");

    let contents = read_to_string(&file_name).expect("Could not load file contents.");

    // TODO: Check if object already exists before doing all this work?
    let hash = write_object(&contents);

    let mut index = load_index_as_map();
    index.insert(file_name, hash);

    write_index(&index);
}
