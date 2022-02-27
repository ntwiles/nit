mod commands;
mod difference;
mod util;

use std::env;

use commands::diff;

fn main() {
    let mut args = env::args();

    // Ignore arg 0
    args.next();

    if let Some(command) = args.next() {
        match command.as_str() {
            "diff" => diff(),
            other => eprintln!("Unknown command: {other}"),
        }
    } else {
        eprintln!("Must enter a command!")
    }
}
