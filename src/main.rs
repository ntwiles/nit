mod commands;
mod difference;
mod hash;
mod index;
mod util;

use std::env;

use commands::{add::*, diff, status::*};

fn main() {
    let mut args = env::args();

    // Ignore arg 0
    args.next();

    if let Some(command) = args.next() {
        match command.as_str() {
            "add" => add(args),
            "diff" => diff(),
            "status" => status(args),
            other => panic!("Unknown command: {other}"),
        };
    } else {
        eprintln!("Must enter a command.")
    }
}
