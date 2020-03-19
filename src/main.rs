//! # The Main Executable Crate
//!
//! This is the main executable crate.

use std::env;
use rcat::cli::{exit, args};
use rcat::app::handler;

/// The main enry point into the program.
fn main() {

    // Get the args provided to the command line.
    let raw_args: Vec<String> = env::args().collect();

    // Parse the args.
    let result = args::parse(raw_args);

    // If we got an error, exit. Otherwise, pass
    // the results off to the app.
    match result {
        Err(err) => {
            match err {
                args::Error::Help(msg) => exit::exit_with_err(msg),
                args::Error::NoArgs(msg) => exit::exit_with_err(msg),
                args::Error::InvalidOpts(msg) => exit::exit_with_err(msg),
            }
        }
        Ok(config) => {
            let filepaths = args::filepaths(config);
            let result = handler::run(filepaths);
            print!("{}", result)
        },
    }

}
