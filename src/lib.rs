//! # The Library Crate
//!
//! This is the main library crate.

/// This module handles the CLI.
pub mod cli {
    pub mod exit;
    pub mod output;
    pub mod args;
}

/// This module handles the app.
pub mod app {
    pub mod cmd;
    pub mod handler;
}
