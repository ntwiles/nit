use std::collections::HashMap;
use std::ffi::OsStr;
use std::fs::read_to_string;
use std::io::prelude::*;
use std::path::Path;

use crate::util::PeekableIterator;

#[derive(Debug)]
pub struct IndexTree {
    pub dirs: Vec<(String, IndexTree)>,
    pub blobs: HashMap<String, String>,
}

impl IndexTree {
    pub fn new() -> Self {
        Self {
            dirs: Vec::new(),
            blobs: HashMap::new(),
        }
    }

    pub fn apply<'a, I>(&mut self, mut path: I, hash: String)
    where
        I: PeekableIterator<Item = &'a OsStr>,
    {
        let name = path.next();

        if let Some(name) = name {
            let name = name.to_str().unwrap().to_owned();

            if path.peek().is_some() {
                let mut dir = IndexTree::new();
                dir.apply(path, hash);

                self.dirs.push((name, dir));
            } else {
                self.blobs.insert(name, hash);
            }
        }
    }
}

pub fn load_index_as_tree() -> IndexTree {
    let map = load_index_as_map();

    let _dirs = HashMap::<String, String>::new();

    let mut tree = IndexTree::new();

    for (file_name, hash) in map {
        let path = Path::new(&file_name);
        tree.apply(path.iter().peekable(), hash);
    }

    tree
}

pub fn load_index_as_map() -> HashMap<String, String> {
    read_to_string(".nit/index")
        .expect("Could not load index file.")
        .lines()
        .map(|l| {
            let kvp: Vec<&str> = l.split(' ').collect();
            (kvp[0].to_string(), kvp[1].to_string())
        })
        .collect()
}

// TODO: Is there a better way to serialize this?
pub fn write_index(index: &HashMap<String, String>) {
    let index_vec: Vec<(&String, &String)> = index.iter().collect();

    let contents = index_vec
        .iter()
        .map(|(key, value)| format!("{key} {value}"))
        .collect::<Vec<String>>()
        .join("\n");

    let mut file = std::fs::OpenOptions::new()
        .write(true)
        .open(".nit/index")
        .expect("Could not open index file to write.");

    file.write_all(contents.as_bytes())
        .expect("Could not write to index file.");
    file.flush().unwrap();
}
