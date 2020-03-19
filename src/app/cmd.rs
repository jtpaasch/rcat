//! This module provides utilities for shelling out commands to the OS.
//!
//! To have the OS execute a program, use the `exec` function.

use std::io::ErrorKind;
use std::process::Command;

/// Explicit errors we handle.
pub enum Error {
    NoProg(String),
    NoFile(String),
    NoPerm(String),
    Other(String),
}

/// The results of a program execution (a system call).
pub struct Execution {
    stdout: String,
}

/// Construct an `Execution` instance.
pub fn make_execution(stdout: String) -> Execution {
    Execution{
        stdout: stdout,
    }
}

/// Get the stdout data of an execution.
pub fn stdout(execution: Execution) -> String {
    execution.stdout
}

/// Have the OS execute a program with arguments.
///
/// Returns an `Execution` record if all goes okay,
/// or an explicit `Error` if something went wrong.
pub fn exec(prog: String, args: Vec<String>) -> Result<Execution, Error> {

    // We'll need a copy of this below.
    let p = prog.clone();

    // Run the process and handle the result.
    let result = Command::new(prog)
        .args(args)
        .output();
    match result {

        // In case the OS raises some errors.
        Err(err) => {
            match err.kind() {
                ErrorKind::NotFound => {
                    let msg =
                        format!("No `{}` program found on your machine", p)
                        .to_string();
                    Err(Error::NoProg(msg))
                },
                ErrorKind::PermissionDenied => {
                    let msg =
                        format!("No permission to execute `{}`", p)
                        .to_string();
                    Err(Error::NoPerm(msg))
                },
                _ => Err(Error::Other(err.to_string())),
            }
        },
        
        // The process exited safely.
        Ok(output) => {

            // Did the process succeed? (Exit code of 0.)
            match output.status.success() {

                // Unpack the stdout.
                true => {
                    let data = String::from_utf8(output.stdout).unwrap();
                    let out = make_execution(data);
                    Ok(out)
                },

                // Inspect stderr to find particular errors.
                false => {
                    let data = String::from_utf8(output.stderr).unwrap();
                    if data.contains("No such file") {
                        let out = Error::NoFile(data);
                        Err(out)
                    } else if data.contains("permission denied") {
                        let out = Error::NoPerm(data);
                        Err(out)
                    } else {
                        let out = Error::Other(data);
                        Err(out)
                    }
                }
            }
        }

    }

}
