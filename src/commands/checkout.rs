use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use crate::commits::{load_commit_tree, CommitTree};
use crate::objects::read_commit_object;

pub fn checkout<I: Iterator<Item = String>>(mut args: I) {
    let commit_hash = args.next().expect("Must specify a commit.");
    // let commit = read_commit_object(&commit_hash);

    let commit_tree = load_commit_tree(&commit_hash);

    // println!("{commit:#?}");
    println!("{commit_tree:#?}");

    process_tree(Path::new(""), &commit_tree);
}

fn process_tree(path: &Path, tree: &CommitTree) {
    for (child_dir, child) in &tree.dirs {
        process_tree(&path.join(child_dir), &child);
    }

    for (file_name, contents) in &tree.blobs {
        let file_path = path.join(file_name);
        println!("{file_path:?}");
        let mut file = File::create(&file_path).unwrap();
        file.write_all(contents.as_bytes())
            .expect(format!("Failed writing to file: {file_path:?}").as_str());
    }
}
