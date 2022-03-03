use std::fs;
use std::io::prelude::*;
use std::path::Path;

use crate::commits::{load_commit_tree, CommitTree};

pub fn checkout<I: Iterator<Item = String>>(mut args: I) {
    let commit_hash = args.next().expect("Must specify a commit.");

    // TODO: This loads the commit object as if it were a tree object,
    // which isn't correct and won't work when more fields are added
    // to the commit object.
    let commit_tree = load_commit_tree(&commit_hash);

    let paths = fs::read_dir("./").unwrap();

    for path in paths {
        let path = path.unwrap();
        if ".nit" != path.file_name() {
            let md = fs::metadata(path.path()).unwrap();
            if md.is_file() {
                fs::remove_file(path.path())
                    .unwrap_or_else(|e| panic!("Could not remove file {:?}: {e}", path.path()));
            } else if md.is_dir() {
                fs::remove_dir_all(path.path())
                    .unwrap_or_else(|e| panic!("Coult not remove dir {:?}: {e}", path.path()));
            }
        }
    }

    process_tree(Path::new(""), &commit_tree);
}

fn process_tree(path: &Path, tree: &CommitTree) {
    for (child_dir, child) in &tree.dirs {
        process_tree(&path.join(child_dir), child);
    }

    for (file_name, contents) in &tree.blobs {
        let file_path = path.join(file_name);

        fs::create_dir_all(path).unwrap();

        let mut file = fs::File::create(&file_path)
            .unwrap_or_else(|e| panic!("Failed to create file {file_path:?}: {e}"));
        file.write_all(contents.as_bytes())
            .unwrap_or_else(|e| panic!("Failed to write to file: {file_path:?}: {e}"));
        file.flush().unwrap();
    }
}
