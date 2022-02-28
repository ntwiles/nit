use crate::index::load_index;

pub fn status(_args: impl Iterator<Item = String>) {
    let index = load_index();

    println!("Changes to be committed:");

    for (k, _) in index {
        println!("\t{k}\n")
    }
}
