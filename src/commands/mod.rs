pub mod add;
mod error;

use std::fs::read_to_string;

use crate::difference::get_diff;

use error::Error;

pub fn diff() -> Result<(), Error> {
    let file_a = read_to_string(".nv/trackMe.txt")?;
    let file_b = read_to_string("data/trackMe.txt")?;

    let deltas = get_diff(file_a.lines(), file_b.lines());

    for delta in deltas {
        println!("{delta}");
    }

    Ok(())
}
