use crate::index::load_index_as_map;

pub fn status(_args: impl Iterator<Item = String>) {
    let index = load_index_as_map();

    println!("Changes to be committed:");

    for (k, _) in index {
        println!("\t{k}\n")
    }
}
