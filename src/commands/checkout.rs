use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use crate::commits::{load_commit_tree, CommitTree};

pub fn checkout<I: Iterator<Item = String>>(mut args: I) {
    let commit_hash = args.next().expect("Must specify a commit.");

    // TODO: This loads the commit object as if it were a tree object,
    // which isn't correct and won't work when more fields are added
    // to the commit object.
    let commit_tree = load_commit_tree(&commit_hash);

    process_tree(Path::new(""), &commit_tree);
}

fn process_tree(path: &Path, tree: &CommitTree) {
    for (child_dir, child) in &tree.dirs {
        process_tree(&path.join(child_dir), child);
    }

    for (file_name, contents) in &tree.blobs {
        let file_path = path.join(file_name);
        let mut file = File::create(&file_path).unwrap();
        file.write_all(contents.as_bytes())
            .unwrap_or_else(|_| panic!("Failed writing to file: {file_path:?}"));
    }
}
