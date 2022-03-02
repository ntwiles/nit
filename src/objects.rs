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
                let dir_name = raw.next().unwrap().to_string();
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
    let mut tree = String::new();

    for line in read_object(hash).lines() {
        let mut line = line.split(' ');
        let label = line.next().unwrap();

        match label {
            "tree" => {
                tree = line
                    .next()
                    .expect("Must supply a hash value to commit tree field.")
                    .to_string()
            }
            _ => todo!(),
        }
    }

    NitCommitObject { tree }
}

pub fn read_tree_object(hash: &str) -> Vec<NitTreeObjectItem> {
    println!("Reading tree object: {hash}");
    let raw = read_object(hash);
    raw.lines().map(|l| NitTreeObjectItem::new(l)).collect()
}

pub fn read_object(hash: &str) -> String {
    let object_file_name = format!(".nit/objects/{}", hash);
    fs::read_to_string(object_file_name)
        .expect(format!("Could not read object file: {hash}").as_str())
}

pub fn write_object(contents: &str) -> String {
    let hash = sha_256_hex(&contents);

    println!("Writing hash {hash}");
    let object_file_name = format!(".nit/objects/{}", hash);
    let mut file = fs::File::create(object_file_name).expect("Could not create object file.");

    file.write_all(contents.as_bytes())
        .expect("Could not write to object file.");

    hash
}
