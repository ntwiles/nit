mod commands;
mod commits;
mod difference;
mod hash;
mod index;
mod objects;
mod util;

use std::env;

use commands::{add::*, checkout::*, commit::*, diff, init::*, status::*};

fn main() {
    let mut args = env::args();

    // Ignore arg 0
    args.next();

    if let Some(command) = args.next() {
        match command.as_str() {
            "add" => add(args),
            "checkout" => checkout(args),
            "commit" => commit(args),
            "diff" => diff(),
            "init" => init(args),
            "status" => status(args),
            other => panic!("Unknown command: {other}"),
        };
    } else {
        eprintln!("Must enter a command.")
    }
}
