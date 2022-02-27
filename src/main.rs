mod difference;
mod util;

use std::fs::read_to_string;

use difference::get_diff;

fn main() {
    let file_a = read_to_string("data/trackMe.txt").unwrap();
    let file_b = read_to_string("data/trackMeNew.txt").unwrap();

    get_diff(file_a.lines(), file_b.lines());
}
