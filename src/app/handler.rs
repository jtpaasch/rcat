//! This module is the main handler for the app.
//!
//! To run the app, pass a vector of filepaths to the `run` function.

use crate::app::cmd;

/// This function runs the main application.
///
/// It takes a vector of filepaths to cat, it asks the OS to `cat` them,
/// and it returns a resulting message, which is either the result
/// if everything went well, or an error message.
pub fn run(filepaths: Vec<String>) -> String {

    // Have the OS `cat` the `filepaths`. 
    let prog = "cat".to_string();
    let args = filepaths.clone();
    let result = cmd::exec(prog, args);

    // Handle the results.
    match result {
        Err(err) => {
            match err {
                cmd::Error::NoProg(msg) => msg,
                cmd::Error::NoFile(msg) => msg,
                cmd::Error::NoPerm(msg) => msg,
                cmd::Error::Other(msg) => msg,
            }
        },
        Ok(execution) => cmd::stdout(execution)
    }

}
