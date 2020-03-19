//! This module provides utilities for exiting the program cleanly.

use std::process;

/// Exit the program with an error.
pub fn exit_with_err(msg: String) {
    eprintln!("Error: {}", msg);
    process::exit(1);
}
