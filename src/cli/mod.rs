pub mod arg_parse;

use std::io;
use log::{error};

fn command_handler(input: &str) -> bool {
    let mut result = false;
    match input.trim().as_ref() {
        "quit" => {
            result = true;
        },
        _ => error!("Unknown command: {}", input)
    }
    result
}

/// Handles an input from command line.
/// Returns `true` if the main loop should be interrupted
fn main_iteration() -> bool {
    let mut input = String::new();
    let mut result = false;
    match io::stdin().read_line(&mut input) {
        Ok(_n) => {
            let input = input.trim();
            result = command_handler(input)
        }
        Err(error) => error!("{}", error),
    }
    result
}

/// Handles command line input
pub fn handler() {
    let mut do_exit:bool = false;
    while !do_exit {
        do_exit = main_iteration();
    }
}