//! # P1 — wordstat: a `wc`-like command-line tool
//!
//! All the *logic* lives here in the library (easy to unit-test);
//! `main.rs` is a thin shell that talks to the outside world.
//!
//! Work milestone by milestone (see README.md):
//!   cargo test -p wordstat m1_     ← only milestone-1 tests
//!   cargo test -p wordstat m2_
//!   cargo test -p wordstat m3_     ← integration tests in tests/cli.rs
//!   cargo test -p wordstat m4_

#![allow(unused_variables, unused_mut, unused_imports, dead_code)]

use std::collections::HashMap;
use std::fmt;
use std::io::{self, Read};
use std::path::PathBuf;

pub const USAGE: &str = "usage: wordstat [-l] [-w] [-c] [-b] [--top N] [FILE | -]";

// ===========================================================================
// Milestone 1 — counting
// ===========================================================================

/// Which statistic to print. Deriving `Ord` means Lines < Words < Chars < Bytes
/// (declaration order) — useful for sorting!
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Field {
    Lines,
    Words,
    Chars,
    Bytes,
}

impl Field {
    pub const ALL: [Field; 4] = [Field::Lines, Field::Words, Field::Chars, Field::Bytes];

    /// "lines", "words", "chars", "bytes"
    pub fn label(&self) -> &'static str {
        todo!()
    }
}

#[derive(Debug, Default, PartialEq)]
pub struct Stats {
    pub lines: usize,
    pub words: usize,
    pub chars: usize,
    pub bytes: usize,
}

impl Stats {
    /// lines: `str::lines` count · words: whitespace-separated
    /// chars: Unicode scalar values · bytes: UTF-8 length
    pub fn count(text: &str) -> Stats {
        todo!()
    }

    pub fn get(&self, field: Field) -> usize {
        todo!()
    }
}

// ===========================================================================
// Milestone 2 — argument parsing
// ===========================================================================

#[derive(Debug, PartialEq)]
pub enum Input {
    Stdin,
    File(PathBuf),
}

#[derive(Debug, PartialEq)]
pub struct Config {
    /// Sorted, de-duplicated. Never empty (defaults to all four).
    pub fields: Vec<Field>,
    pub top: Option<usize>,
    pub input: Input,
}

#[derive(Debug, PartialEq)]
pub enum CliError {
    UnknownFlag(String),
    MissingTopValue,
    InvalidTopValue(String),
    TooManyInputs,
}

/// Messages (exact):
///   UnknownFlag(f)      → "unknown flag: <f>"
///   MissingTopValue     → "--top requires a number"
///   InvalidTopValue(v)  → "invalid --top value: <v>"
///   TooManyInputs       → "only one input is supported"
impl fmt::Display for CliError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

impl std::error::Error for CliError {}

/// Parse arguments (WITHOUT the program name).
///
/// ```text
/// -l -w -c -b    select fields (any order, repeats allowed)
/// --top N        also print the N most frequent words
/// -              read stdin (also the default when no path is given)
/// other "-x"     → UnknownFlag
/// anything else  → a file path (at most one input!)
/// ```
///
/// Hint: `let mut iter = args.iter();` + `while let Some(arg) = iter.next()`
/// lets you grab the value after `--top` with another `iter.next()`.
/// Hint: `match arg.as_str() { "-l" => ..., s if s.starts_with('-') => ... }`
pub fn parse_args(args: &[String]) -> Result<Config, CliError> {
    todo!()
}

// ===========================================================================
// Milestone 3 — I/O and output
// ===========================================================================

/// Read all text from the chosen input.
/// Hint: `std::fs::read_to_string`, and `io::stdin().read_to_string(&mut buf)`.
pub fn read_input(input: &Input) -> io::Result<String> {
    todo!()
}

/// Build the full output. One line per field, then (if `top` is set):
///
/// ```text
/// lines: 4
/// words: 23
/// top 2 words:
///   rust: 2
///   is: 3
/// ```
/// Format: "<label>: <value>\n", then "top <N> words:\n", then
/// "  <word>: <count>\n" per word. (Values above are just an example.)
pub fn render(config: &Config, text: &str) -> String {
    todo!()
}

// ===========================================================================
// Milestone 4 — top words (do this after Module 05!)
// ===========================================================================

