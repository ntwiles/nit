use std::collections::HashMap;
use std::fs::read_to_string;
use std::io::prelude::*;

pub fn load_index() -> HashMap<String, String> {
    read_to_string(".nv/index")
        .expect("Could not load index file!")
        .lines()
        .map(|l| {
            let kvp: Vec<&str> = l.split(":").collect();
            (kvp[0].to_string(), kvp[1].to_string())
        })
        .collect()
}

pub fn write_index(index: &HashMap<String, String>) {
    let index_vec: Vec<(&String, &String)> = index.into_iter().collect();

    let contents = index_vec
        .iter()
        .map(|(key, value)| format!("{key}:{value}"))
        .collect::<Vec<String>>()
        .join("\n");

    let mut file = std::fs::OpenOptions::new()
        .write(true)
        .open(".nv/index")
        .expect("Could not open index file to write!");

    file.write_all(contents.as_bytes())
        .expect("Could not write to index file!");
}
