use crate::index::{load_index_as_tree, IndexTree};
use crate::objects::write_object;

pub fn commit(mut _args: impl Iterator<Item = String>) {
    let commit_tree = load_index_as_tree();

    let tree_hash = write_tree(commit_tree);

    let commit = format!("tree {tree_hash}");
    write_object(&commit);

    // Point current branch to new object.
}

fn write_tree(tree: IndexTree) -> String {
    let mut contents = String::new();

    for (dir_name, dir_tree) in tree.dirs {
        let hash = write_tree(dir_tree);
        contents += format!("tree {hash} {dir_name}\n").as_str();
    }

    for (blob_name, hash) in tree.blobs {
        contents += format!("blob {hash} {blob_name}\n").as_str();
    }

    write_object(&contents)
}
