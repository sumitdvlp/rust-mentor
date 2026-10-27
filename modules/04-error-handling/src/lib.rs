//! # Module 04 — Error Handling
//!
//! cargo test -p m04-error-handling
//!
//! Rule for this module: **no `.unwrap()` or `.expect()` in your
//! implementations** (tests may use them). Propagate errors with `?`.

#![allow(unused_variables, unused_mut, unused_imports, dead_code)]

use std::fmt;
use std::num::ParseIntError;

// ---------------------------------------------------------------------------
// Exercise 1 — Result with a simple String error
// ---------------------------------------------------------------------------

/// Parse an age. Valid: integers 0..=150 (surrounding whitespace allowed).
/// Errors (exact messages):
///   not a number  → Err("not a number: <input-trimmed>")
///   > 150         → Err("unrealistic age: <n>")
pub fn parse_age(s: &str) -> Result<u8, String> {
    todo!()
}

// ---------------------------------------------------------------------------
// Exercise 2 — an error enum
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq)]
pub enum MathError {
    DivisionByZero,
    Overflow,
}

/// Integer division that never panics.
/// i32::MIN / -1 overflows — that's the `Overflow` case.
/// Hint: `a.checked_div(b)` returns None in BOTH bad cases; distinguish them.
pub fn divide(a: i32, b: i32) -> Result<i32, MathError> {
    todo!()
}

// ---------------------------------------------------------------------------
// Exercise 3 — the `?` operator
// ---------------------------------------------------------------------------

/// Parse every string and sum them. Fail on the first bad input.
///
/// Do it twice (keep the one you like):
///   a) a `for` loop with `let n: i32 = item.parse()?;`
///   b) iterator: `items.iter().map(|s| s.parse::<i32>()).sum()`
///      — yes, `Result` implements `Sum`! Collecting into Result
///      short-circuits on the first Err.
pub fn sum_strings(items: &[&str]) -> Result<i32, ParseIntError> {
    todo!()
}

// ---------------------------------------------------------------------------
// Exercise 4 — a real custom error type: Display + Error + From
// ---------------------------------------------------------------------------

/// Errors from parsing a config line like "port=8080".
#[derive(Debug, PartialEq)]
pub enum ConfigError {
    MissingEquals,
    EmptyKey,
    BadNumber(ParseIntError),
}

/// Human-readable messages (exact text):
///   MissingEquals → "expected key=value"
///   EmptyKey      → "key must not be empty"
///   BadNumber(e)  → "invalid number: <e>"
impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

/// Marks ConfigError as a "real" error type (usable with Box<dyn Error>).
/// Nothing to implement: the default methods are fine.
impl std::error::Error for ConfigError {}

/// This is what makes `?` convert a ParseIntError into a ConfigError
/// automatically. `?` calls `From::from(err)` for you.
impl From<ParseIntError> for ConfigError {
    fn from(e: ParseIntError) -> Self {
        todo!()
    }
}

/// Parse "key=value" where value is a u16 port-like number.
/// Trim whitespace around key and value.
/// Requirement: use `?` on the parse — rely on your `From` impl.
pub fn parse_config_line(line: &str) -> Result<(String, u16), ConfigError> {
    todo!()
}

// ---------------------------------------------------------------------------
// Exercise 5 — Option combinators (no `match` allowed!)
// ---------------------------------------------------------------------------

/// Find the first even number and double it.
/// Requirement: one expression using `.iter().find(...)` and `.map(...)`.
pub fn first_even_doubled(v: &[i32]) -> Option<i32> {
    todo!()
}

/// Return the trimmed username, or "anonymous" if None or blank.
/// Hint: `.map(str::trim)`, `.filter(...)`, `.unwrap_or("anonymous")`.
pub fn display_name(name: Option<&str>) -> String {
    todo!()
}

// ---------------------------------------------------------------------------
// Exercise 6 — Box<dyn Error>: "any error" for application code
// ---------------------------------------------------------------------------

/// Parse many config lines and return the sum of all values.
/// Empty lines and lines starting with '#' are skipped.
///
/// `Box<dyn Error>` is like Go's `error` interface or Java's `Exception`:
/// any error type converts into it via `?`.
pub fn total_of_config(text: &str) -> Result<u32, Box<dyn std::error::Error>> {
    todo!()
}

// ---------------------------------------------------------------------------
// Exercise 7 — upgrading Module 03's bank account
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq)]
pub enum BankError {
    InsufficientFunds { needed: u64, available: u64 },
    AccountFrozen,
}

#[derive(Debug, Default)]
pub struct Wallet {
    pub balance: u64,
    pub frozen: bool,
}

impl Wallet {
    /// Withdraw `amount`. Frozen check comes first.
    /// On success return the *new* balance.
    pub fn withdraw(&mut self, amount: u64) -> Result<u64, BankError> {
        todo!()
    }

    /// Move `amount` from self to `other`.
    /// If EITHER wallet is frozen → `AccountFrozen`. On any error, neither
    /// wallet changes. Reuse `withdraw` with `?`.
    pub fn transfer(&mut self, other: &mut Wallet, amount: u64) -> Result<(), BankError> {
        todo!()
    }
}

// ===========================================================================
// Tests — your specification lives in `src/tests.rs`. Read it! Don't edit it.
// ===========================================================================
#[cfg(test)]
mod tests;
