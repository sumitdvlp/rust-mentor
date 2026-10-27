//! # Module 01 — Basics
//!
//! Replace every `todo!()` with a real implementation.
//! Run the tests:        cargo test -p m01-basics
//! Run just one test:    cargo test -p m01-basics fizzbuzz
//!
//! Read `README.md` in this folder first. Rules of the game:
//!   * Make the tests pass WITHOUT editing the tests.
//!   * Then run `cargo clippy -p m01-basics` and fix what it suggests.
//!   * Then ask your mentor for a review.

// Silences "unused variable" noise while functions are still `todo!()`.
// Delete this line once you've finished the module.
#![allow(unused_variables, unused_mut, unused_imports, dead_code)]

// ---------------------------------------------------------------------------
// Exercise 1 — functions & expressions
// ---------------------------------------------------------------------------

/// Convert Celsius to Fahrenheit: F = C × 9/5 + 32.
///
/// Challenge: write it as a single expression, no `return` keyword.
pub fn celsius_to_fahrenheit(c: f64) -> f64 {
    c * f64::from(9.0 / 5.0) + 32.0
}

// ---------------------------------------------------------------------------
// Exercise 2 — `if` is an expression
// ---------------------------------------------------------------------------

/// Return "Fizz" for multiples of 3, "Buzz" for multiples of 5,
/// "FizzBuzz" for both, otherwise the number itself as a String.
///
/// Hint: `n.to_string()` and `String::from("Fizz")`.
pub fn fizzbuzz(n: u32) -> String {
    let label    = if n % 15 == 0 {
        String::from("FizzBuzz")
    } else if n % 5 == 0 {
        String::from("Buzz")
    } else if n % 3 == 0 {
        String::from("Fizz")
    }else{
        n.to_string()
    };
    label
}

// ---------------------------------------------------------------------------
// Exercise 3 — `for` loops and ranges
// ---------------------------------------------------------------------------

/// Sum of squares 1² + 2² + ... + n².  Return 0 for n = 0.
///
/// Try it twice: once with a `for` loop and a `let mut` accumulator,
/// then with an iterator: `(1..=n).map(...).sum()`.
pub fn sum_of_squares(n: u64) -> u64 {
    let mut sum = 0;
    for i in 1..n{
        let mut sum = sum + i * i;
    }
    sum
}

// ---------------------------------------------------------------------------
// Exercise 4 — mutability & tuples
// ---------------------------------------------------------------------------

/// The n-th Fibonacci number: fib(0)=0, fib(1)=1, fib(n)=fib(n-1)+fib(n-2).
///
/// Do it iteratively (no recursion). Tuple destructuring is your friend:
/// `(a, b) = (b, a + b);`
pub fn fibonacci(n: u32) -> u64 {
    let a = 0;
    let b = 1;
    for i in 1..n{
        let tmp = a+b;
        let a = b;
        let b = tmp;
    }
    a+b
}

// ---------------------------------------------------------------------------
// Exercise 5 — `while` / early return
// ---------------------------------------------------------------------------

/// Is `n` prime? (0 and 1 are not.)
///
/// Only test divisors up to √n: loop while `i * i <= n`.
pub fn is_prime(n: u64) -> bool {
    let i = 1;
    let mut sq = 1;
    while sq <= n{
        sq = i*i;
        if sq == n{
            return false;
        }
    }
    true
}

// ---------------------------------------------------------------------------
// Exercise 6 — `loop` can return a value with `break value`
// ---------------------------------------------------------------------------

/// Collatz: if n is even → n/2, if odd → 3n+1. Count steps until n == 1.
/// collatz_steps(1) == 0, collatz_steps(6) == 8.
///
/// Requirement: use `loop` and `break steps;` (a loop is an expression!).
pub fn collatz_steps(n: u64) -> u32 {
    todo!()
}

// ---------------------------------------------------------------------------
// Exercise 7 — `match` with ranges
// ---------------------------------------------------------------------------

/// Map a score to a letter grade:
/// 90..=100 → 'A', 80..=89 → 'B', 70..=79 → 'C', 60..=69 → 'D',
/// 0..=59 → 'F', anything above 100 → '?'.
///
/// Notice: the compiler forces you to handle *every* possible u8.
pub fn grade(score: u8) -> char {
    todo!()
}

// ---------------------------------------------------------------------------
// Exercise 8 — arrays & returning tuples
// ---------------------------------------------------------------------------

/// Return (min, max) of a fixed-size array.
///
/// Arrays have a size known at compile time: `[i32; 5]` is a *different
/// type* from `[i32; 6]`. (Go has the same idea; Java/Python don't.)
pub fn min_max(nums: [i32; 5]) -> (i32, i32) {
    todo!()
}

// ---------------------------------------------------------------------------
// Exercise 9 — strings & chars (sneak peek at `&str`)
// ---------------------------------------------------------------------------

/// Count vowels (a, e, i, o, u — case-insensitive).
///
/// Hint: `s.chars()` gives you an iterator of `char`.
/// `c.to_ascii_lowercase()` and `matches!(c, 'a' | 'e' | ...)` are handy.
pub fn count_vowels(s: &str) -> usize {
    todo!()
}

// ---------------------------------------------------------------------------
// Exercise 10 — integer overflow is NOT silent
// ---------------------------------------------------------------------------

/// Add two u8 values. Return `Some(sum)` if it fits in a u8, else `None`.
///
/// In debug builds `a + b` would *panic* on overflow (unlike Go/Java which
/// wrap silently). Look up `u8::checked_add` in the docs.
pub fn safe_add(a: u8, b: u8) -> Option<u8> {
    todo!()
}

// ---------------------------------------------------------------------------
// Exercise 11 — shadowing
// ---------------------------------------------------------------------------

/// Parse a string like " 21 " into a number and double it.
///
/// Requirement: use shadowing — re-declare `input` with `let` as you
/// transform it (trim → parse → double). Use `.unwrap()` for now;
/// Module 04 teaches proper error handling.
pub fn parse_and_double(input: &str) -> i64 {
    todo!()
}

// ===========================================================================
// Tests — your specification lives in `src/tests.rs`. Read it! Don't edit it.
// ===========================================================================
#[cfg(test)]
mod tests;
