# `rcat`

A simple terminal program, written in Rust. For illustration purposes.
This is just a simple `cat` program.


## Usage

See the help:

    rcat --help

Cat a file:

    rcat /path/to/file

Cat multiple files:

    rcat /path/to/file1 /path/to/file2 ...


## Build

Download the project, e.g.:

    git clone https://..../rcat.git
    cd rcat

Build it:

    cargo build --release

The executable is at `target/release/rcat`.
Install it somewhere if you like.

To clean:

    cargo clean


## Development

Download the project, e.g.:

    git clone https://..../rcat.git
    cd rcat

To have `cargo` build and run it, this is the format:

    cargo run /path/to/file1 /path/to/file2 ...

To see the help:

    cargo run --help

To clean:

    cargo clean