/// Same rules as Module 05's `top_n_words`: lowercase, trim ASCII
/// punctuation from both ends, skip empty, sort count DESC then word ASC.
pub fn top_words(text: &str, n: usize) -> Vec<(String, usize)> {
    todo!()
}

// ===========================================================================
// Unit tests
// ===========================================================================
#[cfg(test)]
mod tests {
    use super::*;

    fn args(list: &[&str]) -> Vec<String> {
        list.iter().map(|s| s.to_string()).collect()
    }

    // ---- Milestone 1 ----
    #[test]
    fn m1_labels() {
        let labels: Vec<&str> = Field::ALL.iter().map(|f| f.label()).collect();
        assert_eq!(labels, ["lines", "words", "chars", "bytes"]);
    }

    #[test]
    fn m1_count_ascii() {
        let s = Stats::count("hello world\nbye\n");
        assert_eq!(
            s,
            Stats {
                lines: 2,
                words: 3,
                chars: 16,
                bytes: 16
            }
        );
        assert_eq!(s.get(Field::Words), 3);
    }

    #[test]
    fn m1_count_unicode() {
        let s = Stats::count("héllo 🦀");
        assert_eq!(s.chars, 7);
        assert_eq!(s.bytes, 11); // é = 2 bytes, 🦀 = 4 bytes
    }

    #[test]
    fn m1_count_empty() {
        assert_eq!(Stats::count(""), Stats::default());
    }

    // ---- Milestone 2 ----
    #[test]
    fn m2_defaults() {
        let c = parse_args(&[]).unwrap();
        assert_eq!(c.fields, Field::ALL.to_vec());
        assert_eq!(c.top, None);
        assert_eq!(c.input, Input::Stdin);
    }

    #[test]
    fn m2_fields_sorted_and_deduped() {
        let c = parse_args(&args(&["-c", "-l", "-c", "notes.txt"])).unwrap();
        assert_eq!(c.fields, vec![Field::Lines, Field::Chars]);
        assert_eq!(c.input, Input::File(PathBuf::from("notes.txt")));
    }

    #[test]
    fn m2_top() {
        let c = parse_args(&args(&["--top", "5", "-"])).unwrap();
        assert_eq!(c.top, Some(5));
        assert_eq!(c.input, Input::Stdin);
    }

    #[test]
    fn m2_errors() {
        assert_eq!(
            parse_args(&args(&["-x"])),
            Err(CliError::UnknownFlag("-x".into()))
        );
        assert_eq!(
            parse_args(&args(&["--top"])),
            Err(CliError::MissingTopValue)
        );
        assert_eq!(
            parse_args(&args(&["--top", "many"])),
            Err(CliError::InvalidTopValue("many".into()))
        );
        assert_eq!(
            parse_args(&args(&["a.txt", "b.txt"])),
            Err(CliError::TooManyInputs)
        );
    }

    #[test]
    fn m2_error_messages() {
        assert_eq!(
            CliError::UnknownFlag("-q".into()).to_string(),
            "unknown flag: -q"
        );
        assert_eq!(
            CliError::MissingTopValue.to_string(),
            "--top requires a number"
        );
        assert_eq!(
            CliError::InvalidTopValue("x".into()).to_string(),
            "invalid --top value: x"
        );
        assert_eq!(
            CliError::TooManyInputs.to_string(),
            "only one input is supported"
        );
    }

    // ---- Milestone 3 ----
    #[test]
    fn m3_render_selected_fields() {
        let c = parse_args(&args(&["-w", "-l"])).unwrap();
        assert_eq!(render(&c, "a b c\nd\n"), "lines: 2\nwords: 4\n");
    }

    // ---- Milestone 4 ----
    #[test]
    fn m4_top_words() {
        let text = "The cat; the HAT. A cat!";
        assert_eq!(
            top_words(text, 2),
            vec![("cat".to_string(), 2), ("the".to_string(), 2)]
        );
    }

    #[test]
    fn m4_render_with_top() {
        let c = parse_args(&args(&["-w", "--top", "1"])).unwrap();
        assert_eq!(
            render(&c, "go rust rust"),
            "words: 3\ntop 1 words:\n  rust: 2\n"
        );
    }
}
