//! This module provides utilities for parsing CLI arguments.
//!
//! To parse a vector of arguments, use the `parse` function.

use crate::cli::output;

/// Explicit errors we handle.
#[derive(Debug, PartialEq)]
pub enum Error {
    Help(String),
    InvalidOpts(String),
    NoArgs(String)
}

/// Raw command line args will be parsed into this.
#[derive(Debug, PartialEq)]
pub struct Config {
    filepaths: Vec<String>,
}

/// Construct a new `Config` instance.
pub fn make_config(filepaths: Vec<String>) -> Config {
    Config {
        filepaths: filepaths,
    }
}

/// Gets the filepaths from config.
pub fn filepaths(config: Config) -> Vec<String> {
    config.filepaths
}

/// Returns the tail of a vector.
///
/// # Examples
///
/// ```ignore
/// let original = vec!["a".to_string(), "b".to_string(), "c".to_string()];
/// let the_tail = tail(original);
///
/// // assert_eq!(the_tail, vec!["b".to_string(), "c".to_string()];
/// ```
fn tail(vec: Vec<String>) -> Vec<String> {
    let tail: Vec<String> = vec[1..].to_vec();
    tail
}

/// Checks if a vector of arguments contain the -h/--help option.
///
/// # Examples
///
/// ```ignore
/// let args_1 = vec!["a".to_string(), "-h".to_string()];
/// let has_help_1 = contains_help(args_1);
///
/// let args_2 = vec!["a".to_string(), "--help".to_string()];
/// let has_help_2 = contains_help(args_2);
///
/// let args_3 = vec!["a".to_string(), "b".to_string()];
/// let has_help_3 = contains_help(args_3);
///
/// assert_eq!(has_help_1, true);
/// assert_eq!(has_help_2, true);
/// assert_eq!(has_help_3, false);
/// ```
fn contains_help(args: Vec<String>) -> bool {
    args.contains(&"-h".to_string()) || args.contains(&"--help".to_string())
}

/// Finds all unrecognized options in a vector of arguments.
///
/// It filters the arguments down to those that start with a dash,
/// but which are not `-h` or `--help`.
///
/// # Examples
///
/// ```ignore
/// let args_1 = vec!["a".to_string(), "-b".to_string(), "-h".to_string()];
/// let invalid_opts_1 = invalid_opts(args_1);
///
/// let args_2 = vec!["-c".to_string(), "--help".to_string()];
/// let invalid_opts_2 = invalid_opts(args_2);
///
/// let args_3 = vec!["a".to_string(), "b".to_string()];
/// let invalid_opts_3 = invalid_opts(args_3);
///
/// assert_eq!(invalid_opts_1, vec!["-b".to_string()]);
/// assert_eq!(invalid_opts_2, vec!["-c".to_string()]);
/// assert_eq!(invalid_opts_3, vec![]);
/// ```
fn invalid_options(args: Vec<String>) -> Vec<String> {
    let mut args_copy = args.clone();
    args_copy.retain(|x| x.starts_with("-") && x != "-h" && x != "--help");
    args_copy.clone()
}

/// Parses a vector of arguments.
///
/// The arguments are assumed to be a list of filepaths, or `-h` or `--help`.
/// If `-h` or `--help` are present, an `Err` with usage is returned.
/// If no arguments are present, an `Err` with a message is returned.
/// If invalid options are present, an `Err` with a message is returned.
/// Otherwise, the arguments are returned, as a vector of filepaths.
/// Note that the first argument in the original list is removed,
/// since that is the name of the invoked program.
///
/// # Examples:
///
/// ```
/// use rcat::cli::args;
/// use rcat::cli::output;
///
/// let arguments = 
///     vec!["rcat".to_string(), "/path/1".to_string(), "/path/2".to_string()];
/// let result = args::parse(arguments);
///
/// let config = 
///     args::make_config(vec!["/path/1".to_string(), "/path/2".to_string()]);
/// let expected = Ok(config);
/// assert_eq!(result, expected);
///
/// let arguments = vec!["rcat".to_string(), "-h".to_string()]; 
/// let result = args::parse(arguments);
///
/// let e = args::Error::Help(output::USAGE.to_string());
/// let expected = Err(e);
/// assert_eq!(result, expected);
///
/// let arguments = vec!["rcat".to_string(), "--help".to_string()]; 
/// let result = args::parse(arguments);
///
/// let e = args::Error::Help(output::USAGE.to_string());
/// let expected = Err(e);
/// assert_eq!(result, expected);
///
/// let arguments = vec!["rcat".to_string()];
/// let result = args::parse(arguments);
/// 
/// let e = args::Error::NoArgs(output::NO_ARGS_ERR.to_string());
/// let expected = Err(e);
/// assert_eq!(result, expected);
///
/// let arguments =
///     vec!["rcat".to_string(), "/path/1".to_string(), "-e".to_string()]; 
/// let result = args::parse(arguments);
///
/// let invalid_opts = vec!["-e".to_string()];
/// let e = args::Error::InvalidOpts(output::invalid_opts_err(invalid_opts));
/// let expected = Err(e);
/// assert_eq!(result, expected);
/// ```
pub fn parse(args: Vec<String>) -> Result<Config, Error> {

    // If there aren't any arguments, report it.
    if args.len() < 2 {
        let err = Error::NoArgs(output::NO_ARGS_ERR.to_string());
        return Err(err);
    }

    // If help is requested, return the help/usage.
    if contains_help(args.clone()) {
        let err = Error::Help(output::USAGE.to_string());
        return Err(err);
    }

    // If there are invalid options, report it.
    let invalid_opts = invalid_options(args.clone());
    if invalid_opts.len() > 0 {
        let err = Error::InvalidOpts(output::invalid_opts_err(invalid_opts));
        return Err(err);
    }

    // If we made it here, we assume the remaining arguments
    // are okay, and that they are filepaths.
    let config = make_config(tail(args));
    Ok(config)

}

