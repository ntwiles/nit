use std::collections::HashMap;

use crate::objects::{read_object, read_tree_object, BlobPointer, NitTreeObjectItem, TreePointer};

#[derive(Debug)]
pub struct CommitTree {
    pub dirs: HashMap<String, CommitTree>,
    pub blobs: Vec<(String, String)>,
}

pub fn load_commit_tree(tree_object: &str) -> CommitTree {
    let tree = read_tree_object(tree_object);

    let mut dirs = HashMap::<String, CommitTree>::new();
    let mut blobs = Vec::new();

    for item in tree {
        println!("item: {item:#?}");
        match item {
            NitTreeObjectItem::BlobPointer(blob_pointer) => {
                let BlobPointer { file_name, hash } = blob_pointer;
                blobs.push((file_name, read_object(&hash)));
            }
            NitTreeObjectItem::TreePointer(tree_pointer) => {
                let TreePointer { dir_name, hash } = tree_pointer;
                dirs.insert(dir_name, load_commit_tree(&hash));
            }
        };
    }

    CommitTree { dirs, blobs }
}
