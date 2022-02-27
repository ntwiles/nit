use crate::difference::get_diff;

use std::fs::read_to_string;

pub fn diff() {
    let file_a = read_to_string("nv/trackMe.txt").unwrap();
    let file_b = read_to_string("data/trackMe.txt").unwrap();

    let deltas = get_diff(file_a.lines(), file_b.lines());

    for delta in deltas {
        println!("{delta}");
    }
}
