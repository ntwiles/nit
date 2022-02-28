mod commands;
mod difference;
mod hash;
mod index;
mod util;

use std::env;

use commands::{add::*, diff};

fn main() {
    let mut args = env::args();

    // Ignore arg 0
    args.next();

    if let Some(command) = args.next() {
        let result = match command.as_str() {
            "diff" => diff(),
            "add" => add(args),
            other => panic!("Unknown command: {other}"),
        };

        if let Err(error) = result {
            println!("Failed to execute command '{command}': {}", error.0);
        }
    } else {
        eprintln!("Must enter a command!")
    }
}
