//! The thin shell around the library: args in, text out, exit code.
//!
//! Try it:
//!   cargo run -p wordstat -- projects/p1-wordstat/sample.txt
//!   cargo run -p wordstat -- -w --top 3 projects/p1-wordstat/sample.txt
//!   echo "hi there" | cargo run -p wordstat -- -w
//!
//! Steps for `main`:
//!   1. Collect `std::env::args()`, skipping the program name.
//!   2. `parse_args` → on error: print "wordstat: <error>" and USAGE to
//!      stderr (`eprintln!`), return exit code 2.
//!   3. `read_input` → on error: print "wordstat: <error>" to stderr,
//!      return exit code 1.
//!   4. `print!` the result of `render` and return `ExitCode::SUCCESS`.

// Delete this line once main() is implemented:
#![allow(unused_imports)]

use std::process::ExitCode;
use wordstat::{USAGE, parse_args, read_input, render};

fn main() -> ExitCode {
    todo!()
}
