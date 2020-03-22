//! Pre-canned messages for output.

/// Generates a message to use if there are invalid options.
///
/// # Examples
///
/// ```
/// use rcat::cli::output;
///
/// let opts = vec!["-e".to_string(), "-j".to_string()];
/// let err = output::invalid_opts_err(opts);
///
/// assert_eq!(err, "Unrecognized option(s): -e, -j\nSee rcat --help");
/// ```
pub fn invalid_opts_err(opts: Vec<String>) -> String {
    let start = "Unrecognized option(s): ".to_string();
    let middle = opts.join(", ");
    let end = "\nSee rcat --help".to_string();
    [start, middle, end].concat()
}

/// A message for if there aren't enough arguments.
pub const NO_ARGS_ERR : &'static str = "Not enough arguments!";

/// A message for help/usage.
pub const USAGE : &'static str =
r#"USAGE: rcat [OPTIONS] [ARGEMNTS]

  A simple cat program.

EXAMPLES:
  rcat --help
  rcat /path/to/file1 /path/to/file2 ...

OPTIONS:
  -h, --help      Display this help.

ARGUMENTS:
  /path/to/file1  A path to a file.
  /path/to/file2  A path to another file.
  ...             Ditto.

"#;
