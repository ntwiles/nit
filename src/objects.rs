use std::fs;
use std::io::prelude::*;

use crate::hash::sha_256_hex;

#[derive(Debug)]
pub struct BlobPointer {
    pub hash: String,
    pub file_name: String,
}

#[derive(Debug)]
pub struct TreePointer {
    pub hash: String,
    pub dir_name: String,
}

#[derive(Debug)]
pub enum NitTreeObjectItem {
    BlobPointer(BlobPointer),
    TreePointer(TreePointer),
}

impl NitTreeObjectItem {
    pub fn new(raw: &str) -> Self {
        let mut raw = raw.split(' ');
        let item_type = raw.next().unwrap();

        match item_type {
            "blob" => {
                let hash = raw.next().unwrap().to_string();
                let file_name = raw.next().unwrap().to_string();
                NitTreeObjectItem::BlobPointer(BlobPointer { file_name, hash })
            }
            "tree" => {
                let hash = raw.next().unwrap().to_string();
                let dir_name = raw
                    .next()
                    .expect("Dir name must be defined in tree object references.")
                    .to_string();
                NitTreeObjectItem::TreePointer(TreePointer { hash, dir_name })
            }
            other => panic!("Unknown tree object type {other}"),
        }
    }
}

#[derive(Debug)]
pub struct NitCommitObject {
    pub tree: String,
}

pub fn read_commit_object(hash: &str) -> NitCommitObject {
    let raw = read_object(hash);
    let tree = raw
        .split(' ')
        .nth(1)
        .expect("Could not read commit object. Missing tree hash.")
        .to_string();

    NitCommitObject { tree }
}

pub fn read_tree_object(hash: &str) -> Vec<NitTreeObjectItem> {
    let raw = read_object(hash);
    raw.lines().map(NitTreeObjectItem::new).collect()
}

pub fn read_object(hash: &str) -> String {
    let object_file_name = format!(".nit/objects/{}", hash);
    fs::read_to_string(&object_file_name)
        .unwrap_or_else(|e| panic!("Could not read object file: {object_file_name}: {e}"))
}

pub fn write_object(contents: &str) -> String {
    let hash = sha_256_hex(contents);

    let object_file_name = format!(".nit/objects/{}", hash);
    let mut file = fs::File::create(object_file_name).expect("Could not create object file.");

    file.write_all(contents.as_bytes())
        .expect("Could not write to object file.");
    file.flush().unwrap();

    hash
}
