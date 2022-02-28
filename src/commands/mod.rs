pub mod add;
pub mod status;

use std::fs::read_to_string;

use crate::difference::get_diff;

// TODO: This is mocked.
pub fn diff() {
    let file_a = read_to_string(".nv/trackMe.txt").expect("Could not load file_a");
    let file_b = read_to_string("data/trackMe.txt").expect("Could not load file_b");

    let deltas = get_diff(file_a.lines(), file_b.lines());

    for delta in deltas {
        println!("{delta}");
    }
}
